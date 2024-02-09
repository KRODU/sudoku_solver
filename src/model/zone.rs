use crate::model::index_key_map::IndexKey;
use std::fmt::Debug;

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
    pub const fn new_unique_from_u16(z: u16) -> Zone {
        Zone {
            z,
            zone_type: ZoneType::Unique,
        }
    }

    pub fn new_unique_from_usize(z: usize) -> Zone {
        Zone {
            z: z.try_into().expect("can not convert from usize to u16"),
            zone_type: ZoneType::Unique,
        }
    }

    #[must_use]
    #[inline]
    pub fn get_zone_num(&self) -> u16 {
        self.z
    }

    #[must_use]
    #[inline]
    pub fn get_zone_type(&self) -> ZoneType {
        self.zone_type
    }
}

impl PartialEq for Zone {
    fn eq(&self, other: &Self) -> bool {
        self.z == other.z && self.zone_type == other.zone_type
    }
}

impl Eq for Zone {}

impl PartialOrd for Zone {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Zone {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.z.cmp(&other.z) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.zone_type.cmp(&other.zone_type)
    }
}

impl IndexKey for Zone {
    #[inline]
    fn index(&self) -> u16 {
        self.get_zone_num()
    }
}
