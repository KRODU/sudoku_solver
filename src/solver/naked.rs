use std::cell::Ref;

use hashbrown::{HashMap, HashSet};

use crate::{
    cell::Cell,
    num_check::NumCheck,
    util::combinations,
    zone::{Zone, ZoneType},
};

use super::{
    solver_history::{SolverResult, SolverType},
    Solver,
};

impl<'a> Solver<'a> {
    pub fn naked(&self) {
        for i in 1..self.t.get_size() {
            self.naked_number(i);
        }
    }

    pub fn naked_number(&self, i: usize) -> Option<SolverResult> {
        // i의 값이 유효하지 않은 경우 None
        if i == 0 || i >= self.t.get_size() {
            return None;
        }

        for z in self.get_zone_list() {
            if let ZoneType::Unique = z.get_zone_type() {
                let result = self.naked_number_zone(i, z);
                if result.is_some() {
                    return result;
                }
            }
        }
        None
    }

    fn naked_number_zone(&self, i: usize, z: &&Zone) -> Option<SolverResult> {
        let borrow_map = self.fill_and_get_borrow_map();
        let mut chk: Vec<&Cell> = Vec::with_capacity(self.t.get_size());

        for c in self.zone_iter(z) {
            let borrow: &Ref<NumCheck> = borrow_map.get(c).unwrap();
            if borrow.get_true_cnt() == i {
                chk.push(c);
            }
        }

        let com_result = combinations(&chk, i, |comb| {
            let first_node = borrow_map.get(comb[0]).unwrap();
            comb.iter()
                .all(|c| borrow_map.get(*c).unwrap().is_same_note(first_node))
        });

        let mut effect_cells: HashMap<&Cell, Vec<usize>> = HashMap::new();

        for r in com_result {
            let naked_value: &Ref<NumCheck> = borrow_map.get(r[0]).unwrap();
            // zone을 순회하며 삭제할 노트가 있는지 찾음
            for zone_cell in self.zone_iter(z) {
                // 순회 대상에서 자기 자신은 제외
                if r.contains(&zone_cell) {
                    continue;
                }

                let b = borrow_map.get(zone_cell).unwrap();
                let union: Vec<usize> = b.union_note(naked_value);

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
                return Some(SolverResult::new(
                    SolverType::Naked {
                        found_chks: naked_value.clone_chk_list(),
                    },
                    found_cells,
                    effect_cells,
                ));
            }
        }

        None
    }
}
