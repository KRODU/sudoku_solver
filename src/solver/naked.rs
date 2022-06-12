use hashbrown::{HashMap, HashSet};

use crate::{
    cell::Cell,
    combinations::combinations,
    zone::{Zone, ZoneType},
};

use super::{
    solver_history::{SolverResult, SolverResultDetail},
    Solver,
};

impl<'a> Solver<'a> {
    pub fn naked(&self, changed_zone: &HashSet<&'a Zone>) -> Option<SolverResult<'a>> {
        for z in changed_zone {
            if let ZoneType::Unique = z.get_zone_type() {
                for i in 2..=self.t.get_size() / 2 {
                    let result = self.naked_number_zone(z, i);

                    if result.is_some() {
                        return result;
                    }
                }
            }
        }

        None
    }

    fn naked_number_zone(&self, z: &Zone, i: usize) -> Option<SolverResult<'a>> {
        let mut union_node: HashSet<usize> = HashSet::new();

        let comblist = combinations(&self.ref_cache[z], i, |arr| {
            if !arr.iter().any(|c| self.changed_cell.contains(*c)) {
                return None;
            }

            union_node.clear();
            for c in arr {
                let b = c.chk.read().unwrap();
                let note_cnt = b.get_true_cnt();

                if note_cnt == 1 {
                    return None;
                }

                if note_cnt > i {
                    return None;
                }
                b.union_note(&mut union_node);
                if union_node.len() > i {
                    return None;
                }
            }

            if union_node.len() != i {
                return None;
            }

            Some(union_node.clone())
        });
        std::mem::drop(union_node);

        // if !comblist.is_empty() {
        //     println!("i:{}, l:{}", i, comblist.len());
        // }
        let mut effect_cells: HashMap<&Cell, HashSet<usize>> = HashMap::new();
        for (cells, union_node) in comblist {
            // zone을 순회하며 삭제할 노트가 있는지 찾음
            for zone_cell in self.zone_iter(z) {
                // 순회 대상에서 자기 자신은 제외
                if cells.contains(&zone_cell) {
                    continue;
                }

                let b = zone_cell.chk.read().unwrap();
                let inter = b.intersection_note(&union_node);

                // 제거할 노트를 발견한 경우
                if !inter.is_empty() {
                    effect_cells.insert(zone_cell, inter);
                }
            }

            // effect_cells에 값이 존재하는 경우 제거한 노트를 발견한 것임.
            // 해당 값을 return하고 종료
            if !effect_cells.is_empty() {
                let mut found_cells: HashSet<&'a Cell> = HashSet::with_capacity(cells.len());
                for effect_cell in cells {
                    found_cells.insert(effect_cell);
                }
                return Some(SolverResult {
                    solver_type: SolverResultDetail::Naked {
                        found_chks: union_node,
                    },
                    found_cells,
                    effect_cells,
                });
            }
        }

        None
    }
}
