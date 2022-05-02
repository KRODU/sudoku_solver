use hashbrown::{HashMap, HashSet};

use crate::cell::Cell;

pub enum SolverType {
    Naked { found_chks: Vec<usize> },
}

pub struct SolverResult<'a> {
    solver_type: SolverType,
    found_cells: HashSet<&'a Cell>,
    effect_cells: HashMap<&'a Cell, Vec<usize>>,
}

impl<'a> SolverResult<'a> {
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
