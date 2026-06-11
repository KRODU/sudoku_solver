use super::{
    Solver,
    solver_history::{SolverResult, SolverResultDetail},
    solver_simple::SolverSimple,
};
use crate::model::array_vector::ArrayVector;
use crate::{
    combinations::{Combination, TwoGroupCombination},
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
    fn find_naked_from_combination<'b>(
        &self,
        arr: &[&&'a Cell<N>],
        cells: &[&'a Cell<N>],
        read: &'b TableLockReadGuard<N>,
        result_list: &'b Mutex<Vec<SolverResult<'a, N>>>,
        is_break: &'b RelaxedBool,
        i: usize,
        i_u32: u32,
        arr_is_sorted: bool,
    ) -> bool {
        if is_break.get() {
            return false;
        }

        debug_assert_eq!(i, arr.len());
        let mut union_bit_flag = u64::MIN;
        for c in arr {
            let b = read.read_from_cell(**c);
            union_bit_flag |= b.bit_flag();

            if union_bit_flag.count_ones() > i_u32 {
                return false;
            }
        }

        let union_node_true_cnt = union_bit_flag.count_ones();
        if union_node_true_cnt != i_u32 {
            return false;
        }

        let mut effect_cells: Vec<(&Cell<N>, ArrayVector<MaxNum<N>, N>)> = Vec::new();
        for zone_cell in cells {
            let found_cell_contains_zone_cell = if arr_is_sorted {
                arr.binary_search(&zone_cell).is_ok()
            } else {
                arr.iter().any(|c| **c == *zone_cell)
            };

            if found_cell_contains_zone_cell {
                continue;
            }

            let b = read.read_from_cell(zone_cell);
            let mut inter: ArrayVector<MaxNum<N>, N> = ArrayVector::new();
            for &true_note in b.get_true_list() {
                if union_bit_flag & (1 << true_note.get_value()) != 0 {
                    inter.push(true_note);
                }
            }

            if !inter.is_empty() {
                is_break.set(true);
                if effect_cells.is_empty() {
                    effect_cells.reserve_exact(N);
                }
                effect_cells.push((zone_cell, inter));
            }
        }

        if effect_cells.is_empty() {
            return false;
        }

        let mut found_chks: ArrayVector<MaxNum<N>, N> = ArrayVector::new();
        for n in MaxNum::<N>::iter() {
            if union_bit_flag & (1 << n.get_value()) != 0 {
                found_chks.push(n);
            }
        }
        debug_assert_eq!(found_chks.len(), union_node_true_cnt as usize);

        let mut found_cell: Vec<&Cell<N>> = arr.iter().map(|c| **c).collect();
        found_cell.sort_unstable();

        let result = SolverResult {
            solver_type: SolverResultDetail::Naked {
                found_chks,
                found_cell,
            },
            effect_cells,
        };

        let mut result_list_lock = result_list.lock().unwrap();
        result_list_lock.push(result);
        true
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

        let mut non_final_cells: Vec<&Cell<N>> = Vec::with_capacity(cells.len());
        non_final_cells.extend(
            cells
                .iter()
                .copied()
                .filter(|c| read.read_from_cell(c).true_cnt() > 1),
        );
        let non_final_cells = &non_final_cells;

        let find_some = AtomicBool::new(false);
        let find_some = &find_some;
        let full_scan_required = self.zone_cache.naked_full_scan_required(zone);
        let last_changed_cells = self.zone_cache.last_changed_cells(zone);

        rayon::scope_fifo(|s| {
            for i in 2..N / 2 {
                let i_u32 = i as u32;
                s.spawn_fifo(move |_| {
                    let mut comp_cell_target: Vec<&Cell<N>> =
                        Vec::with_capacity(non_final_cells.len());
                    comp_cell_target.extend(
                        non_final_cells
                            .iter()
                            .copied()
                            .filter(|c| read.read_from_cell(c).true_cnt() <= i),
                    );

                    if full_scan_required {
                        let mut comb_iter = Combination::new(&comp_cell_target, i);

                        while let Some(arr) = comb_iter.next_comb() {
                            if self.find_naked_from_combination(
                                arr,
                                cells,
                                read,
                                result_list,
                                is_break,
                                i,
                                i_u32,
                                true,
                            ) {
                                find_some.store(true, Ordering::Relaxed);
                                return;
                            }

                            if is_break.get() {
                                return;
                            }
                        }
                    } else {
                        let mut mandatory_group: Vec<&Cell<N>> = Vec::with_capacity(
                            last_changed_cells.len().min(comp_cell_target.len()),
                        );
                        let mut optional_group: Vec<&Cell<N>> =
                            Vec::with_capacity(comp_cell_target.len());

                        for &cell in &comp_cell_target {
                            if last_changed_cells.contains(&cell) {
                                mandatory_group.push(cell);
                            } else {
                                optional_group.push(cell);
                            }
                        }

                        if mandatory_group.is_empty() {
                            return;
                        }

                        let mut comb_iter =
                            TwoGroupCombination::new(&mandatory_group, &optional_group, i);

                        while let Some(arr) = comb_iter.next_comb() {
                            if self.find_naked_from_combination(
                                arr,
                                cells,
                                read,
                                result_list,
                                is_break,
                                i,
                                i_u32,
                                false,
                            ) {
                                find_some.store(true, Ordering::Relaxed);
                                return;
                            }

                            if is_break.get() {
                                return;
                            }
                        }
                    }
                });
            }
        });

        if !find_some.load(Ordering::Relaxed) && !is_break.get() {
            self.zone_cache
                .checked_zone_set_bool_true(*zone, SolverSimple::Naked);
            if full_scan_required {
                self.zone_cache.naked_full_scan_required_set_false(*zone);
            }
        }
    }
}
