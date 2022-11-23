use super::solver_history::SolverResultDetail;
use enum_iterator::Sequence;

#[derive(Sequence, Debug)]
pub enum SolverSimple {
    Validate,
    Single,
    Naked,
    BoxLineReduction,
}

impl SolverSimple {
    pub fn convert_detail_to_simple(detail: &SolverResultDetail) -> Self {
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

impl std::hash::Hash for SolverSimple {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Clone for SolverSimple {
    fn clone(&self) -> Self {
        match self {
            Self::Validate => Self::Validate,
            Self::Single => Self::Single,
            Self::Naked => Self::Naked,
            Self::BoxLineReduction => Self::BoxLineReduction,
        }
    }
}
