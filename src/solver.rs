use self::solver_history::{SolverHistory, SolverHistoryType, SolverResult};
use self::solver_simple::SolverSimple;
use crate::model::array_vector::ArrayVector;
use crate::model::table::Table;
use crate::model::{cell::Cell, cell_with_read::CellWithRead, ref_zone::RefZone, zone::Zone};
use enum_iterator::all;
use hashbrown::{HashMap, HashSet};
use rand::{prelude::StdRng, RngCore, SeedableRng};
use scoped_threadpool::Pool;
use std::sync::{Mutex, RwLock};
use std::time::{Duration, Instant};

pub mod box_line_reduction;
pub mod guess;
pub mod naked;
pub mod single;
pub mod solver_history;
pub mod solver_simple;
pub mod validater;

pub struct Solver<'a, const N: usize> {
    t: &'a Table<N>,
    solver_history_stack: Vec<SolverHistory<'a, N>>,
    rng: StdRng,
    rand_seed: u64,
    solve_cnt: HashMap<SolverSimple, u32>,
    guess_cnt: u32,
    guess_rollback_cnt: u32,
    guess_backtrace_rollback_cnt: u32,
    changed_cell: HashSet<&'a Cell<N>>,
    ordered_zone: Vec<(&'a Zone, Vec<&'a Cell<N>>)>,
    connect_zone: HashMap<&'a Zone, HashSet<&'a Zone>>,
    checked_zone: RwLock<HashMap<&'a Zone, HashMap<SolverSimple, bool>>>,
    pool: Mutex<Pool>,
}

impl<'a, const N: usize> Solver<'a, N> {
    /// 제한시간 내에 스도쿠를 모두 채우려고 시도합니다.
    ///
    /// 풀리지 않고 남은 cell의 개수를 반환합니다.
    /// 제한시간을 초과했거나 풀 수 없는 문제인 경우 1이상일 수 있습니다.
    pub fn fill_puzzle_with_timeout(&mut self, timeout: Duration) -> usize {
        let start = Instant::now();

        while !self.is_complete_puzzle() {
            // println!("{}", self.t);
            // println!("-----------------------------");
            // timeout 또는 모든 문제를 풀 수 없는 경우 return
            if (Instant::now() - start) >= timeout || self.solve().is_none() {
                return self
                    .t
                    .into_iter()
                    .filter(|n| !n.chk.read().unwrap().is_final_num())
                    .count();
            }
        }

        0
    }

    pub fn solve(&mut self) -> Option<&SolverHistory<'a, N>> {
        let ref_zone = Solver::<N>::get_zone_ref_with_read(&self.ordered_zone);
        debug_assert!(self.t.num_check_validater());

        // 먼저 오류가 있는지 체크하여 있을 경우 롤백
        if self.find_error_cell(&ref_zone).is_some() {
            drop(ref_zone);
            // println!("ROLLBACK");
            if self.history_rollback_last_guess() {
                return self.solver_history_stack.last();
            } else {
                return None;
            }
        }

        // Single Solver 적용
        let mut result = self.single(&ref_zone);
        if !result.is_empty() {
            // println!("SINGLE");
            drop(ref_zone);
            self.solve_result_commit(result);
            return self.solver_history_stack.last();
        }

        // Naked Solver 적용
        result = self.naked(&ref_zone);
        if !result.is_empty() {
            // println!("NAKED");
            drop(ref_zone);
            self.solve_result_commit(result);
            return self.solver_history_stack.last();
        }

        // Box Line Reduction Solver 적용
        let ref_zone_hash = ref_zone.iter().collect::<HashSet<_>>();
        result = self.box_line_reduction(&ref_zone, &ref_zone_hash);
        if !result.is_empty() {
            drop(ref_zone_hash);
            drop(ref_zone);
            self.solve_result_commit(result);
            return self.solver_history_stack.last();
        }

