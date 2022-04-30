use crate::cell::Cell;

use super::Table;

pub struct NakedResult {
    pub naked_number: usize,
    
}

impl Table {
    pub fn naked(&self) {
        for i in 1..self.size {
            self.naked_number(i);
        }
    }

    pub fn naked_number(&self, i: usize) {
        let mut chk: Vec<&Cell> = Vec::with_capacity(self.size);
        for z in self.zone_list_iter() {
            chk.clear();
            for c in self.zone_iter(z) {
                if c.get_chk().get_true_cnt() == i {
                    chk.push(c);
                }
            }
        }
    }
}
