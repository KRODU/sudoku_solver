use super::{
    coordinate::Coordinate, max_num::MaxNum, unsafe_cell_sync::UnsafeCellSync, zone::Zone,
};
use crate::num_check::NumCheck;
use hashbrown::HashSet;

#[derive(Debug)]
pub struct Cell<const N: usize> {
    pub(super) chk_unsafe: UnsafeCellSync<NumCheck<N>>,
    pub zone_set: HashSet<Zone>,
    pub zone_vec: Vec<Zone>,
    pub coordi: Coordinate<N>,
}

impl<const N: usize> Cell<N> {
    #[must_use]
    pub fn new(x: usize, y: usize, zone: Vec<Zone>) -> Self {
        Cell {
            chk_unsafe: UnsafeCellSync::new(NumCheck::<N>::new_with_true()),
            zone_set: zone.iter().cloned().collect(),
            zone_vec: zone,
            coordi: Coordinate::new(
                MaxNum::new_with_zero_offset(x),
                MaxNum::new_with_zero_offset(y),
            ),
        }
    }
}

impl<const N: usize> PartialEq for Cell<N> {
    fn eq(&self, other: &Self) -> bool {
        self.coordi == other.coordi
    }
}

impl<const N: usize> Eq for Cell<N> {}

impl<const N: usize> std::hash::Hash for Cell<N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coordi.hash(state);
    }
}
