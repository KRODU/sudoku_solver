use hashbrown::HashSet;

use crate::zone::Zone;

pub struct ZoneSet {
    zone: HashSet<Zone>,
}

impl ZoneSet {
    pub fn new_with_zone(zone: &[Zone]) -> Self {
        let mut z: HashSet<Zone> = HashSet::with_capacity(zone.len());
        for item in zone {
            z.insert(item.clone());
        }

        ZoneSet { zone: z }
    }

    pub fn new_single_zone(zone: Zone) -> Self {
        let mut z: HashSet<Zone> = HashSet::with_capacity(1);
        z.insert(zone);

        ZoneSet { zone: z }
    }

    /// 지정된 zone에 포함되는지 여부를 확인
    #[must_use]
    #[inline]
    pub fn is_contain(&self, zone: Zone) -> bool {
        self.zone.contains(&zone)
    }
}

impl<'a> IntoIterator for &'a ZoneSet {
    type Item = &'a Zone;
    type IntoIter = hashbrown::hash_set::Iter<'a, Zone>;

    fn into_iter(self) -> Self::IntoIter {
        self.zone.iter()
    }
}
