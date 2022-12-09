use super::{
    solver_history::{SolverResult, SolverResultDetail},
    solver_simple::SolverSimple,
    Solver,
};
use crate::combinations::combinations;
use crate::model::{cell::Cell, cell_with_read::CellWithRead, ref_zone::RefZone, zone::ZoneType};
use hashbrown::HashSet;
use std::sync::Mutex;

impl<'a> Solver<'a> {
    pub fn naked(&self, zone_ref_with_read: &Vec<RefZone<'a>>) -> Vec<SolverResult<'a>> {
        let mut pool = self.pool.lock().unwrap();
        let result_list: Mutex<Vec<SolverResult>> = Mutex::new(Vec::new());

        pool.scoped(|s| {
            for zone_ref in zone_ref_with_read {
                let ZoneType::Unique = zone_ref.zone.get_zone_type() else { continue; };

                if self.checked_zone_get_bool(zone_ref.zone, SolverSimple::Naked) {
                    continue;
                }

                let result_list_borrow = &result_list;

                s.execute(move || {
                    if let Some(result) = self.naked_number_zone(zone_ref) {
                        let mut result_list_lock = result_list_borrow.lock().unwrap();
                        result_list_lock.push(result);
                    }
                });
            }
        });

        result_list.into_inner().unwrap()
    }

    fn naked_number_zone(&self, ref_zone: &RefZone<'a>) -> Option<SolverResult<'a>> {
        let mut union_node: HashSet<usize> = HashSet::new();
        let mut ret: Option<SolverResult<'a>> = None;
        let cell_list = &ref_zone.cells;
        let mut comp_cell_target: Vec<&CellWithRead<'a>> = Vec::with_capacity(cell_list.len());

        for i in 2..self.t.size / 2 {
            comp_cell_target.clear();
            // 검증대상 cell 필터링 후 처리. 이렇게 하면 처리 시간을 많이 줄일 수 있음.
            comp_cell_target.extend(cell_list.iter().filter(|c| {
                let true_cnt = c.read.get_true_cnt();
                true_cnt <= i && true_cnt > 1
            }));

            combinations(&comp_cell_target, i, |arr| {
                if !arr.iter().any(|c| self.changed_cell.contains(c.cell)) {
                    return true;
                }

                union_node.clear();
                for c in arr {
                    let b = &c.read;

                    b.union_note(&mut union_node);
                    if union_node.len() > i {
                        return true;
                    }
                }

                if union_node.len() != i {
                    return true;
                }

                let mut effect_cells: Vec<(&Cell, Vec<usize>)> = Vec::new();
                // zone을 순회하며 삭제할 노트가 있는지 찾음
                for zone_cell in cell_list {
                    // 순회 대상에서 자기 자신은 제외
                    if arr.contains(&&zone_cell) {
                        continue;
                    }

                    let b = &zone_cell.read;
                    let inter = b.intersection_note(&union_node);

                    // 제거할 노트를 발견한 경우
                    if !inter.is_empty() {
                        effect_cells.push((zone_cell.cell, inter.into_iter().collect()));
                    }
                }

                // effect_cells에 값이 존재하는 경우 제거한 노트를 발견한 것임.
                if !effect_cells.is_empty() {
                    ret = Some(SolverResult {
                        solver_type: SolverResultDetail::Naked {
                            found_chks: union_node.iter().copied().collect(),
                        },
                        effect_cells,
                    });

                    return false;
                }

                true
            });

            if ret.is_some() {
                return ret;
            }
        }

        self.checked_zone_set_bool_true(ref_zone.zone, SolverSimple::Naked);
        None
    }
}
