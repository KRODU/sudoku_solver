use super::{
    solver_history::{SolverResult, SolverResultDetail},
    solver_simple::SolverSimple,
    Solver,
};
use crate::model::{
    cell::Cell, cell_with_read::CellWithRead, max_num::MaxNum, ref_zone::RefZone, zone::ZoneType,
};
use crate::num_check::NumCheck;
use crate::{combinations::combinations, model::array_vector::ArrayVector};
use std::sync::Mutex;

impl<'a, const N: usize> Solver<'a, N> {
    pub fn naked(&self, zone_ref_with_read: &Vec<RefZone<'a, N>>) -> Vec<SolverResult<'a, N>> {
        let mut pool = self.pool.lock().unwrap();
        let result_list: Mutex<Vec<SolverResult<N>>> = Mutex::new(Vec::new());

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

    fn naked_number_zone(&self, ref_zone: &RefZone<'a, N>) -> Option<SolverResult<'a, N>> {
        let mut union_node: NumCheck<N> = NumCheck::new_with_false();
        let mut ret: Option<SolverResult<'a, N>> = None;
        let cell_list = &ref_zone.cells;
        let mut comp_cell_target: Vec<&CellWithRead<'a, N>> = Vec::with_capacity(cell_list.len());

        for i in 2..N / 2 {
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

                union_node.set_all_false();
                for c in arr {
                    let b = &c.read;

                    b.union_note(&mut union_node);
                    if union_node.get_true_cnt() > i {
                        return true;
                    }
                }

                if union_node.get_true_cnt() != i {
                    return true;
                }

                let mut effect_cells: Vec<(&Cell<N>, ArrayVector<MaxNum<N>, N>)> = Vec::new();
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
                        let note: ArrayVector<MaxNum<N>, N> = inter.into_iter().collect();
                        effect_cells.push((zone_cell.cell, note));
                    }
                }

                // effect_cells에 값이 존재하는 경우 제거한 노트를 발견한 것임.
                if !effect_cells.is_empty() {
                    let found_chks: ArrayVector<MaxNum<N>, N> =
                        union_node.iter().copied().collect();
                    ret = Some(SolverResult {
                        solver_type: SolverResultDetail::Naked { found_chks },
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
