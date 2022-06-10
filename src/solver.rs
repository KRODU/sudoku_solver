use std::{
    cell::RefCell,
    time::{Duration, Instant},
};

use enum_iterator::IntoEnumIterator;
use hashbrown::{HashMap, HashSet};
use rand::{prelude::StdRng, RngCore, SeedableRng};

use crate::{
    cell::Cell,
    table::Table,
    zone::{Zone, ZoneType},
};

use self::{
    solver_history::{SolverHistory, SolverHistoryType, SolverResult},
    solver_skip_result::SolverResultSimple,
};

pub mod guess;
pub mod hidden;
pub mod naked;
pub mod single;
pub mod solver_history;
pub mod solver_skip_result;
pub mod validater;

pub struct Solver<'a> {
    t: &'a Table,
    zone_list: HashSet<&'a Zone>,
    ref_cache: HashMap<&'a Zone, Vec<&'a Cell>>,
    solver_history_stack: Vec<SolverHistory<'a>>,
    rng: RefCell<StdRng>,
    rand_seed: u64,
    solve_cnt: HashMap<SolverResultSimple, u32>,
    guess_cnt: u32,
    guess_rollback_cnt: u32,
    guess_backtrace_rollback_cnt: u32,
    changed_cell: HashSet<&'a Cell>,
    init: bool,
}

impl<'a> Solver<'a> {
    /// 제한시간 내에 스도쿠를 모두 채우려고 시도합니다.
    ///
    /// 풀리지 않고 남은 cell의 개수를 반환합니다.
    /// 제한시간을 초과했거나 풀 수 없는 문제인 경우 1이상일 수 있습니다.
    pub fn fill_puzzle_with_timeout(&mut self, timeout: Duration) -> u32 {
        let start = Instant::now();

        while !self.is_complete_puzzle() {
            println!("{}", self.t); // FIXME
            println!("-----------------------------");
            // timeout 또는 모든 문제를 풀 수 없는 경우 return
            if (Instant::now() - start) >= timeout || self.solve().is_none() {
                return self
                    .t
                    .into_iter()
                    .filter(|n| !n.chk.borrow().is_final_num())
                    .count() as u32;
            }
        }

        0
    }

