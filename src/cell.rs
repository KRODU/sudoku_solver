use std::sync::RwLock;

use crate::{coordinate::Coordinate, num_check::NumCheck, zone::Zone, zone_set::ZoneSet};

#[derive(Debug)]
pub struct Cell {
    pub chk: RwLock<NumCheck>,
    pub zone: ZoneSet,
    pub coordi: Coordinate,
}

impl Cell {
    #[must_use]
    pub fn new(size: usize, x: usize, y: usize, zone: Vec<Zone>) -> Self {
        Cell {
            chk: RwLock::new(NumCheck::new(size)),
            zone: ZoneSet { zone },
            coordi: Coordinate { x, y },
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
