use std::hash::Hash;

use super::{cell_with_read::CellWithRead, zone::Zone};

pub struct RefZone<'a> {
    pub zone: &'a Zone,
    pub cells: Vec<CellWithRead<'a>>,
}

impl<'a> PartialEq for RefZone<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.zone == other.zone
    }
}

impl<'a> Eq for RefZone<'a> {}

impl<'a> Hash for RefZone<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.zone.hash(state);
    }
}

impl<'a> std::borrow::Borrow<Zone> for RefZone<'a> {
    fn borrow(&self) -> &Zone {
        self.zone
    }
}

impl<'a> std::borrow::Borrow<Zone> for &RefZone<'a> {
    fn borrow(&self) -> &Zone {
        self.zone
    }
}
