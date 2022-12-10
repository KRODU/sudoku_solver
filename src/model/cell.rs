use super::{coordinate::Coordinate, zone::Zone};
use crate::num_check::NumCheck;
use hashbrown::HashSet;
use std::sync::RwLock;

#[derive(Debug)]
pub struct Cell<const N: usize> {
    pub chk: RwLock<NumCheck<N>>,
    pub zone_set: HashSet<Zone>,
    pub zone_vec: Vec<Zone>,
    pub coordi: Coordinate,
}

impl<const N: usize> Cell<N> {
    #[must_use]
    pub fn new(x: usize, y: usize, zone: Vec<Zone>) -> Self {
        Cell {
            chk: RwLock::new(NumCheck::<N>::new_with_true()),
            zone_set: zone.iter().cloned().collect(),
            zone_vec: zone,
            coordi: Coordinate::new(x, y),
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
