use crate::cell::Cell;

use super::Solver;

pub struct NakedResult {
    pub naked_number: usize,
}

impl Solver<'_> {
    pub fn naked(&self) {
        for i in 1..self.t.get_size() {
            self.naked_number(i);
        }
    }

    pub fn naked_number(&self, i: usize) {
        let mut chk: Vec<&Cell> = Vec::with_capacity(self.t.get_size());
        for z in self.get_zone_list() {
            chk.clear();

            for c in self.zone_iter(z) {
                if c.get_chk().get_true_cnt() == i {
                    chk.push(c);
                }
            }

            Solver::naked_check(&chk, i);
        }
    }

    fn naked_check(chk: &[&Cell], i: usize) {}

    fn naked_check_recur(chk: &[&Cell], i: usize) {}
}
