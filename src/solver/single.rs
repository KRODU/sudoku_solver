use super::solver_history::{SolverResult, SolverResultDetail};
use super::solver_simple::SolverSimple;
use super::Solver;
use crate::model::array_vector::ArrayVector;
use crate::model::max_num::MaxNum;
use crate::model::table_lock::TableLockReadGuard;
use crate::model::{cell::Cell, zone::ZoneType};
use std::sync::atomic::{AtomicBool, Ordering};

impl<'a, const N: usize> Solver<'a, N> {
    pub fn single(
        &self,
        read: &TableLockReadGuard<N>,
        is_break: &AtomicBool,
    ) -> Vec<SolverResult<'a, N>> {
        for (zone, cells) in &self.ordered_zone {
            if is_break.load(Ordering::Relaxed) {
                break;
            }

            let ZoneType::Unique = zone.get_zone_type() else { continue; };

            if self.checked_zone_get_bool(zone, SolverSimple::Single) {
                continue;
            }

            for c in cells {
                let Some(final_num) = read.read_from_cell(c).get_final_num() else { continue; };
                let mut effect_cells: Vec<(&Cell<N>, ArrayVector<MaxNum<N>, N>)> = Vec::new();

                for c_comp in cells {
                    // 노트가 확정된 경우 Zone을 순회하면서 해당 노트를 가진 cell이 있나 찾음

                    // 나 자신은 비교 대상에서 제외
                    if c_comp == c {
                        continue;
                    }

                    // 찾음
                    if read.read_from_cell(c_comp).get_chk(final_num) {
                        let mut note_vec = ArrayVector::new();
                        note_vec.push(final_num);
                        effect_cells.push((c_comp, note_vec));
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

                    is_break.store(true, Ordering::Relaxed);
                    return vec![solver_result];
                }
            }

            self.checked_zone_set_bool_true(zone, SolverSimple::Single);
        }

        Vec::new()
    }
}
