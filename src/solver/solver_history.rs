use crate::model::{array_vector::ArrayVector, cell::Cell, note::Note};

#[derive(Debug)]
pub enum SolverResultDetail<const N: usize> {
    Single { found_chk: Note<N> },
    Naked { found_chks: ArrayVector<Note<N>, N> },
    BoxLineReduction { found_chk: Note<N> },
}

#[derive(Debug)]
pub struct SolverResult<'a, const N: usize> {
    pub solver_type: SolverResultDetail<N>,
    pub effect_cells: Vec<(&'a Cell<N>, ArrayVector<Note<N>, N>)>,
}

#[derive(Debug)]
pub enum SolverHistoryType<'a, const N: usize> {
    /// solver에 의해 스도쿠를 푼 경우입니다.
    Solve { solver_result: SolverResult<'a, N> },

    /// 스도쿠에서 임의의 값을 추측한 경우입니다.
    Guess {
        cell: &'a Cell<N>,
        final_num: Note<N>,
    },

    /// Guess가 실패한 경우 실패항 guess 숫자를 제외하게 됩니다.
    /// 그것을 추적하기 위한 타입입니다.
    GuessBacktrace {
        cell: &'a Cell<N>,
        except_num: Note<N>,
    },
}

#[derive(Debug)]
pub struct SolverHistory<'a, const N: usize> {
    pub history_type: SolverHistoryType<'a, N>,
    pub backup_chk: Vec<(&'a Cell<N>, ArrayVector<Note<N>, N>)>,
}
