pub struct Zone {
    z: usize,
}

impl Zone {
    pub fn new_from_num(z: usize) -> Zone {
        Zone { z }
    }

    pub fn get_zone_num(&self) -> usize {
        self.z
    }
}

impl Clone for Zone {
    fn clone(&self) -> Self {
        Self { z: self.z }
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
