use super::{
    Solver,
    solver_history::{SolverResult, SolverResultDetail},
    solver_simple::SolverSimple,
};
use crate::model::array_vector::ArrayVector;
use crate::{
    combinations::Combination,
    model::{
        cell::Cell,
        max_num::MaxNum,
        relaxed_bool::RelaxedBool,
        table_lock::TableLockReadGuard,
        zone::{Zone, ZoneType},
    },
};
use rayon::ScopeFifo;
use std::sync::{
    Mutex,
    atomic::{AtomicBool, Ordering},
};

impl<'a, const N: usize> Solver<'a, N> {
    pub fn naked<'scope, 'b: 'scope>(
        &'b self,
        read: &'b TableLockReadGuard<N>,
        s: &ScopeFifo<'scope>,
        result_list: &'b Mutex<Vec<SolverResult<'a, N>>>,
        is_break: &'b RelaxedBool,
    ) {
        for (zone, cells) in self.zone_cache.zone() {
            let ZoneType::Unique = zone.get_zone_type() else {
                continue;
            };

            if self
                .zone_cache
                .checked_zone_get_bool(zone, SolverSimple::Naked)
            {
                continue;
            }

            s.spawn_fifo(|_| {
                self.naked_number_zone(zone, cells, read, result_list, is_break);
            });
        }
    }

    #[inline]
    fn naked_number_zone<'b>(
        &self,
        zone: &Zone,
        cells: &'b Vec<&'a Cell<N>>,
        read: &'b TableLockReadGuard<N>,
        result_list: &'b Mutex<Vec<SolverResult<'a, N>>>,
        is_break: &'b RelaxedBool,
    ) {
        if is_break.get() {
            return;
        }
        let non_final_cells = {
            let mut non_final_cells: Vec<&Cell<N>> = Vec::with_capacity(cells.len());
            non_final_cells.extend(
                cells
                    .iter()
                    .copied()
                    .filter(|c| read.read_from_cell(c).true_cnt() > 1),
            );
            non_final_cells
        };

        let non_final_cells = &non_final_cells;

        let find_some = AtomicBool::new(false);
        let find_some = &find_some;

        rayon::scope_fifo(|s| {
            for i in 2..N / 2 {
                let i_u32 = i as u32;
                s.spawn_fifo(move |_| {
                    let mut comp_cell_target: Vec<&Cell<N>> =
                        Vec::with_capacity(non_final_cells.len());
                    // 검증대상 cell 필터링 후 처리. 이렇게 하면 처리 시간을 많이 줄일 수 있음.
                    comp_cell_target.extend(
                        non_final_cells
                            .iter()
                            .filter(|c| read.read_from_cell(c).true_cnt() <= i),
                    );

                    let mut comb_iter = Combination::new(&comp_cell_target, i);

                    'comb_loop: while let Some(arr) = comb_iter.next_comb() {
                        if is_break.get() {
                            return;
                        }

                        debug_assert_eq!(i, arr.len());
                        let mut union_bit_flag = u64::MIN;
                        for c in arr {
                            let b = read.read_from_cell(c);
                            union_bit_flag |= b.bit_flag();

                            if union_bit_flag.count_ones() > i_u32 {
                                continue 'comb_loop;
                            }
                        }

                        let union_node_true_cnt = union_bit_flag.count_ones();

                        if union_node_true_cnt != i_u32 {
                            continue 'comb_loop;
                        }

                        let mut effect_cells: Vec<(&Cell<N>, ArrayVector<MaxNum<N>, N>)> =
                            Vec::new();
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
                                if union_bit_flag & (1 << true_note.get_value()) != 0 {
                                    inter.push(true_note);
                                }
                            }

                            // 제거할 노트를 발견한 경우
                            if !inter.is_empty() {
                                is_break.set(true);
                                if effect_cells.is_empty() {
                                    effect_cells.reserve_exact(N);
                                }
                                effect_cells.push((zone_cell, inter));
                            }
                        }

                        // effect_cells에 값이 존재하는 경우 제거한 노트를 발견한 것임.
                        if !effect_cells.is_empty() {
                            let mut found_chks: ArrayVector<MaxNum<N>, N> = ArrayVector::new();
                            for n in MaxNum::<N>::iter() {
                                if union_bit_flag & (1 << n.get_value()) != 0 {
                                    found_chks.push(n);
                                }
                            }
                            debug_assert_eq!(found_chks.len(), union_node_true_cnt as usize);
                            let result = SolverResult {
                                solver_type: SolverResultDetail::Naked {
                                    found_chks,
                                    found_cell: arr.iter().map(|c| **c).collect(),
                                },
                                effect_cells,
                            };

                            let mut result_list_lock = result_list.lock().unwrap();
                            result_list_lock.push(result);
                            find_some.store(true, Ordering::Relaxed);
                            return;
                        }
                    }
                });
            }
        });

        // 아무것도 찾지 못한 경우에만 zone_cache 업데이트
        if !find_some.load(Ordering::Relaxed) {
            self.zone_cache
                .checked_zone_set_bool_true(*zone, SolverSimple::Naked);
        }
    }
}
