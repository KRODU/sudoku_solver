use crate::zone::Zone;

use super::solver_history::SolverResult;

pub enum SolverSkipType {
    Naked,
}

impl PartialEq for SolverSkipType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for SolverSkipType {}

impl std::hash::Hash for SolverSkipType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Clone for SolverSkipType {
    fn clone(&self) -> Self {
        match self {
            Self::Naked => Self::Naked,
        }
    }
}

pub struct SovlerSkipResult<'a> {
    pub skip_type: SolverSkipType,
    pub skip_zone: Vec<Zone>,
    pub solver_result: Option<SolverResult<'a>>,
}
