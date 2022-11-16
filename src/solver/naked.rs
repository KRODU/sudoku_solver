use super::{
    solver_history::{SolverResult, SolverResultDetail},
    Solver,
};
use crate::{
    combinations::combinations,
    model::{cell::Cell, cell_with_read::CellWithRead, ref_zone::RefZone, zone::ZoneType},
};
use hashbrown::{HashMap, HashSet};

impl<'a> Solver<'a> {
    pub fn naked(&self, zone_ref_with_read: &Vec<RefZone<'a>>) -> Vec<SolverResult<'a>> {
        std::thread::scope(|s| {
            let mut join_handle_list = Vec::new();

            for zone_ref in zone_ref_with_read {
                if !zone_ref.changed {
                    continue;
                }

                let ZoneType::Unique = zone_ref.zone.get_zone_type() else { continue; };

                let join_handle = s.spawn(move || self.naked_number_zone(&zone_ref.cells));
                join_handle_list.push(join_handle);
            }

            let mut result_list = Vec::new();
            for join_handle in join_handle_list {
                if let Some(result) = join_handle.join().unwrap() {
                    result_list.push(result);
                }
            }

            result_list
        })
    }

    fn naked_number_zone(&self, cell_list: &Vec<CellWithRead<'a>>) -> Option<SolverResult<'a>> {
        let mut union_node: HashSet<usize> = HashSet::new();
        let mut ret: Option<SolverResult<'a>> = None;
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

                let mut effect_cells: HashMap<&Cell, HashSet<usize>> = HashMap::new();
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
                        effect_cells.insert(zone_cell.cell, inter);
                    }
                }

                // effect_cells에 값이 존재하는 경우 제거한 노트를 발견한 것임.
                if !effect_cells.is_empty() {
                    let mut found_cells: HashSet<&'a Cell> = HashSet::with_capacity(arr.len());
                    for effect_cell in arr {
                        found_cells.insert(effect_cell.cell);
                    }
                    ret = Some(SolverResult {
                        solver_type: SolverResultDetail::Naked {
                            found_chks: union_node.clone(),
                        },
                        found_cells,
                        effect_cells,
                    });
                }

                false
            });

            if ret.is_some() {
                return ret;
            }
        }

        ret
    }
}
