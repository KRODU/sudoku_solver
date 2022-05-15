use hashbrown::{HashMap, HashSet};

use crate::{
    cell::Cell,
    combinations::combinations,
    zone::{Zone, ZoneType},
};

use super::{
    solver_history::{SolverResult, SolverResultType},
    solver_skip_result::{SolverSkipResult, SolverSkipType},
    Solver,
};

impl<'a> Solver<'a> {
    pub fn hidden(&self) -> SolverSkipResult<'a> {
        let mut total_skip_list: Vec<Zone> = Vec::new();

        for z in self.get_zone_list() {
            if let ZoneType::Unique = z.get_zone_type() {
                if self.skip_this[z].contains(&SolverSkipType::Hidden) {
                    continue;
                }

                for i in 1..self.t.get_size() {
                    let result = self.hidden_number_zone(z, i);

                    if result.is_some() {
                        return SolverSkipResult {
                            skip_type: SolverSkipType::Hidden,
                            skip_zone: total_skip_list,
                            solver_result: result,
                        };
                    }
                }

                total_skip_list.push((*z).clone());
            }
        }

        SolverSkipResult {
            skip_type: SolverSkipType::Hidden,
            skip_zone: total_skip_list,
            solver_result: None,
        }
    }

    fn hidden_number_zone(&self, z: &Zone, i: u32) -> Option<SolverResult<'a>> {
        let mut chk: Vec<&Cell> = Vec::new();

        for c in self.zone_iter(z) {
            let borrow = c.chk.borrow();
            if borrow.get_true_cnt() == i {
                chk.push(c);
            }
        }

        let com_result = combinations(&chk, i as usize, |comb| {
            let first_node = comb[0].chk.borrow();
            comb.iter()
                .all(|c| c.chk.borrow().is_same_note(&*first_node))
        });

        let mut effect_cells: HashMap<&Cell, Vec<u32>> = HashMap::new();

        for r in com_result {
            let naked_value = r[0].chk.borrow();
            // zone을 순회하며 삭제할 노트가 있는지 찾음
            for zone_cell in self.zone_iter(z) {
                // 순회 대상에서 자기 자신은 제외
                if r.contains(&zone_cell) {
                    continue;
                }

                let b = zone_cell.chk.borrow();
                let union: Vec<u32> = b.union_note(&*naked_value);

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
                    found_cells.insert(*effect_cell);
                }
                return Some(SolverResult {
                    solver_type: SolverResultType::Naked {
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
