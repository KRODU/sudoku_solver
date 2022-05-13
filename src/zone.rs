#[derive(Debug)]
pub enum ZoneType {
    Unique,
    Sum { sum: u32 },
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

#[derive(Debug)]
pub struct Zone {
    pub z: u32,
    pub zone_type: ZoneType,
}

impl Zone {
    pub fn new_from_num(z: u32) -> Zone {
        Zone {
            z,
            zone_type: ZoneType::Unique,
        }
    }

    #[must_use]
    pub fn get_zone_num(&self) -> u32 {
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
        }
    }
}

impl PartialEq for Zone {
    fn eq(&self, other: &Self) -> bool {
        self.z == other.z
    }
}

impl Eq for Zone {}

impl std::hash::Hash for Zone {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.z.hash(state);
    }
}
