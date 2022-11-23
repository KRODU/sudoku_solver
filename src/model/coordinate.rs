use std::hash::Hasher;

#[derive(Debug)]
pub struct Coordinate {
    x: usize,
    y: usize,
    hash_cache: u64,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        let mut state = ahash::AHasher::default();
        state.write_usize(x);
        state.write_usize(y);

        Self {
            x,
            y,
            hash_cache: state.finish(),
        }
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Coordinate {}

impl std::hash::Hash for Coordinate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash_cache);
    }
}
