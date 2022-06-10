use enum_iterator::IntoEnumIterator;

use super::solver_history::SolverResultDetail;

#[derive(IntoEnumIterator, Debug)]
pub enum SolverResultSimple {
    Single,
    Naked,
    Hidden,
}

impl SolverResultSimple {
    pub fn convert_detail_to_simple(detail: &SolverResultDetail) -> Self {
        match detail {
            SolverResultDetail::Single { .. } => SolverResultSimple::Single,
            SolverResultDetail::Naked { .. } => SolverResultSimple::Naked,
            SolverResultDetail::Hidden { .. } => SolverResultSimple::Hidden,
        }
    }
}

impl PartialEq for SolverResultSimple {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for SolverResultSimple {}

impl std::hash::Hash for SolverResultSimple {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Clone for SolverResultSimple {
    fn clone(&self) -> Self {
        match self {
            Self::Single => Self::Single,
            Self::Naked => Self::Naked,
            Self::Hidden => Self::Hidden,
        }
    }
}
