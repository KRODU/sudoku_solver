pub mod box_line_reduction;
pub mod guess;
pub mod naked;
pub mod single;
pub mod solver_history;
pub mod solver_simple;
pub mod validater;

use self::solver_history::{SolverHistory, SolverHistoryType, SolverResult};
use self::solver_simple::SolverSimple;
use crate::model::array_vector::ArrayVector;
use crate::model::index_key_map::{IndexKeyMap, IndexKeySet};
use crate::model::max_num::MaxNum;
use crate::model::non_atomic_bool::NonAtomicBool;
use crate::model::table_lock::{TableLock, TableLockReadGuard};
use crate::model::zone_cache::ZoneCache;
use crate::model::{cell::Cell, zone::Zone};
use crate::punch::Punch;
use enum_iterator::all;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use std::fmt::Debug;
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct Solver<'a, const N: usize> {
    table: &'a TableLock<N>,
    solver_history_stack: Vec<SolverHistory<'a, N>>,
    rng: SmallRng,
    rand_seed: u64,
    solve_cnt: IndexKeyMap<SolverSimple, u32>,
    guess_cnt: u32,
    guess_rollback_cnt: u32,
    guess_backtrace_rollback_cnt: u32,
    zone_cache: ZoneCache<'a, N>,
}

pub trait GeneralSolve<'a> {
    /// 제한시간 내에 스도쿠를 모두 채우려고 시도합니다.
    ///
    /// 풀리지 않고 남은 cell의 개수를 반환합니다.
    /// 제한시간을 초과했거나 풀 수 없는 문제인 경우 1이상일 수 있습니다.
    fn fill_puzzle_with_timeout(&mut self, timeout: Duration) -> usize;

    /// solver를 적용하여 문제를 풉니다. solver가 풀지 못한 경우 guess합니다.
    fn fill_once(&mut self) -> bool;

    /// solver를 적용하여 문제를 풉니다. 문제는 푼 경우 true, 풀지 못한 경우 false를 반환합니다.
    fn solve(&mut self) -> bool;

    /// 이 스도쿠 퍼즐의 미완성 Cell 개수 반환
    #[must_use]
    fn get_unsolved_cell_cnt(&self) -> usize;

    #[must_use]
    fn get_random_seed(&self) -> u64;

    fn set_random_seed(&mut self, rand_seed: u64);

    /// Get the solver's solve cnt.
    #[must_use]
    fn solve_cnt(&self, result_simple: SolverSimple) -> u32;

    /// Get the solver's guess cnt.
    #[must_use]
    fn guess_cnt(&self) -> u32;

    /// Get the solver's guess rollback cnt.
    #[must_use]
    fn guess_backtrace_rollback_cnt(&self) -> u32;

    /// Get the solver's guess rollback cnt.
    #[must_use]
    fn guess_rollback_cnt(&self) -> u32;
}

