use hashbrown::{HashMap, HashSet};

use crate::cell::Cell;

#[derive(Debug)]
pub enum SolverResultType {
    Naked { found_chks: Vec<u32> },
}

#[derive(Debug)]
pub struct SolverResult<'a> {
    pub solver_type: SolverResultType,
    pub found_cells: HashSet<&'a Cell>,
    pub effect_cells: HashMap<&'a Cell, Vec<u32>>,
}

#[derive(Debug)]
pub enum SolverHistoryType<'a> {
    /// solver에 의해 스도쿠를 푼 경우입니다.
    Solve { solver_result: SolverResult<'a> },

    /// 스도쿠에서 임의의 값을 추측한 경우입니다.
    Guess { cell: &'a Cell, final_num: u32 },

    /// Guess가 실패한 경우 실패항 guess 숫자를 제외하게 됩니다.
    /// 그것을 추적하기 위한 타입입니다.
    GuessBacktrace { cell: &'a Cell, except_num: u32 },
}

#[derive(Debug)]
pub struct SolverHistory<'a> {
    pub history_type: SolverHistoryType<'a>,
    pub backup_chk: HashMap<&'a Cell, Vec<u32>>,
}
