use super::max_num::MaxNum;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Coordinate<const N: usize> {
    x: MaxNum<N>,
    y: MaxNum<N>,
    hash_cache: u64,
}

impl<const N: usize> Coordinate<N> {
    pub fn new(x: MaxNum<N>, y: MaxNum<N>) -> Self {
        let mut state = ahash::AHasher::default();
        x.hash(&mut state);
        y.hash(&mut state);

        Self {
            x,
            y,
            hash_cache: state.finish(),
        }
    }
}

impl<const N: usize> PartialEq for Coordinate<N> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<const N: usize> Eq for Coordinate<N> {}

impl<const N: usize> std::hash::Hash for Coordinate<N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash_cache);
    }
}
