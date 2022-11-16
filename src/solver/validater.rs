use super::Solver;
use crate::model::{cell::Cell, ref_zone::RefZone, zone::ZoneType};

impl<'a> Solver<'a> {
    /// 현재 스도쿠 퍼즐의 유효성 검사하여 에러셀을 반환.
    /// 에러셀이 없다면 None
    pub fn find_error_cell(&self, zone_ref_with_read: &Vec<RefZone<'a>>) -> Option<&Cell> {
        let mut unique_chk_map: Vec<bool> = vec![false; self.t.size];

        for zone_ref in zone_ref_with_read {
            match zone_ref.zone.get_zone_type() {
                // 파라미터의 모든 Cell이 고유값을 가지고 있는지 확인
                ZoneType::Unique => {
                    unique_chk_map.iter_mut().for_each(|b| *b = false);

                    for c in &zone_ref.cells {
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
                    let mut total_sum = 0;

                    for c in &zone_ref.cells {
                        let chk_borr = &c.read;
                        let final_num = chk_borr.get_final_num();
                        if let Some(num) = final_num {
                            total_sum += num;
                        }
                    }

                    if total_sum > *sum {
                        return Some(zone_ref.cells[0].cell);
                    }
                }
            }
        }

        None
    }
}
