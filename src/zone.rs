use hashbrown::HashSet;

pub struct Zone {
    zone: HashSet<usize>,
}

impl Zone {
    pub fn new_with_zone(zone: &[usize]) -> Self {
        let mut z: HashSet<usize> = HashSet::with_capacity(zone.len());
        for item in zone {
            z.insert(*item);
        }

        Zone { zone: z }
    }

    pub fn new_single_zone(zone: usize) -> Self {
        let mut z: HashSet<usize> = HashSet::with_capacity(1);
        z.insert(zone);

        Zone { zone: z }
    }

    /// 지정된 zone에 포함되는지 여부를 확인
    #[must_use]
    #[inline]
    pub fn is_contain(&self, zone: usize) -> bool {
        self.zone.contains(&zone)
    }
}

impl<'a> IntoIterator for &'a Zone {
    type Item = &'a usize;
    type IntoIter = hashbrown::hash_set::Iter<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.zone.iter()
    }
}
