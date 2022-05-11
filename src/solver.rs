use hashbrown::HashMap;
use rusty_pool::ThreadPool;

use crate::{cell::Cell, coordinate::Coordinate, table::Table, zone::Zone};

use self::solver_history::{SolverHistory, SolverHistoryType, SolverResult};

pub mod guess;
pub mod naked;
pub mod solver_history;
pub mod validater;

pub struct Solver<'a> {
    t: &'a Table,
    zone_list: Vec<&'a Zone>,
    ref_cache: HashMap<&'a Zone, Vec<&'a Cell>>,
    pool: ThreadPool,
    solver_history_stack: Vec<SolverHistory<'a>>,
}

impl<'a> Solver<'a> {
    pub fn solve(&mut self) {
        let result = self.naked();

        self.shutdown_solve_thread_pool();

        // 오류가 있는지 체크하여 있을 경우 롤백
        if self.find_error_cell().is_some() {
            self.history_rollback_last_guess();
        }
    }

    /// 스도푸를 푼 경우 해당 결과를 적용합니다.
    pub fn solve_result_commit(&mut self, result: Option<SolverResult<'a>>) -> bool {
        if let Some(solver_result) = result {
            let history = {
                let mut backup_chk: HashMap<&'a Cell, Vec<usize>> =
                    HashMap::with_capacity(solver_result.get_effect_cells().len());

                for c in solver_result.get_effect_cells().keys() {
                    let backup = c.chk.borrow().clone_chk_list();
                    backup_chk.insert(c, backup);
                }

                SolverHistory {
                    history_type: SolverHistoryType::Solve { solver_result },
                    backup_chk,
                }
            };

            if let SolverHistoryType::Solve { solver_result } = history.history_type {
                for (c, v) in solver_result.get_effect_cells() {
                    c.chk.borrow_mut().set_to_false_list(v);
                }
            } else {
                panic!("뭔가 잘못됨")
            }

            return true;
        }

        false
    }

    pub fn shutdown_solve_thread_pool(&mut self) {
        let old_pool: ThreadPool = std::mem::take(&mut self.pool);
        old_pool.shutdown();
    }

    fn history_rollback_last_guess(&mut self) {
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
            return;
        }

        while let Some(history) = self.solver_history_stack.pop() {
            for (c, backup) in history.backup_chk {
                c.chk.borrow_mut().set_to_chk_list(&backup);
            }

            // 추측된 숫자를 실패로 간주하여 제외시킴
            // Guess를 만난 경우 롤백 중단
            if let SolverHistoryType::Guess { cell, final_num } = history.history_type {
                let mut mut_chk = cell.chk.borrow_mut();

                let backup_chk_list: Vec<usize> = mut_chk.clone_chk_list();
                let mut backup: HashMap<&'a Cell, Vec<usize>> = HashMap::with_capacity(1);
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
    }

    #[must_use]
    pub fn new(t: &'a Table) -> Self {
        let zone_list = Solver::get_zone_list_init(t.get_cell(), t.get_size());
        Solver {
            t,
            zone_list,
            ref_cache: Solver::get_zone_ref(t),
            pool: ThreadPool::default(),
            solver_history_stack: Vec::new(),
        }
    }

    #[must_use]
    fn get_zone_ref(t: &'a Table) -> HashMap<&'a Zone, Vec<&'a Cell>> {
        let size: usize = t.get_size();
        let mut zone_ref: HashMap<&'a Zone, Vec<&'a Cell>> = HashMap::with_capacity(size * size);
        for (_, this_cell) in t.get_cell() {
            for z in this_cell.get_zone() {
                let row: &mut Vec<&Cell> = zone_ref
                    .entry(z)
                    .or_insert_with(|| Vec::with_capacity(size));
                row.push(this_cell);
            }
        }

        zone_ref
    }

    #[must_use]
    fn get_zone_list_init(cells: &'a HashMap<Coordinate, Cell>, size: usize) -> Vec<&'a Zone> {
        let mut ret: Vec<&'a Zone> = Vec::with_capacity(size);

        for (_, this_cell) in cells {
            for z in this_cell.get_zone() {
                if !ret.contains(&z) {
                    ret.push(z);
                }
            }
        }
        ret
    }

    #[must_use]
    #[inline]
    pub fn get_zone_list(&self) -> &Vec<&Zone> {
        &self.zone_list
    }

    #[inline]
    /// 지정된 Zone을 순회하는 Iterator를 반환합니다.
    pub fn zone_iter<'b>(&'b self, zone: &'b Zone) -> std::slice::Iter<'_, &'a Cell> {
        self.ref_cache[zone].iter()
    }

    #[must_use]
    #[inline]
    pub fn get_cache(&self) -> &HashMap<&Zone, Vec<&Cell>> {
        &self.ref_cache
    }
}
