use super::{coordinate::Coordinate, zone::Zone};
use crate::num_check::NumCheck;
use hashbrown::HashSet;
use std::sync::RwLock;

#[derive(Debug)]
pub struct Cell {
    pub chk: RwLock<NumCheck>,
    pub zone_set: HashSet<Zone>,
    pub zone_vec: Vec<Zone>,
    pub coordi: Coordinate,
}

impl Cell {
    #[must_use]
    pub fn new(size: usize, x: usize, y: usize, zone: Vec<Zone>) -> Self {
        Cell {
            chk: RwLock::new(NumCheck::new(size)),
            zone_set: zone.iter().cloned().collect(),
            zone_vec: zone,
            coordi: Coordinate::new(x, y),
        }
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.coordi == other.coordi
    }
}

impl Eq for Cell {}

impl std::hash::Hash for Cell {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.coordi.hash(state);
    }
}
