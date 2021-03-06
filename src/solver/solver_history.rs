use hashbrown::{HashMap, HashSet};

use crate::cell::Cell;

#[derive(Debug)]
pub enum SolverResultDetail {
    Single { found_chk: usize },
    Naked { found_chks: HashSet<usize> },
    Hidden { found_chks: HashSet<usize> },
}

#[derive(Debug)]
pub struct SolverResult<'a> {
    pub solver_type: SolverResultDetail,
    pub found_cells: HashSet<&'a Cell>,
    pub effect_cells: HashMap<&'a Cell, HashSet<usize>>,
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
    pub backup_chk: HashMap<&'a Cell, HashSet<usize>>,
}
