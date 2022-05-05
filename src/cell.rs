use std::cell::RefCell;

use crate::{coordinate::Coordinate, num_check::NumCheck, zone::Zone, zone_set::ZoneSet};

pub struct Cell {
    pub chk: RefCell<NumCheck>,
    zone: ZoneSet,
    coordi: Coordinate,
}

impl Cell {
    #[must_use]
    pub fn new(size: usize, x: usize, y: usize, zone: &[Zone]) -> Self {
        Cell {
            chk: RefCell::new(NumCheck::new(size)),
            zone: ZoneSet::new_with_zone(zone),
            coordi: Coordinate { x, y },
        }
    }

    pub fn new_single_zone(size: usize, x: usize, y: usize, zone: Zone) -> Self {
        Cell {
            chk: RefCell::new(NumCheck::new(size)),
            zone: ZoneSet::new_single_zone(zone),
            coordi: Coordinate { x, y },
        }
    }

    ///
    #[must_use]
    #[inline]
    pub fn get_zone(&self) -> &ZoneSet {
        &self.zone
    }

    #[must_use]
    #[inline]
    pub fn is_zone_contain(&self, zone: Zone) -> bool {
        self.zone.is_contain(zone)
    }

    /// 현재 Cell의 x, y 좌표를 가져옵니다.
    #[must_use]
    #[inline]
    pub fn get_coordinate(&self) -> Coordinate {
        self.coordi
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
