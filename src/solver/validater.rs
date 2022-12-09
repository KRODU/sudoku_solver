use super::{solver_simple::SolverSimple, Solver};
use crate::model::{cell::Cell, ref_zone::RefZone, zone::ZoneType};

impl<'a, const N: usize> Solver<'a, N> {
    /// 현재 스도쿠 퍼즐의 유효성 검사하여 에러셀을 반환.
    /// 에러셀이 없다면 None
    pub fn find_error_cell(&self, zone_ref_with_read: &Vec<RefZone<'a, N>>) -> Option<&Cell<N>> {
        let mut unique_chk_map = [false; N];

        for ref_zone in zone_ref_with_read {
            if self.checked_zone_get_bool(ref_zone.zone, SolverSimple::Validate) {
                continue;
            }

            match ref_zone.zone.get_zone_type() {
                // 파라미터의 모든 Cell이 고유값을 가지고 있는지 확인
                ZoneType::Unique => {
                    unique_chk_map.iter_mut().for_each(|b| *b = false);

                    for c in &ref_zone.cells {
                        let chk_borr = &c.read;
                        if let Some(num) = chk_borr.get_final_num() {
                            let num = num - 1;
                            if unique_chk_map[num] {
                                return Some(c.cell);
                            }
                            unique_chk_map[num] = true;
                        }

                        // 가능한 숫자가 하나도 없는 cell이 존재하는지 확인
                        if chk_borr.get_true_cnt() == 0 {
                            return Some(c.cell);
                        }
                    }
                }
                // 파라미터의 모든 Cell의 합이 일치하는지 여부 확인
                ZoneType::Sum { sum } => {
                    let mut cell_total = 0;
                    let mut all_final = true;

                    for c in &ref_zone.cells {
                        let read = &c.read;

                        if let Some(num) = read.get_final_num() {
                            cell_total += num;
                        } else {
                            all_final = false;

                            // cell의 값이 확정되지 않은 경우 사용 가능한 노트 중 가장 작은 값을 total_sum에 더함
                            let Some(minimum) = read.get_minimum_chk() else {
                                return Some(c.cell);
                            };

                            cell_total += minimum;
                        }
                    }

                    // cell이 모두 확정된 경우 cell_total과 sum은 같아야 함.
                    if all_final {
                        if cell_total != *sum {
                            return Some(ref_zone.cells[0].cell);
                        }
                        // 미확정 cell이 있는 경우 cell_total과 sum은 다를 수 있음.
                        // 이 경우에도 cell_total이 sum을 넘어서면 안 됨
                    } else if cell_total > *sum {
                        return Some(ref_zone.cells[0].cell);
                    }
                }
            }

            self.checked_zone_set_bool_true(ref_zone.zone, SolverSimple::Validate);
        }

        None
    }
}
