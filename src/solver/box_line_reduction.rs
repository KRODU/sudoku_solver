use super::{
    solver_history::{SolverResult, SolverResultDetail},
    solver_simple::SolverSimple,
    Solver,
};
use crate::model::{cell::Cell, ref_zone::RefZone, zone::ZoneType};
use hashbrown::HashSet;

impl<'a> Solver<'a> {
    pub fn box_line_reduction(
        &self,
        ref_zone_hash: &HashSet<&RefZone<'a>>,
    ) -> Vec<SolverResult<'a>> {
        for z1 in ref_zone_hash {
            let ZoneType::Unique = z1.zone.get_zone_type() else { continue; };

            if self.checked_zone_get_bool(z1.zone, SolverSimple::BoxLineReduction) {
                continue;
            }

            let Some(connect_zone) = self.connect_zone.get(z1.zone) else {
                continue;
            };

            let current_zone_union_note =
                z1.cells.iter().fold(HashSet::<usize>::new(), |mut h, c| {
                    let read = &c.read;
                    if read.get_true_cnt() > 1 {
                        read.union_note(&mut h);
                    }
                    h
                });

            for &z2 in connect_zone {
                let ZoneType::Unique = z2.get_zone_type() else { continue; };

                let Some(z2_ref) = ref_zone_hash.get(z2) else {
                    continue;
                };

                for &note in &current_zone_union_note {
                    let target_this_note = !z1.cells.iter().any(|c| {
                        if c.cell.zone_set.contains(z2) {
                            return false;
                        }

                        c.read.get_chk(note)
                    });

                    if target_this_note {
                        let mut effect_cells: Vec<(&'a Cell, Vec<usize>)> = Vec::new();

                        for z2_cell in &z2_ref.cells {
                            if z2_cell.cell.zone_set.contains(z1.zone) {
                                continue;
                            }

                            if z2_cell.read.get_chk(note) {
                                effect_cells.push((z2_cell.cell, vec![note]));
                            }
                        }

                        if !effect_cells.is_empty() {
                            return vec![SolverResult {
                                solver_type: SolverResultDetail::BoxLineReduction {
                                    found_chk: note,
                                },
                                effect_cells,
                            }];
                        }
                    }
                }
            }

            self.checked_zone_set_bool_true(z1.zone, SolverSimple::BoxLineReduction);
        }

        Vec::new()
    }
}
