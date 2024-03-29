use crate::model::{array_vector::ArrayVector, cell::Cell, max_num::MaxNum};

#[derive(Debug, Clone)]
pub enum SolverResultDetail<'a, const N: usize> {
    Single {
        found_chk: MaxNum<N>,
    },
    Naked {
        found_chks: ArrayVector<MaxNum<N>, N>,
        found_cell: Vec<&'a Cell<N>>,
    },
    BoxLineReduction {
        found_chk: MaxNum<N>,
    },
}

#[derive(Debug, Clone)]
pub struct SolverResult<'a, const N: usize> {
    pub solver_type: SolverResultDetail<'a, N>,
    pub effect_cells: Vec<(&'a Cell<N>, ArrayVector<MaxNum<N>, N>)>,
}

#[derive(Debug, Clone)]
pub enum SolverHistoryType<'a, const N: usize> {
    /// solver에 의해 스도쿠를 푼 경우입니다.
    Solve { solver_result: SolverResult<'a, N> },

    /// 스도쿠에서 임의의 값을 추측한 경우입니다.
    Guess {
        cell: &'a Cell<N>,
        final_num: MaxNum<N>,
    },

    /// Guess가 실패한 경우 실패한 guess 숫자를 제외하게 됩니다.
    /// 그것을 추적하기 위한 타입입니다.
    GuessBacktrace {
        cell: &'a Cell<N>,
        except_num: MaxNum<N>,
    },
}

#[derive(Debug, Clone)]
pub struct SolverHistory<'a, const N: usize> {
    pub history_type: SolverHistoryType<'a, N>,
    pub backup_chk: Vec<(&'a Cell<N>, ArrayVector<MaxNum<N>, N>)>,
}
