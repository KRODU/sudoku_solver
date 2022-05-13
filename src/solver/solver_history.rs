use hashbrown::{HashMap, HashSet};

use crate::cell::Cell;

#[derive(Debug)]
pub enum SolverType {
    Naked { found_chks: Vec<usize> },
}

#[derive(Debug)]
pub struct SolverResult<'a> {
    solver_type: SolverType,
    found_cells: HashSet<&'a Cell>,
    effect_cells: HashMap<&'a Cell, Vec<usize>>,
}

impl<'a, 'z> SolverResult<'a> {
    pub fn new(
        solver_type: SolverType,
        found_cells: HashSet<&'a Cell>,
        effect_cells: HashMap<&'a Cell, Vec<usize>>,
    ) -> Self {
        Self {
            solver_type,
            found_cells,
            effect_cells,
        }
    }

    pub fn get_solver_type(&self) -> &SolverType {
        &self.solver_type
    }

    pub fn get_found_cells(&self) -> &HashSet<&'a Cell> {
        &self.found_cells
    }

    pub fn get_effect_cells(&self) -> &HashMap<&'a Cell, Vec<usize>> {
        &self.effect_cells
    }
}

#[derive(Debug)]
pub enum SolverHistoryType<'a> {
    /// solver에 의해 스도쿠를 푼 경우입니다.
    Solve { solver_result: SolverResult<'a> },

    /// 스도쿠에서 임의의 값을 추측한 경우입니다.
    Guess { cell: &'a Cell, final_num: usize },

    /// Guess가 실패한 경우 실패항 guess 숫자를 제외하게 됩니다.
    /// 그것을 추적하기 위한 타입입니다.
    GuessBacktrace { cell: &'a Cell, except_num: usize },
}

#[derive(Debug)]
pub struct SolverHistory<'a> {
    pub history_type: SolverHistoryType<'a>,
    pub backup_chk: HashMap<&'a Cell, Vec<usize>>,
}
