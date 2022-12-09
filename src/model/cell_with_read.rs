use super::cell::Cell;
use crate::num_check::NumCheck;
use std::sync::RwLockReadGuard;

pub struct CellWithRead<'a, const N: usize> {
    pub cell: &'a Cell<N>,
    pub read: RwLockReadGuard<'a, NumCheck<N>>,
}

impl<'a, const N: usize> PartialEq for CellWithRead<'a, N> {
    fn eq(&self, other: &Self) -> bool {
        self.cell == other.cell
    }
}
