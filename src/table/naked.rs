use super::Table;

pub struct naked_result {
    pub naked_number: usize,
    
}

impl Table {
    pub fn naked(&self) {
        for i in 0..=self.size {
            self.naked_number(i);
        }
    }

    pub fn naked_number(&self, i: usize) {

    }
}
