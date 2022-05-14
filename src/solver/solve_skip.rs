use crate::zone::Zone;

pub enum SkipType {
    Naked,
}

impl PartialEq for SkipType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for SkipType {}

impl std::hash::Hash for SkipType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Clone for SkipType {
    fn clone(&self) -> Self {
        match self {
            Self::Naked => Self::Naked,
        }
    }
}

pub struct SkipThis {
    pub skip_type: SkipType,
    pub skip_zone: Zone,
}

impl PartialEq for SkipThis {
    fn eq(&self, other: &Self) -> bool {
        self.skip_type == other.skip_type && self.skip_zone == other.skip_zone
    }
}

impl Eq for SkipThis {}

impl std::hash::Hash for SkipThis {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.skip_type.hash(state);
        self.skip_zone.hash(state);
    }
}

impl Clone for SkipThis {
    fn clone(&self) -> Self {
        Self {
            skip_type: self.skip_type.clone(),
            skip_zone: self.skip_zone.clone(),
        }
    }
}
