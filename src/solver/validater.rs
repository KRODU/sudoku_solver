use hashbrown::HashSet;

use crate::{cell::Cell, zone::ZoneType};

use super::Solver;

impl<'a> Solver<'a> {
    /// 현재 스도쿠 퍼즐의 유효성 검사하여 에러셀을 반환.
    /// 에러셀이 없다면 None
    pub fn find_error_cell(&self) -> Option<&Cell> {
        // 가능한 숫자가 하나도 없는 cell이 존재하는지 확인
        for cell in self.t.into_iter() {
            let chk_borr = cell.chk.borrow();
            if chk_borr.get_true_cnt() == 0 {
                return Some(cell);
            }
        }

        for (zone, vec) in &self.ref_cache {
            match zone.get_zone_type() {
                // 파라미터의 모든 Cell이 고유값을 가지고 있는지 확인
                ZoneType::Unique => {
                    let mut unique_chk_map = HashSet::with_capacity(vec.len());

                    for c in vec {
                        let chk_borr = c.chk.borrow();
                        let final_num = chk_borr.get_final_num();
                        if let Some(num) = final_num {
                            if !unique_chk_map.insert(num) {
                                return Some(c);
                            }
                        }
                    }
                }
                // 파라미터의 모든 Cell의 합이 일치하는지 여부 확인
                ZoneType::Sum { sum } => {
                    let mut total_sum = 0;

                    for c in vec {
                        let chk_borr = c.chk.borrow();
                        let final_num = chk_borr.get_final_num();
                        if let Some(num) = final_num {
                            total_sum += num;
                        }
                    }

                    if total_sum > *sum {
                        return Some(vec[0]);
                    }
                }
            }
        }

        None
    }
}
