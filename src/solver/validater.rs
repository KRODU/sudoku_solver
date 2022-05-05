use hashbrown::HashSet;

use crate::{cell::Cell, zone::ZoneType};

use super::Solver;

impl<'a> Solver<'a> {
    /// 현재 스도쿠 퍼즐의 유효성 검사하여 에러셀을 반환.
    /// 에러셀이 없다면 None
    pub fn find_error_cell(&self) -> Option<&Cell> {
        for cell in self.t.get_cell().values() {
            if cell.chk.borrow().get_true_cnt() == 0 {
                return Some(cell);
            }
        }

        for (zone, vec) in &self.ref_cache {
            match zone.get_zone_type() {
                ZoneType::Unique => {
                    let chk_result: Option<&Cell> = Solver::validater_unique_chk(vec);
                    if let Some(_) = chk_result {
                        return chk_result;
                    }
                }
                ZoneType::Sum { sum } => {
                    let chk_result: Option<&Cell> = Solver::validater_sum_chk(*sum, vec);
                    if let Some(_) = chk_result {
                        return chk_result;
                    }
                }
            }
        }

        None
    }

    /// 파라미터의 모든 Cell이 고유값을 가지고 있는지 확인
    fn validater_unique_chk(vec: &Vec<&'a Cell>) -> Option<&'a Cell> {
        let mut unique_chk_map: HashSet<usize> = HashSet::with_capacity(vec.len());

        for c in vec {
            let final_num: Option<usize> = c.chk.borrow().get_final_num();
            if let Some(num) = final_num {
                if !unique_chk_map.insert(num) {
                    return Some(c);
                }
            }
        }
        None
    }

    /// 파라미터의 모든 Cell의 합이 일치하는지 여부 확인
    fn validater_sum_chk(sum: usize, vec: &Vec<&'a Cell>) -> Option<&'a Cell> {
        let mut total_sum: usize = 0;

        for c in vec {
            let final_num: Option<usize> = c.chk.borrow().get_final_num();
            if let Some(num) = final_num {
                total_sum += num;
            }
        }

        if total_sum > sum {
            return Some(vec[0]);
        }
        None
    }
}