impl<'a, const N: usize> Solver<'a, N> {
    /// 스도푸를 푼 경우 해당 결과를 적용합니다.
    fn solve_result_commit(
        &mut self,
        read: TableLockReadGuard<N>,
        result: Vec<SolverResult<'a, N>>,
    ) {
        let mut write = read.upgrade_to_write();
        let mut changed_zone_set: IndexKeySet<Zone> = IndexKeySet::new();

        for mut solver_result in result {
            let mut backup_chk: Vec<(&'a Cell<N>, ArrayVector<MaxNum<N>, N>)> =
                Vec::with_capacity(solver_result.effect_cells.len());

            for (c, effect_note) in &mut solver_result.effect_cells {
                let cell = write.write_from_cell(c);

                // 멀티 스레딩 방식으로 체크하기 때문에 동일한 결과가 중복되어 나올 수 있음.
                // 여기서 중복되는 결과를 제거.
                effect_note.r_loop_swap_remove(|&n| !cell.get_chk(n));

                if effect_note.is_empty() {
                    continue;
                }

                let backup = cell.clone_chk_list_rand();
                backup_chk.push((c, backup));

                cell.set_to_false_list(effect_note);

                // 변경된 Cell의 checked_zone 캐시를 초기화
                // changed_zone_set에 이미 초기화한 Zone을 넣어서 중복 초기화 방지
                for zone in &c.zone_vec {
                    if !changed_zone_set.contains(zone) {
                        self.zone_cache.checked_zone()[zone]
                            .iter()
                            .for_each(|(_, value)| value.set(false));
                        changed_zone_set.insert(*zone);
                    }
                }
            }

            if backup_chk.is_empty() {
                continue;
            }

            *self
                .solve_cnt
                .get_mut(&SolverSimple::convert_detail_to_simple(
                    &solver_result.solver_type,
                ))
                .unwrap() += 1;

            self.solver_history_stack.push(SolverHistory {
                history_type: SolverHistoryType::Solve { solver_result },
                backup_chk,
            });
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
            for (c, backup) in &history.backup_chk {
                write.write_from_cell(c).set_to_chk_list(backup);
            }

            // Rollback된 Cell의 checked_zone 캐시를 초기화
            self.zone_cache
                .checked_zone_clear(history.backup_chk.iter().map(|(c, _)| *c));

            if let SolverHistoryType::GuessBacktrace { .. } = history.history_type {
                self.guess_backtrace_rollback_cnt += 1;
            }

            // 추측된 숫자를 실패로 간주하여 제외시킴
            // Guess를 만난 경우 롤백 중단
            if let SolverHistoryType::Guess { cell, final_num } = history.history_type {
                let mut_chk = write.write_from_cell(cell);

                let backup_chk_list = mut_chk.clone_chk_list_rand();
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

    /// TableLock을 mut로 받을 필요는 없으나, 동일한 Table에 대해 여러 Solver를 생성하는 것을 방지하기 위해 일부러 mut로 받음
    #[must_use]
    pub fn new(t: &'a mut TableLock<N>) -> Self {
        let rand_seed = rand::rngs::OsRng.next_u64();
        Self::new_with_seed(t, rand_seed)
    }

    #[must_use]
    pub fn new_with_seed(t: &'a mut TableLock<N>, rand_seed: u64) -> Self {
        let mut solve_cnt: IndexKeyMap<SolverSimple, u32> = IndexKeyMap::new();
        for n in all::<SolverSimple>() {
            solve_cnt.insert(n, 0u32);
        }

        Solver {
            table: t,
            solver_history_stack: Vec::with_capacity(N * N * N),
            rng: SmallRng::seed_from_u64(rand_seed),
            rand_seed,
            guess_cnt: 0,
            guess_rollback_cnt: 0,
            guess_backtrace_rollback_cnt: 0,
            solve_cnt,
            zone_cache: ZoneCache::new(t),
        }
    }

    #[must_use]
    pub fn new_with_cache(t: &'a TableLock<N>, zone_cache: ZoneCache<'a, N>) -> Self {
        let mut solve_cnt: IndexKeyMap<SolverSimple, u32> = IndexKeyMap::new();
        for n in all::<SolverSimple>() {
            solve_cnt.insert(n, 0u32);
        }
        let rand_seed = rand::rngs::OsRng.next_u64();

        Solver {
            table: t,
            solver_history_stack: Vec::with_capacity(N * N * N),
            rng: SmallRng::seed_from_u64(rand_seed),
            rand_seed,
            guess_cnt: 0,
            guess_rollback_cnt: 0,
            guess_backtrace_rollback_cnt: 0,
            solve_cnt,
            zone_cache,
        }
    }

    #[must_use]
    #[inline]
    pub fn into_punch(self) -> Punch<'a, N> {
        Punch::new(self.table, self.rng, self.zone_cache)
    }

    #[must_use]
    pub fn get_table(&self) -> &TableLock<N> {
        self.table
    }

    #[must_use]
    pub fn get_solver_history(&self) -> &Vec<SolverHistory<'a, N>> {
        &self.solver_history_stack
    }
}

impl<'a, const N: usize> GeneralSolve<'a> for Solver<'a, N> {
    /// 제한시간 내에 스도쿠를 모두 채우려고 시도합니다.
    ///
    /// 풀리지 않고 남은 cell의 개수를 반환합니다.
    /// 제한시간을 초과했거나 풀 수 없는 문제인 경우 1이상일 수 있습니다.
    fn fill_puzzle_with_timeout(&mut self, timeout: Duration) -> usize {
        let start = Instant::now();

        loop {
            let unsolved_cell_cnt = self.get_unsolved_cell_cnt();
            if unsolved_cell_cnt == 0 {
                break;
            }

            // println!("{}", self.t);
            // println!("-----------------------------");
            // timeout 또는 모든 문제를 풀 수 없는 경우 return
            if (Instant::now() - start) >= timeout || !self.fill_once() {
                return unsolved_cell_cnt;
            }
        }

        0
    }

    /// solver를 적용하여 문제를 풉니다. solver가 풀지 못한 경우 guess합니다.
    fn fill_once(&mut self) -> bool {
        if self.solve() {
            true
        } else {
            // 푸는게 실패할 경우 guess를 적용
            if self.guess_random() {
                self.guess_cnt += 1;
                true
            } else {
                false
            }
        }
    }

    /// solver를 적용하여 문제를 풉니다. 문제는 푼 경우 true, 풀지 못한 경우 false를 반환합니다.
    fn solve(&mut self) -> bool {
        let read = self.table.read_lock();
        self.table.table_debug_validater();

        let result_list: Mutex<Vec<SolverResult<N>>> = Mutex::new(Vec::new());
        let mut error_cell: Option<&Cell<N>> = None;
        let is_break = NonAtomicBool::new(false);

        rayon::scope_fifo(|s| {
            s.spawn_fifo(|_| {
                // print!("VAL ");
                // 먼저 오류가 있는지 체크하여 있을 경우 롤백
                error_cell = self.validater_inner(&read);
                if error_cell.is_some() {
                    is_break.set(true);
                }
            });

            s.spawn_fifo(|s| {
                // Single Solver 적용
                // print!("SINGLE ");
                self.single(&read, s, &result_list, &is_break);
            });

            s.spawn_fifo(|s| {
                // Naked Solver 적용
                // print!("NAKED ");
                self.naked(&read, s, &result_list, &is_break);
            });

            s.spawn_fifo(|s| {
                // print!("BLR ");
                // Box Line Reduction Solver 적용
                self.box_line_reduction(&read, s, &result_list, &is_break);
            });

            false
        });

        if error_cell.is_some() {
            // println!("ROLLBACK");
            return self.history_rollback_last_guess(read);
        }

        let result_list = result_list.into_inner().unwrap();
        if result_list.is_empty() {
            false
        } else {
            self.solve_result_commit(read, result_list);
            true
        }
    }

    /// 이 스도쿠 퍼즐의 미완성 Cell 개수 반환
    #[must_use]
    fn get_unsolved_cell_cnt(&self) -> usize {
        let read = self.table.read_lock();
        self.table
            .into_iter()
            .filter(|c| !read.read_from_cell(c).is_final_num())
            .count()
    }

    #[must_use]
    fn get_random_seed(&self) -> u64 {
        self.rand_seed
    }

    fn set_random_seed(&mut self, rand_seed: u64) {
        self.rng = SmallRng::seed_from_u64(rand_seed);
        self.rand_seed = rand_seed;
    }

    /// Get the solver's solve cnt.
    #[must_use]
    #[inline]
    fn solve_cnt(&self, result_simple: SolverSimple) -> u32 {
        self.solve_cnt[&result_simple]
    }

    /// Get the solver's guess cnt.
    #[must_use]
    #[inline]
    fn guess_cnt(&self) -> u32 {
        self.guess_cnt
    }

    /// Get the solver's guess rollback cnt.
    #[must_use]
    #[inline]
    fn guess_backtrace_rollback_cnt(&self) -> u32 {
        self.guess_backtrace_rollback_cnt
    }

    /// Get the solver's guess rollback cnt.
    #[must_use]
    #[inline]
    fn guess_rollback_cnt(&self) -> u32 {
        self.guess_rollback_cnt
    }
}

impl<'a, const N: usize> PartialEq for Solver<'a, N> {
    fn eq(&self, other: &Self) -> bool {
        self.table == other.table
    }
}

impl<'a, const N: usize> Eq for Solver<'a, N> {}

impl<'a, const N: usize> Debug for Solver<'a, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Solver").field("t", &self.table).finish()
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
