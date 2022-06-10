use hashbrown::{HashMap, HashSet};

use crate::{cell::Cell, zone::ZoneType};

use super::{
    solver_history::{SolverResult, SolverResultDetail},
    Solver,
};

impl<'a> Solver<'a> {
    pub fn single(&self) -> Option<SolverResult<'a>> {
        let mut effect_cells: HashMap<&Cell, HashSet<u32>> = HashMap::new();

        for c in &self.changed_cell {
            let b = c.chk.borrow();

            // 노트가 확정된 경우 Zone을 순회하면서 해당 노트를 가진 cell이 있나 찾음
            if let Some(final_num) = b.get_final_num() {
                for z in c.get_zone() {
                    if let ZoneType::Unique = z.zone_type {
                        for c_comp in self.zone_iter(z) {
                            // 나 자신은 비교 대상에서 제외
                            if c_comp == c {
                                continue;
                            }

                            // 찾음
                            if c_comp.chk.borrow().get_chk(final_num) {
                                let mut v: HashSet<u32> = HashSet::with_capacity(1);
                                v.insert(final_num);
                                effect_cells.insert(c_comp, v);
                            }
                        }
                    }
                }

                if !effect_cells.is_empty() {
                    // 하나 이상의 삭제할 노트를 가진 cell을 찾을 경우
                    let mut found_cells: HashSet<&'a Cell> = HashSet::with_capacity(1);
                    found_cells.insert(c);

                    let solver_result: Option<SolverResult<'a>> = Some(SolverResult {
                        solver_type: SolverResultDetail::Single {
                            found_chk: final_num,
                        },
                        found_cells,
                        effect_cells,
                    });

                    return solver_result;
                }
            }
        }

        None
    }
}
