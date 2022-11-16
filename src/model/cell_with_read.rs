use super::cell::Cell;
use crate::num_check::NumCheck;
use std::sync::RwLockReadGuard;

pub struct CellWithRead<'a> {
    pub cell: &'a Cell,
    pub read: RwLockReadGuard<'a, NumCheck>,
}

impl<'a> PartialEq for CellWithRead<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cell == other.cell
    }
}
