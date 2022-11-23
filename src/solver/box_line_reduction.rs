use super::{solver_history::SolverResult, solver_simple::SolverSimple, Solver};
use crate::model::{ref_zone::RefZone, zone::ZoneType};

impl<'a> Solver<'a> {
    pub fn box_line_reduction(
        &self,
        zone_ref_with_read: &Vec<RefZone<'a>>,
    ) -> Vec<SolverResult<'a>> {
        for z1 in zone_ref_with_read {
            let ZoneType::Unique = z1.zone.get_zone_type() else { continue; };

            if self.checked_zone_get_bool(z1.zone, SolverSimple::BoxLineReduction) {
                continue;
            }

            for z2 in zone_ref_with_read {
                let ZoneType::Unique = z2.zone.get_zone_type() else { continue; };
            }

            self.checked_zone_set_bool_true(z1.zone, SolverSimple::BoxLineReduction);
        }

        Vec::new()
    }
}
