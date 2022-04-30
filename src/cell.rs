use crate::{coordinate::Coordinate, num_check::NumCheck, zone::Zone};

pub struct Cell {
    pub chk: NumCheck,
    zone: Zone,
    coordi: Coordinate,
}

impl Cell {
    #[must_use]
    pub fn new(size: usize, x: usize, y: usize, zone: &[usize]) -> Self {
        let ret = Cell {
            chk: NumCheck::new(size),
            zone: Zone::new_with_zone(zone),
            coordi: Coordinate { x, y },
        };
        ret
    }

    pub fn new_single_zone(size: usize, x: usize, y: usize, zone: usize) -> Self {
        let ret = Cell {
            chk: NumCheck::new(size),
            zone: Zone::new_single_zone(zone),
            coordi: Coordinate { x, y },
        };
        ret
    }
    #[must_use]
    #[inline]
    pub fn get_chk(&self) -> &NumCheck {
        &self.chk
    }

    ///
    #[must_use]
    #[inline]
    pub fn get_zone(&self) -> &Zone {
        &self.zone
    }

    #[must_use]
    #[inline]
    pub fn is_zone_contain(&self, zone: usize) -> bool {
        self.zone.is_contain(zone)
    }

    /// 현재 Cell의 x, y 좌표를 가져온다.
    #[must_use]
    #[inline]
    pub fn get_coordinate(&self) -> Coordinate {
        self.coordi
    }
}
