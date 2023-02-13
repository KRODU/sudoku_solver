use super::{
    solver_history::{SolverResult, SolverResultDetail},
    solver_simple::SolverSimple,
    Solver,
};
use crate::model::{
    array_vector::ArrayVector, cell::Cell, max_num::MaxNum, non_atomic_bool::NonAtomicBool,
    table_lock::TableLockReadGuard, zone::ZoneType,
};
use crate::num_check::NumCheck;
use rayon::ScopeFifo;
use std::sync::Mutex;

impl<'a, const N: usize> Solver<'a, N> {
    pub fn box_line_reduction<'scope, 'b: 'scope>(
        &'b self,
        read: &'b TableLockReadGuard<N>,
        s: &ScopeFifo<'scope>,
        result_list: &'b Mutex<Vec<SolverResult<'a, N>>>,
        is_break: &'b NonAtomicBool,
    ) {
        for (z1, z1_cells) in &self.ordered_zone {
            let ZoneType::Unique = z1.get_zone_type() else { continue; };

            let Some(connect_zone) = self.connect_zone.get(z1) else {
                continue;
            };

            s.spawn_fifo(move |_| {
                if is_break.get() {
                    return;
                }

                if self.checked_zone_get_bool(z1, SolverSimple::BoxLineReduction) {
                    return;
                }

                let current_zone_union_note =
                    z1_cells
                        .iter()
                        .fold(NumCheck::new_with_false(), |mut h, c| {
                            let cell_read = read.read_from_cell(c);
                            if cell_read.get_true_cnt() > 1 {
                                cell_read.union_note_num_check(&mut h);
                            }
                            h
                        });

                // Box Line Reduction 검색 알고리즘:
                // 1. 서로 연결되어 있는 두 Zone을 찾는다. (2개의 Zone이 있을 때 두 Zone에 모두 겹쳐있는 Cell이 하나 이상 있을 경우, 두 Zone은 서로 연결되었다고 한다.)
                // 2. 서로 연결된 두 Zone을 z1, z2라 할 경우, 특정 노트 N은 z1에게 있어서 z2와 겹쳐지는 영역 내에서만 존재하는 노트일 수 있다.
                // 3. 이때 z1와 z2가 겹쳐지는 영역을 제외한 나머지 z2의 영역에서 노트 N이 발견되는 경우, 해당 노트는 제거할 수 있다.
                for &z2 in connect_zone {
                    let ZoneType::Unique = z2.get_zone_type() else { continue; };

                    let Some(z2_cells) = self.hashed_zone.get(&z2) else {
                        unreachable!();
                    };

                    for &note in &current_zone_union_note {
                        let target_this_note = !z1_cells.iter().any(|c| {
                            if c.zone_set.contains(&z2) {
                                return false;
                            }

                            read.read_from_cell(c).get_chk(note)
                        });

                        if target_this_note {
                            let mut effect_cells: Vec<(&'a Cell<N>, ArrayVector<MaxNum<N>, N>)> =
                                Vec::new();

                            for z2_cell in z2_cells {
                                if z2_cell.zone_set.contains(z1) {
                                    continue;
                                }

                                if read.read_from_cell(z2_cell).get_chk(note) {
                                    let mut note_vec = ArrayVector::new();
                                    note_vec.push(note);
                                    effect_cells.push((z2_cell, note_vec));
                                }
                            }

                            if !effect_cells.is_empty() {
                                is_break.set(true);
                                let mut result_lock = result_list.lock().unwrap();
                                result_lock.push(SolverResult {
                                    solver_type: SolverResultDetail::BoxLineReduction {
                                        found_chk: note,
                                    },
                                    effect_cells,
                                });
                            }
                        }
                    }
                }

                self.checked_zone_set_bool_true(*z1, SolverSimple::BoxLineReduction);
            });
        }
    }
}
