use hashbrown::{HashMap, HashSet};

use crate::{
    cell::Cell,
    combinations::combinations,
    zone::{Zone, ZoneType},
};

use super::{
    solve_skip::{SkipThis, SkipType},
    solver_history::{SolverResult, SolverType},
    Solver,
};

impl<'a> Solver<'a> {
    pub fn naked(&self) -> (Vec<SkipThis>, Option<SolverResult<'a>>) {
        let mut total_skip_list: Vec<SkipThis> = Vec::new();

        for i in 1..self.t.get_size() {
            let mut result = self.naked_number(i);

            total_skip_list.append(&mut result.0);

            if result.1.is_some() {
                return (total_skip_list, result.1);
            }
        }

        (total_skip_list, None)
    }

    pub fn naked_number(&self, i: u32) -> (Vec<SkipThis>, Option<SolverResult<'a>>) {
        let mut skip_this_list: Vec<SkipThis> = Vec::new();

        // i의 값이 유효하지 않은 경우 return
        if i == 0 || i >= self.t.get_size() {
            return (skip_this_list, None);
        }

        for z in self.get_zone_list() {
            if let ZoneType::Unique = z.get_zone_type() {
                if self.skip_this[z].contains(&SkipType::Naked) {
                    continue;
                }
                let result = self.naked_number_zone(z, i);

                if result.is_some() {
                    return (skip_this_list, result);
                } else {
                    let skip_this_chk = SkipThis {
                        skip_type: SkipType::Naked,
                        skip_zone: (*z).clone(),
                    };
                    skip_this_list.push(skip_this_chk);
                }
            }
        }

        (skip_this_list, None)
    }

    fn naked_number_zone(&self, z: &Zone, i: u32) -> Option<SolverResult<'a>> {
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
                    solver_type: SolverType::Naked {
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
