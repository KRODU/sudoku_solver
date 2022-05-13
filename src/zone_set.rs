use crate::zone::Zone;

#[derive(Debug)]
pub struct ZoneSet {
    pub zone: Vec<Zone>,
}

impl ZoneSet {
    /// 지정된 zone에 포함되는지 여부를 확인
    #[must_use]
    #[inline]
    pub fn is_contain(&self, zone: &Zone) -> bool {
        self.zone.contains(zone)
    }
}

impl<'a> IntoIterator for &'a ZoneSet {
    type Item = &'a Zone;
    type IntoIter = std::slice::Iter<'a, Zone>;

    fn into_iter(self) -> Self::IntoIter {
        self.zone.iter()
    }
}