        // 푸는게 실패할 경우 guess를 적용
        // println!("GUESS");
        self.changed_cell.clear();
        drop(ref_zone_hash);
        self.guess_random(ref_zone);
        self.guess_cnt += 1;
        self.solver_history_stack.last()
    }

    /// 스도푸를 푼 경우 해당 결과를 적용합니다.
    fn solve_result_commit(&mut self, mut result: Vec<SolverResult<'a, N>>) -> bool {
        let mut commit_flag = false;
        while let Some(solver_result) = result.pop() {
            let history = {
                let mut backup_chk: Vec<(&'a Cell<N>, ArrayVector<usize, N>)> =
                    Vec::with_capacity(solver_result.effect_cells.len());

                for (c, _) in &solver_result.effect_cells {
                    let backup = c.chk.read().unwrap().clone_chk_list_vec();
                    backup_chk.push((c, backup));
                }

                SolverHistory {
                    history_type: SolverHistoryType::Solve { solver_result },
                    backup_chk,
                }
            };

            let SolverHistoryType::Solve { ref solver_result } = history.history_type else {
                unreachable!();
            };

            for (c, v) in &solver_result.effect_cells {
                let mut write = c.chk.write().unwrap();
                write.set_to_false_list(v);
                self.changed_cell.insert(c);
                self.checked_zone_clear(c);
            }

            *self
                .solve_cnt
                .get_mut(&SolverSimple::convert_detail_to_simple(
                    &solver_result.solver_type,
                ))
                .unwrap() += 1;

            self.solver_history_stack.push(history);

            commit_flag = true;
        }

        commit_flag
    }

    /// 가장 최근의 guess까지 롤백
    fn history_rollback_last_guess(&mut self) -> bool {
        let mut no_guess: bool = true;
        for history in &self.solver_history_stack {
            if let SolverHistoryType::Guess { .. } = history.history_type {
                no_guess = false;
                break;
            }
        }

        // 만약 히스토리에 guess가 없는 경우 롤백할 의미가 없으므로 return
        if no_guess {
            return false;
        }

        self.guess_rollback_cnt += 1;
        while let Some(history) = self.solver_history_stack.pop() {
            for (c, backup) in history.backup_chk {
                c.chk.write().unwrap().set_to_chk_list(&backup);
                self.changed_cell.insert(c);
                self.checked_zone_clear(c);
            }

            if let SolverHistoryType::GuessBacktrace { .. } = history.history_type {
                self.guess_backtrace_rollback_cnt += 1;
            }

            // 추측된 숫자를 실패로 간주하여 제외시킴
            // Guess를 만난 경우 롤백 중단
            if let SolverHistoryType::Guess { cell, final_num } = history.history_type {
                let mut mut_chk = cell.chk.write().unwrap();

                let backup_chk_list = mut_chk.clone_chk_list_vec();
                let backup = vec![(cell, backup_chk_list)];
                mut_chk.set_chk(final_num, false);

                self.solver_history_stack.push(SolverHistory {
                    history_type: SolverHistoryType::GuessBacktrace {
                        cell,
                        except_num: final_num,
                    },
                    backup_chk: backup,
                });
                break;
            }
        }

        true
    }

    /// 이 스도쿠 퍼즐이 완성되었는지 여부를 반환
    #[must_use]
    pub fn is_complete_puzzle(&self) -> bool {
        self.t
            .into_iter()
            .all(|c| c.chk.read().unwrap().is_final_num())
    }

    #[must_use]
    pub fn new(t: &'a mut Table<N>) -> Self {
        let rand_seed: u64 = StdRng::from_entropy().next_u64();
        let mut solve_cnt: HashMap<SolverSimple, u32> = HashMap::new();
        for n in all::<SolverSimple>() {
            solve_cnt.insert(n, 0u32);
        }

        let mut changed_cell = HashSet::with_capacity(N * N);
        for c in t.into_iter() {
            changed_cell.insert(c);
        }

        let ordered_zone = Solver::get_ordered_zone(t);
        for (_, c) in &ordered_zone {
            if c.len() != N {
                panic!(
                    "Unique 타입의 개수는 퍼즐 사이즈와 동일해야 함! 사이즈:{}, 실제 갯수: {}",
                    N,
                    c.len()
                )
            }
        }

        let connect_zone = Solver::<N>::get_connected_zone(&ordered_zone);

        Solver {
            t,
            solver_history_stack: Vec::new(),
            rng: rand::prelude::StdRng::seed_from_u64(rand_seed),
            rand_seed,
            guess_cnt: 0,
            guess_rollback_cnt: 0,
            guess_backtrace_rollback_cnt: 0,
            solve_cnt,
            changed_cell,
            ordered_zone,
            connect_zone,
            checked_zone: RwLock::new(HashMap::new()),
            pool: Mutex::new(Pool::new(4)),
        }
    }

    #[must_use]
    fn get_ordered_zone(t: &'a Table<N>) -> Vec<(&'a Zone, Vec<&'a Cell<N>>)> {
        let mut zone_ref: HashMap<&'a Zone, Vec<&Cell<N>>> = HashMap::with_capacity(N * N);
        for cell in t {
            for z in &cell.zone_vec {
                let row = zone_ref.entry(z).or_insert_with(|| Vec::with_capacity(N));
                row.push(cell);
            }
        }

        let mut ret: Vec<(&'a Zone, Vec<&'a Cell<N>>)> = zone_ref.into_iter().collect();
        ret.sort_unstable_by_key(|(z, _)| *z);
        ret
    }

    #[must_use]
    fn get_zone_ref_with_read(
        ordered_zone: &Vec<(&'a Zone, Vec<&'a Cell<N>>)>,
    ) -> Vec<RefZone<'a, N>> {
        let mut ret: Vec<RefZone<N>> = Vec::with_capacity(ordered_zone.len());

        for (z, cell_list) in ordered_zone {
            let mut cell_with_read = Vec::with_capacity(cell_list.len());
            for cell in cell_list {
                let read = cell.chk.read().unwrap();
                cell_with_read.push(CellWithRead { cell, read });
            }
            ret.push(RefZone {
                zone: z,
                cells: cell_with_read,
            });
        }

        ret
    }

    #[must_use]
    fn get_connected_zone(
        ordered_zone: &Vec<(&'a Zone, Vec<&'a Cell<N>>)>,
    ) -> HashMap<&'a Zone, HashSet<&'a Zone>> {
        let mut ret: HashMap<&'a Zone, HashSet<&'a Zone>> =
            HashMap::with_capacity(ordered_zone.len());

        for (z1, _) in ordered_zone {
            for (z2, c2_list) in ordered_zone {
                let connected = c2_list.iter().any(|c2| c2.zone_set.contains(*z1));

                if connected {
                    ret.entry(z1).or_insert_with(HashSet::new).insert(z2);
                }
            }
        }

        ret
    }

    /// 특정 zone에 대한 checked 여부를 반환
    #[must_use]
    fn checked_zone_get_bool(&self, z: &Zone, solver: SolverSimple) -> bool {
        let read = self.checked_zone.read().unwrap();

        let Some(map) = read.get(z) else {
            return false;
        };

        let Some(bool) = map.get(&solver) else {
            return false;
        };

        *bool
    }

    /// 특정 zone에 대한 checked 여부를 true로 설정
    fn checked_zone_set_bool_true(&self, z: &'a Zone, solver: SolverSimple) {
        let mut write = self.checked_zone.write().unwrap();

        write
            .entry(z)
            .or_insert_with(HashMap::new)
            .insert(solver, true);
    }

    /// 특정 zone에 대한 checked를 모두 초기화
    fn checked_zone_clear(&self, c: &Cell<N>) {
        let mut write = self.checked_zone.write().unwrap();

        for z in &c.zone_vec {
            write.remove(z);
        }
    }

    pub fn get_table(&self) -> &Table<N> {
        self.t
    }

    pub fn get_random_seed(&self) -> u64 {
        self.rand_seed
    }

    pub fn set_random_seed(&mut self, rand_seed: u64) {
        self.rng = rand::prelude::StdRng::seed_from_u64(rand_seed);
        self.rand_seed = rand_seed;
    }

    /// Get the solver's solve cnt.
    #[must_use]
    #[inline]
    pub fn solve_cnt(&self, result_simple: &SolverSimple) -> u32 {
        self.solve_cnt[result_simple]
    }

    /// Get the solver's guess cnt.
    #[must_use]
    #[inline]
    pub fn guess_cnt(&self) -> u32 {
        self.guess_cnt
    }

    /// Get the solver's guess rollback cnt.
    #[must_use]
    #[inline]
    pub fn guess_backtrace_rollback_cnt(&self) -> u32 {
        self.guess_backtrace_rollback_cnt
    }

    /// Get the solver's guess rollback cnt.
    #[must_use]
    #[inline]
    pub fn guess_rollback_cnt(&self) -> u32 {
        self.guess_rollback_cnt
    }
}

