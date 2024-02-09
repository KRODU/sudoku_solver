use super::solver_history::SolverResultDetail;
use crate::model::index_key_map::IndexKey;
use enum_iterator::Sequence;

#[derive(Sequence, Debug, Clone, Copy)]
pub enum SolverSimple {
    Validate,
    Single,
    Naked,
    BoxLineReduction,
}

impl SolverSimple {
    pub fn convert_detail_to_simple<const N: usize>(detail: &SolverResultDetail<N>) -> Self {
        match detail {
            SolverResultDetail::Single { .. } => SolverSimple::Single,
            SolverResultDetail::Naked { .. } => SolverSimple::Naked,
            SolverResultDetail::BoxLineReduction { .. } => SolverSimple::BoxLineReduction,
        }
    }
}

impl PartialEq for SolverSimple {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for SolverSimple {}

impl IndexKey for SolverSimple {
    fn index(&self) -> u16 {
        *self as u16
    }
}
