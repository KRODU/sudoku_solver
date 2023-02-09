use rayon::ScopeFifo;

use super::{
    solver_history::{SolverResult, SolverResultDetail},
    solver_simple::SolverSimple,
    Solver,
};
use crate::model::{
    array_note::ArrayNote,
    cell::Cell,
    max_num::MaxNum,
    table_lock::TableLockReadGuard,
    zone::{Zone, ZoneType},
};
use crate::{combinations::combinations, model::array_vector::ArrayVector};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};

impl<'a, const N: usize> Solver<'a, N> {
    pub fn naked<'scope, 'b: 'scope>(
        &'b self,
        read: &'b TableLockReadGuard<N>,
        s: &ScopeFifo<'scope>,
        result_list: &'b Mutex<Vec<SolverResult<'a, N>>>,
        is_break: &'b AtomicBool,
    ) {
        for (zone, cells) in &self.ordered_zone {
            let ZoneType::Unique = zone.get_zone_type() else { continue; };

            if self.checked_zone_get_bool(zone, SolverSimple::Naked) {
                continue;
            }

            s.spawn_fifo(|_| {
                if let Some(result) = self.naked_number_zone(zone, cells, read, is_break) {
                    let mut result_list_lock = result_list.lock().unwrap();
                    result_list_lock.push(result);
                }
            });
        }
    }

    fn naked_number_zone(
        &self,
        zone: &'a Zone,
        cells: &Vec<&'a Cell<N>>,
        read: &TableLockReadGuard<N>,
        is_break: &AtomicBool,
    ) -> Option<SolverResult<'a, N>> {
        let mut ret: Option<SolverResult<'a, N>> = None;
        let mut comp_cell_target: Vec<&Cell<N>> = Vec::with_capacity(cells.len());
        let mut chk_all = true;

        for i in 2..N / 2 {
            if is_break.load(Ordering::Relaxed) {
                chk_all = false;
                break;
            }

            comp_cell_target.clear();
            // 검증대상 cell 필터링 후 처리. 이렇게 하면 처리 시간을 많이 줄일 수 있음.
            comp_cell_target.extend(cells.iter().filter(|c| {
                let true_cnt = read.read_from_cell(c).get_true_cnt();
                true_cnt <= i && true_cnt > 1
            }));

            combinations(&comp_cell_target, i, |arr| {
                debug_assert_eq!(i, arr.len());
                let mut union_node = ArrayNote::new([false; N]);
                let mut union_node_true_cnt = 0;
                for c in arr {
                    let b = read.read_from_cell(c);

                    for &true_note in b.get_true_list() {
                        if union_node[true_note] {
                            continue;
                        } else {
                            union_node[true_note] = true;
                            union_node_true_cnt += 1;

                            if union_node_true_cnt > i {
                                return true;
                            }
                        }
                    }
                }

                if union_node_true_cnt != i {
                    return true;
                }

                let mut effect_cells: Vec<(&Cell<N>, ArrayVector<MaxNum<N>, N>)> = Vec::new();
                // zone을 순회하며 삭제할 노트가 있는지 찾음
                for zone_cell in cells {
                    // 순회 대상에서 자기 자신은 제외
                    // combinations 함수는 입력 배열이 정렬되어있을 경우 출력 배열 또한 정렬되어 있음
                    // cells -> comp_cell_target -> combinations 함수로 데이터가 흘러가며 cells가 정렬되어있으니 combinations또한 정렬됨
                    if arr.binary_search(&zone_cell).is_ok() {
                        continue;
                    }
                    debug_assert!(!arr.contains(&zone_cell)); // 실수로 버그를 도입할 수 있으니 이중체크..

                    let b = read.read_from_cell(zone_cell);
                    let mut inter: ArrayVector<MaxNum<N>, N> = ArrayVector::new();
                    for &true_note in b.get_true_list() {
                        if union_node[true_note] {
                            inter.push(true_note);
                        }
                    }

                    // 제거할 노트를 발견한 경우
                    if !inter.is_empty() {
                        effect_cells.push((zone_cell, inter));
                    }
                }

                // effect_cells에 값이 존재하는 경우 제거한 노트를 발견한 것임.
                if !effect_cells.is_empty() {
                    let union_node_array_vec = union_node.bool_array_note_to_array_vec();
                    debug_assert_eq!(union_node_array_vec.len(), union_node_true_cnt);
                    ret = Some(SolverResult {
                        solver_type: SolverResultDetail::Naked {
                            found_chks: union_node_array_vec,
                            found_cell: arr.iter().map(|c| **c).collect(),
                        },
                        effect_cells,
                    });

                    return false;
                }

                true
            });

            if ret.is_some() {
                is_break.store(true, Ordering::Relaxed);
                return ret;
            }
        }

        // is_break가 true여서 중간에 break된 경우 checked_zone을 설정하면 안 됨..
        if chk_all {
            self.checked_zone_set_bool_true(zone, SolverSimple::Naked);
        }

        None
    }
}
