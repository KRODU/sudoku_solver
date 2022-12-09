use std::hash::Hash;

use super::{cell_with_read::CellWithRead, zone::Zone};

pub struct RefZone<'a, const N: usize> {
    pub zone: &'a Zone,
    pub cells: Vec<CellWithRead<'a, N>>,
}

impl<'a, const N: usize> PartialEq for RefZone<'a, N> {
    fn eq(&self, other: &Self) -> bool {
        self.zone == other.zone
    }
}

impl<'a, const N: usize> Eq for RefZone<'a, N> {}

impl<'a, const N: usize> Hash for RefZone<'a, N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.zone.hash(state);
    }
}

impl<'a, const N: usize> std::borrow::Borrow<Zone> for RefZone<'a, N> {
    fn borrow(&self) -> &Zone {
        self.zone
    }
}

impl<'a, const N: usize> std::borrow::Borrow<Zone> for &RefZone<'a, N> {
    fn borrow(&self) -> &Zone {
        self.zone
    }
}
