use super::solver_history::{SolverResult, SolverResultDetail};
use super::solver_simple::SolverSimple;
use super::Solver;
use crate::model::array_vector::ArrayVector;
use crate::model::note::Note;
use crate::model::{cell::Cell, ref_zone::RefZone, zone::ZoneType};

impl<'a, const N: usize> Solver<'a, N> {
    pub fn single(&self, zone_ref_with_read: &Vec<RefZone<'a, N>>) -> Vec<SolverResult<'a, N>> {
        for z in zone_ref_with_read {
            let ZoneType::Unique = z.zone.get_zone_type() else { continue; };

            if self.checked_zone_get_bool(z.zone, SolverSimple::Single) {
                continue;
            }

            for c in &z.cells {
                let Some(final_num) = c.read.get_final_num() else { continue; };
                let mut effect_cells: Vec<(&Cell<N>, ArrayVector<Note<N>, N>)> = Vec::new();

                for c_comp in &z.cells {
                    // 노트가 확정된 경우 Zone을 순회하면서 해당 노트를 가진 cell이 있나 찾음

                    // 나 자신은 비교 대상에서 제외
                    if c_comp == c {
                        continue;
                    }

                    // 찾음
                    if c_comp.read.get_chk(final_num) {
                        let mut note_vec = ArrayVector::new();
                        note_vec.push(final_num);
                        effect_cells.push((c_comp.cell, note_vec));
                    }
                }

                if !effect_cells.is_empty() {
                    // 하나 이상의 삭제할 노트를 가진 cell을 찾을 경우
                    let solver_result: SolverResult<'a, N> = SolverResult {
                        solver_type: SolverResultDetail::Single {
                            found_chk: final_num,
                        },
                        effect_cells,
                    };

                    return vec![solver_result];
                }
            }

            self.checked_zone_set_bool_true(z.zone, SolverSimple::Single);
        }

        Vec::new()
    }
}