/// 동일한 시드에 대해서 같은 퍼즐을 생성하는지 테스트
#[test]
#[cfg_attr(miri, ignore)] // 이 테스트는 miri test가 너무 오래걸려서 miri에서는 제외..
fn same_seed_puzzle_test() {
    let mut t1 = Table::new_default_9();
    let mut solver1 = Solver::new(&mut t1);
    solver1.fill_puzzle_with_timeout(std::time::Duration::MAX);

    let mut t2 = Table::new_default_9();
    let mut solver2 = Solver::new(&mut t2);
    solver2.set_random_seed(solver1.get_random_seed());
    solver2.fill_puzzle_with_timeout(std::time::Duration::MAX);
    drop(solver1);
    drop(solver2);

    assert_eq!(t1, t2);

    let mut t1 = Table::new_default_9();
    let mut solver1 = Solver::new(&mut t1);
    solver1.fill_puzzle_with_timeout(std::time::Duration::MAX);

    let mut t2 = Table::new_default_9();
    let mut solver2 = Solver::new(&mut t2);
    solver2.set_random_seed(solver1.get_random_seed() + 1);
    solver2.fill_puzzle_with_timeout(std::time::Duration::MAX);
    drop(solver1);
    drop(solver2);

    assert_ne!(t1, t2);
}
