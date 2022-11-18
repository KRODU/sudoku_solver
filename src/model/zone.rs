#[derive(Debug)]
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
        match (self, other) {
            (Self::Sum { sum: l_sum }, Self::Sum { sum: r_sum }) => l_sum == r_sum,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Eq for ZoneType {}

impl std::hash::Hash for ZoneType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

#[derive(Debug)]
pub struct Zone {
    pub z: usize,
    pub zone_type: ZoneType,
}

impl Zone {
    pub fn new_unique_from_num(z: usize) -> Zone {
        Zone {
            z,
            zone_type: ZoneType::Unique,
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

impl PartialEq for Zone {
    fn eq(&self, other: &Self) -> bool {
        self.z == other.z && self.zone_type == other.zone_type
    }
}

impl Eq for Zone {}

impl std::hash::Hash for Zone {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.z.hash(state);
        self.zone_type.hash(state);
    }
}
