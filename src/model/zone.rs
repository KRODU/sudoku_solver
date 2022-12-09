use std::hash::{Hash, Hasher};

#[derive(PartialOrd, Ord, Debug)]
pub enum ZoneType {
    Unique,
    Sum { sum: usize },
}

impl Clone for ZoneType {
    fn clone(&self) -> Self {
        match self {
            Self::Unique => Self::Unique,
            Self::Sum { sum } => Self::Sum { sum: *sum },
        }
    }
}

impl Default for ZoneType {
    fn default() -> Self {
        Self::Unique
    }
}

impl PartialEq for ZoneType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for ZoneType {}

impl Hash for ZoneType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

#[derive(Debug)]
pub struct Zone {
    z: usize,
    zone_type: ZoneType,
    hash_cache: u64,
}

impl Zone {
    pub fn new_unique_from_num(z: usize) -> Zone {
        let mut state = ahash::AHasher::default();
        z.hash(&mut state);
        ZoneType::Unique.hash(&mut state);

        Zone {
            z,
            zone_type: ZoneType::Unique,
            hash_cache: state.finish(),
        }
    }

    #[must_use]
    pub fn get_zone_num(&self) -> usize {
        self.z
    }

    #[must_use]
    pub fn get_zone_type(&self) -> &ZoneType {
        &self.zone_type
    }
}

impl Clone for Zone {
    fn clone(&self) -> Self {
        Self {
            z: self.z,
            zone_type: self.zone_type.clone(),
            hash_cache: self.hash_cache,
        }
    }
}

impl PartialEq for Zone {
    fn eq(&self, other: &Self) -> bool {
        self.z == other.z && self.zone_type == other.zone_type
    }
}

impl Eq for Zone {}

impl std::hash::Hash for Zone {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash_cache);
    }
}

impl PartialOrd for Zone {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.z.partial_cmp(&other.z) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.zone_type.partial_cmp(&other.zone_type)
    }
}

impl Ord for Zone {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.z.cmp(&other.z) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.zone_type.cmp(&other.zone_type)
    }
}
