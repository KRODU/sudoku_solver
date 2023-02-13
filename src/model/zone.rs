use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

#[derive(PartialOrd, Ord, Debug, Clone, Copy)]
pub enum ZoneType {
    Unique,
    Sum { sum: usize },
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

impl Debug for Zone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Zone")
            .field("z", &self.z)
            .field("zone_type", &self.zone_type)
            .finish()
    }
}

#[derive(Clone, Copy, Default)]
pub struct Zone {
    z: u16,
    zone_type: ZoneType,
}

impl Zone {
    pub fn new_unique_from_num(z: usize) -> Zone {
        Zone {
            z: z.try_into().expect("can not convert from usize to u16"),
            zone_type: ZoneType::Unique,
        }
    }

    #[must_use]
    pub fn get_zone_num(&self) -> u16 {
        self.z
    }

    #[must_use]
    pub fn get_zone_type(&self) -> &ZoneType {
        &self.zone_type
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
        state.write_u16(self.z);
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
