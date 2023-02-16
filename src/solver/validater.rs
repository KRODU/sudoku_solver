use super::{solver_simple::SolverSimple, Solver};
use crate::model::{
    array_note::ArrayNote, cell::Cell, non_atomic_bool::NonAtomicBool,
    table_lock::TableLockReadGuard, zone::ZoneType,
};

impl<'a, const N: usize> Solver<'a, N> {
    pub fn validater(&self) -> Option<&Cell<N>> {
        let read = self.table.read_lock();
        let is_break = NonAtomicBool::new(false);

        self.validater_inner(&read, &is_break)
    }

    /// 현재 스도쿠 퍼즐의 유효성 검사하여 에러셀을 반환.
    /// 에러셀이 없다면 None
    pub(crate) fn validater_inner(
        &self,
        read: &TableLockReadGuard<N>,
        is_break: &NonAtomicBool,
    ) -> Option<&Cell<N>> {
        let mut unique_chk_arr: ArrayNote<bool, N>;

        for (zone, cells) in &self.ordered_zone {
            if is_break.get() {
                break;
            }
            if self.checked_zone_get_bool(zone, SolverSimple::Validate) {
                continue;
            }

            match zone.get_zone_type() {
                // 파라미터의 모든 Cell이 고유값을 가지고 있는지 확인
                ZoneType::Unique => {
                    unique_chk_arr = ArrayNote::new([false; N]);

                    for c in cells {
                        let chk_borr = read.read_from_cell(c);
                        if let Some(num) = chk_borr.get_final_num() {
                            if unique_chk_arr[num] {
                                return Some(c);
                            }
                            unique_chk_arr[num] = true;
                        }

                        // 가능한 숫자가 하나도 없는 cell이 존재하는지 확인
                        if chk_borr.get_true_cnt() == 0 {
                            return Some(c);
                        }
                    }
                }
                // 파라미터의 모든 Cell의 합이 일치하는지 여부 확인
                ZoneType::Sum { sum } => {
                    let mut cell_total = 0;
                    let mut all_final = true;

                    for c in cells {
                        let read = read.read_from_cell(c);

                        if let Some(num) = read.get_final_num() {
                            cell_total += num.get_value() + 1; // 값은 0부터 시작하므로 1을 더해야 함.
                        } else {
                            all_final = false;

                            // cell의 값이 확정되지 않은 경우 사용 가능한 노트 중 가장 작은 값을 total_sum에 더함
                            let Some(minimum) = read.get_minimum_chk() else {
                                return Some(c);
                            };

                            cell_total += minimum.get_value() + 1; // 값은 0부터 시작하므로 1을 더해야 함.
                        }
                    }

                    // cell이 모두 확정된 경우 cell_total과 sum은 같아야 함.
                    if all_final {
                        if cell_total != *sum {
                            return Some(cells[0]);
                        }
                        // 미확정 cell이 있는 경우 cell_total과 sum은 다를 수 있음.
                        // 이 경우에도 cell_total이 sum을 넘어서면 안 됨
                    } else if cell_total > *sum {
                        return Some(cells[0]);
                    }
                }
            }

            self.checked_zone_set_bool_true(*zone, SolverSimple::Validate);
        }

        None
    }
}