    fn solve_history_rollback_last_guess(&self) -> Option<&SolverHistory<'a>> {
        Some(&self.solver_history_stack[self.solver_history_stack.len() - 1])
    }

    pub fn solve(&mut self) -> Option<&SolverHistory<'a>> {
        let mut changed_zone: HashSet<&'a Zone> = HashSet::new();
        for c in &self.changed_cell {
            for z in c.get_zone() {
                changed_zone.insert(z);
            }
        }

        // 먼저 오류가 있는지 체크하여 있을 경우 롤백
        if self.find_error_cell().is_some() {
            if self.history_rollback_last_guess() {
                return self.solve_history_rollback_last_guess();
            } else {
                return None;
            }
        }

        // Single Solver 적용
        let mut result = self.single();
        if self.solve_result_commit(result) {
            return self.solve_history_rollback_last_guess();
        }

        // Naked Solver 적용
        result = self.naked(&changed_zone);
        if self.solve_result_commit(result) {
            return self.solve_history_rollback_last_guess();
        }

        // 푸는게 실패할 경우 guess를 적용
        self.changed_cell.clear();
        self.guess_random();
        self.guess_cnt += 1;
        self.solve_history_rollback_last_guess()
    }

    /// 스도푸를 푼 경우 해당 결과를 적용합니다.
    pub fn solve_result_commit(&mut self, result: Option<SolverResult<'a>>) -> bool {
        if let Some(solver_result) = result {
            let history = {
                let mut backup_chk: HashMap<&'a Cell, HashSet<u32>> =
                    HashMap::with_capacity(solver_result.effect_cells.len());

                for c in solver_result.effect_cells.keys() {
                    let backup = c.chk.borrow().clone_chk_list();
                    backup_chk.insert(c, backup);
                }

                SolverHistory {
                    history_type: SolverHistoryType::Solve { solver_result },
                    backup_chk,
                }
            };

            if let SolverHistoryType::Solve { ref solver_result } = history.history_type {
                for (c, v) in &solver_result.effect_cells {
                    c.chk.borrow_mut().set_to_false_list(v);
                    self.changed_cell.insert(c);
                }

                *self
                    .solve_cnt
                    .get_mut(&SolverResultSimple::convert_detail_to_simple(
                        &solver_result.solver_type,
                    ))
                    .unwrap() += 1;
            } else {
                panic!("뭔가 잘못됨")
            }

            self.solver_history_stack.push(history);

            return true;
        }

        false
    }

    /// 가장 최근의 guess까지 롤백
    fn history_rollback_last_guess(&mut self) -> bool {
        let mut no_guess: bool = true;
        for history in &self.solver_history_stack {
            if let SolverHistoryType::Guess {
                cell: _,
                final_num: _,
            } = history.history_type
            {
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
                c.chk.borrow_mut().set_to_chk_list(&backup);
                self.changed_cell.insert(c);
            }

            if let SolverHistoryType::GuessBacktrace {
                cell: _,
                except_num: _,
            } = history.history_type
            {
                self.guess_backtrace_rollback_cnt += 1;
            }

            // 추측된 숫자를 실패로 간주하여 제외시킴
            // Guess를 만난 경우 롤백 중단
            if let SolverHistoryType::Guess { cell, final_num } = history.history_type {
                let mut mut_chk = cell.chk.borrow_mut();

                let backup_chk_list: HashSet<u32> = mut_chk.clone_chk_list();
                let mut backup: HashMap<&'a Cell, HashSet<u32>> = HashMap::with_capacity(1);
                backup.insert(cell, backup_chk_list);
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
        self.t.into_iter().all(|c| c.chk.borrow().is_final_num())
    }

    #[must_use]
    pub fn new(t: &'a mut Table) -> Self {
        let size = t.get_size();
        let rand_seed: u64 = StdRng::from_entropy().next_u64();
        let zone_list = Solver::get_zone_list_init(t, size);
        let mut solve_cnt: HashMap<SolverResultSimple, u32> = HashMap::new();
        for n in SolverResultSimple::into_enum_iter() {
            solve_cnt.insert(n, 0u32);
        }

        let mut changed_cell = HashSet::with_capacity((size * size) as usize);
        for c in t.into_iter() {
            changed_cell.insert(c);
        }

        Solver {
            t,
            zone_list,
            ref_cache: Solver::get_zone_ref(t),
            solver_history_stack: Vec::new(),
            rng: RefCell::new(rand::prelude::StdRng::seed_from_u64(rand_seed)),
            rand_seed,
            guess_cnt: 0,
            guess_rollback_cnt: 0,
            guess_backtrace_rollback_cnt: 0,
            solve_cnt,
            changed_cell,
            init: true,
        }
    }

    #[must_use]
    fn get_zone_ref(t: &'a Table) -> HashMap<&'a Zone, Vec<&'a Cell>> {
        let size = t.get_size();
        let mut zone_ref: HashMap<&'a Zone, Vec<&'a Cell>> =
            HashMap::with_capacity((size * size) as usize);
        for this_cell in t {
            for z in this_cell.get_zone() {
                let row: &mut Vec<&Cell> = zone_ref
                    .entry(z)
                    .or_insert_with(|| Vec::with_capacity(size as usize));
                row.push(this_cell);
            }
        }

        // 퍼즐 유효성 체크
        for (z, c) in &zone_ref {
            if let ZoneType::Unique = z.zone_type {
                if c.len() != size as usize {
                    panic!(
                        "Unique 타입의 개수는 퍼즐 사이즈와 동일해야 함! 사이즈:{}, 실제 갯수: {}",
                        size,
                        c.len()
                    )
                }
            }
        }
        zone_ref
    }

    #[must_use]
    fn get_zone_list_init(cells: &'a Table, size: u32) -> HashSet<&'a Zone> {
        let mut ret: HashSet<&'a Zone> = HashSet::with_capacity(size as usize);

        for this_cell in cells {
            for z in this_cell.get_zone() {
                if !ret.contains(&z) {
                    ret.insert(z);
                }
            }
        }
        ret
    }

    #[must_use]
    #[inline]
    pub fn get_zone_list(&self) -> &HashSet<&Zone> {
        &self.zone_list
    }

    #[inline]
    /// 지정된 Zone을 순회하는 Iterator를 반환합니다.
    pub fn zone_iter<'b>(&'b self, zone: &'b Zone) -> std::slice::Iter<&'a Cell> {
        self.ref_cache[zone].iter()
    }

    pub fn get_table(&self) -> &Table {
        self.t
    }

    #[must_use]
    #[inline]
    pub fn get_cache(&self) -> &HashMap<&Zone, Vec<&Cell>> {
        &self.ref_cache
    }

    pub fn get_random_seed(&self) -> u64 {
        self.rand_seed
    }

    pub fn set_random_seed(&mut self, rand_seed: u64) {
        self.rng = RefCell::new(rand::prelude::StdRng::seed_from_u64(rand_seed));
        self.rand_seed = rand_seed;
    }

    /// Get the solver's solve cnt.
    #[must_use]
    #[inline]
    pub fn solve_cnt(&self, result_simple: &SolverResultSimple) -> u32 {
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
