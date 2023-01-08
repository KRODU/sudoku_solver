use std::fmt::{Display, Formatter};
use std::hash::Hash;

/// MaxNum의 값은 0 <= value < N를 보장함.
#[derive(Debug)]
pub struct MaxNum<const N: usize> {
    num: usize,
}

impl<const N: usize> MaxNum<N> {
    /// num의 값은 num < N을 충족해야 함.
    pub fn new(num: usize) -> Self {
        assert!(num < N);
        Self { num }
    }

    /// # Safety
    ///
    /// num의 값은 num < N을 충족해야 함.
    pub unsafe fn new_unchecked(num: usize) -> Self {
        debug_assert!(num < N);
        Self { num }
    }

    #[inline]
    pub fn get_value(&self) -> usize {
        self.num
    }

    pub fn iter() -> MaxNumIter<N> {
        MaxNumIter { cur: 0 }
    }
}

impl<const N: usize> Clone for MaxNum<N> {
    fn clone(&self) -> Self {
        Self { num: self.num }
    }
}

impl<const N: usize> Copy for MaxNum<N> {}

impl<const N: usize> PartialEq for MaxNum<N> {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

impl<const N: usize> Eq for MaxNum<N> {}

impl<const N: usize> Display for MaxNum<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.num)
    }
}

impl<const N: usize> Hash for MaxNum<N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.num.hash(state);
    }
}

pub struct MaxNumIter<const N: usize> {
    cur: usize,
}

impl<const N: usize> Iterator for MaxNumIter<N> {
    type Item = MaxNum<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == N {
            return None;
        }
        let ret = unsafe { Some(MaxNum::new_unchecked(self.cur)) };

        self.cur += 1;
        ret
    }
}
