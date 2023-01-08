use self::solver_history::{SolverHistory, SolverHistoryType, SolverResult};
use self::solver_simple::SolverSimple;
use crate::model::array_vector::ArrayVector;
use crate::model::max_num::MaxNum;
use crate::model::table_lock::{TableLock, TableLockReadGuard};
use crate::model::{cell::Cell, zone::Zone};
use enum_iterator::{all, cardinality};
use hashbrown::{HashMap, HashSet};
use rand::{prelude::StdRng, RngCore, SeedableRng};
use std::sync::atomic::{AtomicBool, Ordering};
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
    t: &'a TableLock<N>,
    solver_history_stack: Vec<SolverHistory<'a, N>>,
    rng: StdRng,
    rand_seed: u64,
    solve_cnt: HashMap<SolverSimple, u32>,
    guess_cnt: u32,
    guess_rollback_cnt: u32,
    guess_backtrace_rollback_cnt: u32,
    // Zone과 Zone에 속한 Cell 목록을 Vec로 정렬
    ordered_zone: Vec<(&'a Zone, Vec<&'a Cell<N>>)>,
    hashed_zone: HashMap<&'a Zone, Vec<&'a Cell<N>>>,
    connect_zone: HashMap<&'a Zone, HashSet<&'a Zone>>,
    checked_zone: RwLock<HashMap<&'a Zone, HashMap<SolverSimple, bool>>>,
}

impl<'a, const N: usize> Solver<'a, N> {
    /// 제한시간 내에 스도쿠를 모두 채우려고 시도합니다.
    ///
    /// 풀리지 않고 남은 cell의 개수를 반환합니다.
    /// 제한시간을 초과했거나 풀 수 없는 문제인 경우 1이상일 수 있습니다.
    pub fn fill_puzzle_with_timeout(&mut self, timeout: Duration) -> usize {
        let start = Instant::now();

        loop {
            let unsolved_cell_cnt = self.get_unsolved_cell_cnt();
            if unsolved_cell_cnt == 0 {
                break;
            }

            // println!("{}", self.t);
            // println!("-----------------------------");
            // timeout 또는 모든 문제를 풀 수 없는 경우 return
            if (Instant::now() - start) >= timeout || !self.solve() {
                return unsolved_cell_cnt;
            }
        }

        0
    }

    pub fn solve(&mut self) -> bool {
        debug_assert!(self.t.table_num_chk_validater());
        let read = self.t.read_lock();

        let result_list: Mutex<Vec<SolverResult<N>>> = Mutex::new(Vec::new());
        let mut error_cell: Option<&Cell<N>> = None;
        let is_break = AtomicBool::new(false);

        rayon::scope_fifo(|s| {
            s.spawn_fifo(|_| {
                // print!("VAL ");
                // 먼저 오류가 있는지 체크하여 있을 경우 롤백
                error_cell = self.find_error_cell(&read, &is_break);
                if error_cell.is_some() {
                    is_break.store(true, Ordering::Relaxed);
                }
            });

            s.spawn_fifo(|_| {
                // Single Solver 적용
                // print!("SINGLE ");
                let mut result = self.single(&read, &is_break);
                if !result.is_empty() {
                    let mut lock = result_list.lock().unwrap();
                    lock.append(&mut result);
                }
            });

            s.spawn_fifo(|s2| {
                // Naked Solver 적용
                // print!("NAKED ");
                self.naked(&read, s2, &result_list, &is_break);
            });

            s.spawn_fifo(|_| {
                // print!("BLR ");
                // Box Line Reduction Solver 적용
                let mut result = self.box_line_reduction(&read, &is_break);
                if !result.is_empty() {
                    let mut lock = result_list.lock().unwrap();
                    lock.append(&mut result);
                }
            });

            false
        });

        if error_cell.is_some() {
            // println!("ROLLBACK");
            return self.history_rollback_last_guess(read);
        }

        let result_list = result_list.into_inner().unwrap();
        if result_list.is_empty() {
            // 푸는게 실패할 경우 guess를 적용
            self.guess_random(read);
            self.guess_cnt += 1;
        } else {
            self.solve_result_commit(read, result_list);
        }

        true
    }

