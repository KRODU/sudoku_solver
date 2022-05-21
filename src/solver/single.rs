use hashbrown::{HashMap, HashSet};

use crate::{
    cell::Cell,
    zone::{Zone, ZoneType},
};

use super::{
    solver_history::{SolverResult, SolverResultDetail},
    solver_skip_result::{SolverResultSimple, SolverSkipResult},
    Solver,
};

impl<'a> Solver<'a> {
    pub fn single(&self) -> SolverSkipResult<'a> {
        let mut total_skip_list: Vec<Zone> = Vec::new();

        for z in self.get_zone_list() {
            if let ZoneType::Unique = z.get_zone_type() {
                if self.skip_this[z].contains(&SolverResultSimple::Single) {
                    continue;
                }

                for c in self.zone_iter(z) {
                    let b = c.chk.borrow();

                    // 노트가 확정된 경우 Zone을 순회하면서 해당 노트를 가진 cell이 있나 찾음
                    if let Some(final_num) = b.get_final_num() {
                        let mut effect_cells: HashMap<&Cell, Vec<u32>> = HashMap::new();

                        for c_comp in self.zone_iter(z) {
                            // 나 자신은 비교 대상에서 제외
                            if c_comp == c {
                                continue;
                            }

                            // 찾음
                            if c_comp.chk.borrow().get_chk(final_num) {
                                let v: Vec<u32> = vec![final_num];
                                effect_cells.insert(c_comp, v);
                            }
                        }

                        if effect_cells.is_empty() {
                            // 삭제할 cell을 찾지 못한 경우 skip으로 추가
                            total_skip_list.push((*z).clone());
                        } else {
                            // 하나 이상의 삭제할 노트를 가진 cell을 찾을 경우
                            let mut found_cells: HashSet<&'a Cell> = HashSet::with_capacity(1);
                            found_cells.insert(c);

                            let solver_result: Option<SolverResult<'a>> = Some(SolverResult {
                                solver_type: SolverResultDetail::Single {
                                    found_chk: final_num,
                                },
                                found_cells,
                                effect_cells,
                            });

                            return SolverSkipResult {
                                skip_type: SolverResultSimple::Single,
                                skip_zone: total_skip_list,
                                solver_result,
                            };
                        }
                    }
                }
            }
        }

        SolverSkipResult {
            skip_type: SolverResultSimple::Single,
            skip_zone: total_skip_list,
            solver_result: None,
        }
    }
}
