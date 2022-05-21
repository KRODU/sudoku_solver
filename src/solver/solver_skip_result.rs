use enum_iterator::IntoEnumIterator;

use crate::zone::Zone;

use super::solver_history::SolverResult;

#[derive(IntoEnumIterator, Debug)]
pub enum SolverResultSimple {
    Single,
    Naked,
    Hidden,
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

pub struct SolverSkipResult<'a> {
    pub skip_type: SolverResultSimple,
    pub skip_zone: Vec<Zone>,
    pub solver_result: Option<SolverResult<'a>>,
}
