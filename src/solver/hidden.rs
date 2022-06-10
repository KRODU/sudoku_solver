use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use crate::{
    cell::Cell,
    zone::{Zone, ZoneType},
};

use super::{
    solver_history::{SolverResult, SolverResultDetail},
    Solver,
};

impl<'a> Solver<'a> {
    pub fn hidden(&self, changed_zone: &HashSet<&'a Zone>) -> Option<SolverResult<'a>> {
        for z in changed_zone {
            if let ZoneType::Unique = z.get_zone_type() {
                for i in 2..=self.t.get_size() / 2 {
                    let result = self.hidden_number_zone(z, i);

                    if result.is_some() {
                        return result;
                    }
                }
            }
        }

        None
    }

    fn hidden_number_zone(&self, z: &Zone, i: u32) -> Option<SolverResult<'a>> {
        let mut chk: Vec<&Cell> = Vec::new();

        for c in self.zone_iter(z) {
            let borrow = c.chk.borrow();
            if borrow.get_true_cnt() == i {
                chk.push(c);
            }
        }

        let mut effect_cells: HashMap<&Cell, HashSet<u32>> = HashMap::new();

        for r in chk.into_iter().combinations(i as usize) {
            let naked_value = r[0].chk.borrow();
            // zone을 순회하며 삭제할 노트가 있는지 찾음
            for zone_cell in self.zone_iter(z) {
                // 순회 대상에서 자기 자신은 제외
                if r.contains(zone_cell) {
                    continue;
                }

                let b = zone_cell.chk.borrow();
                let union: HashSet<u32> = b.intersection_note(&naked_value.clone_chk_list());

                // 제거할 노트를 발견한 경우
                if !union.is_empty() {
                    effect_cells.insert(zone_cell, union);
                }
            }

            // effect_cells에 값이 존재하는 경우 제거한 노트를 발견한 것임.
            // 해당 값을 return하고 종료
            if !effect_cells.is_empty() {
                let mut found_cells: HashSet<&'a Cell> = HashSet::with_capacity(r.len());
                for effect_cell in r {
                    found_cells.insert(effect_cell);
                }
                return Some(SolverResult {
                    solver_type: SolverResultDetail::Hidden {
                        found_chks: naked_value.clone_chk_list(),
                    },
                    found_cells,
                    effect_cells,
                });
            }
        }

        None
    }
}