    /// 스도푸를 푼 경우 해당 결과를 적용합니다.
    fn solve_result_commit(
        &mut self,
        read: TableLockReadGuard<N>,
        result: Vec<SolverResult<'a, N>>,
    ) {
        let mut write = read.upgrade_to_write();

        for solver_result in result {
            let history = {
                let mut backup_chk: Vec<(&'a Cell<N>, ArrayVector<MaxNum<N>, N>)> =
                    Vec::with_capacity(solver_result.effect_cells.len());

                for (c, _) in &solver_result.effect_cells {
                    let backup = write.read_from_cell(c).clone_chk_list();
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
                let write_cell = write.write_from_cell(c);
                write_cell.set_to_false_list(v);
                self.checked_zone_clear(c);
            }

            *self
                .solve_cnt
                .get_mut(&SolverSimple::convert_detail_to_simple(
                    &solver_result.solver_type,
                ))
                .unwrap() += 1;

            self.solver_history_stack.push(history);
        }
    }

    /// 가장 최근의 guess까지 롤백
    fn history_rollback_last_guess(&mut self, read: TableLockReadGuard<N>) -> bool {
        let no_guess = self
            .solver_history_stack
            .iter()
            .all(|history| !matches!(history.history_type, SolverHistoryType::Guess { .. }));

        // 만약 히스토리에 guess가 없는 경우 롤백할 의미가 없으므로 return
        if no_guess {
            return false;
        }

        let mut write = read.upgrade_to_write();
        self.guess_rollback_cnt += 1;
        while let Some(history) = self.solver_history_stack.pop() {
            for (c, backup) in history.backup_chk {
                write.write_from_cell(c).set_to_chk_list(&backup);
                self.checked_zone_clear(c);
            }

            if let SolverHistoryType::GuessBacktrace { .. } = history.history_type {
                self.guess_backtrace_rollback_cnt += 1;
            }

            // 추측된 숫자를 실패로 간주하여 제외시킴
            // Guess를 만난 경우 롤백 중단
            if let SolverHistoryType::Guess { cell, final_num } = history.history_type {
                let mut_chk = write.write_from_cell(cell);

                let backup_chk_list = mut_chk.clone_chk_list();
                let backup = vec![(cell, backup_chk_list)];
                mut_chk.set_false(final_num);

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

    /// 이 스도쿠 퍼즐의 미완성 Cell 개수 반환
    #[must_use]
    pub fn get_unsolved_cell_cnt(&self) -> usize {
        let read = self.t.read_lock();
        self.t
            .into_iter()
            .filter(|c| !read.read_from_cell(c).is_final_num())
            .count()
    }

    // TableLock을 mut로 받을 필요는 없으나, 동일한 Table에 대해 여러 Solver를 생성하는 것을 방지하기 위해 일부러 mut로 받음
    #[must_use]
    pub fn new(t: &'a mut TableLock<N>) -> Self {
        let rand_seed: u64 = StdRng::from_entropy().next_u64();
        let mut solve_cnt: HashMap<SolverSimple, u32> = HashMap::new();
        for n in all::<SolverSimple>() {
            solve_cnt.insert(n, 0u32);
        }

        let hashed_zone = Solver::get_zone_hashmap(t);
        let zone_cnt = hashed_zone.len();

        let mut ordered_zone = hashed_zone
            .iter()
            .map(|(z, v)| (*z, v.clone()))
            .collect::<Vec<_>>();
        ordered_zone.sort_unstable_by_key(|(z, _)| *z);

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
            solver_history_stack: Vec::with_capacity(N * N * N),
            rng: rand::prelude::StdRng::seed_from_u64(rand_seed),
            rand_seed,
            guess_cnt: 0,
            guess_rollback_cnt: 0,
            guess_backtrace_rollback_cnt: 0,
            solve_cnt,
            ordered_zone,
            hashed_zone,
            connect_zone,
            checked_zone: RwLock::new(HashMap::with_capacity(zone_cnt)),
        }
    }

    #[must_use]
    fn get_zone_hashmap(t: &'a TableLock<N>) -> HashMap<&'a Zone, Vec<&Cell<N>>> {
        let mut hash: HashMap<&'a Zone, Vec<&Cell<N>>> = HashMap::with_capacity(N * N);
        for cell in t {
            for z in &cell.zone_vec {
                let row = hash.entry(z).or_insert_with(|| Vec::with_capacity(N));
                row.push(cell);
            }
        }

        hash
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
            .or_insert_with(|| HashMap::with_capacity(cardinality::<SolverSimple>()))
            .insert(solver, true);
    }

    /// 특정 zone에 대한 checked를 모두 초기화
    fn checked_zone_clear(&self, c: &Cell<N>) {
        let mut write = self.checked_zone.write().unwrap();

        for z in &c.zone_vec {
            let Some(simple_hashmap) = write.get_mut(z) else {
                continue;
            };

            simple_hashmap
                .iter_mut()
                .for_each(|(_, value)| *value = false);
        }
    }

    pub fn get_table(&self) -> &TableLock<N> {
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
    use crate::model::table::Table;
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
