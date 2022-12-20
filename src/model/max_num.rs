use std::fmt::{Display, Formatter};
use std::hash::Hash;

#[derive(Debug)]
pub struct MaxNum<const N: usize> {
    num: usize,
}

impl<const N: usize> MaxNum<N> {
    pub fn new(num: usize) -> Self {
        assert!(num > 0 && num <= N);
        Self { num }
    }

    pub fn new_with_zero_offset(num: usize) -> Self {
        Self::new(num + 1)
    }

    /// # Safety
    ///
    /// num의 값은 num >= MIN && num <= MAX을 충족해야 함.
    pub unsafe fn new_unchecked(num: usize) -> Self {
        Self { num }
    }

    pub fn get_note(&self) -> usize {
        self.num
    }

    pub fn get_zero_offset(&self) -> usize {
        self.num - 1
    }

    pub fn note_iter() -> NoteIter<N> {
        NoteIter { cur: 1 }
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

pub struct NoteIter<const N: usize> {
    cur: usize,
}

impl<const N: usize> Iterator for NoteIter<N> {
    type Item = MaxNum<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur > N {
            return None;
        }
        let ret = unsafe { Some(MaxNum::new_unchecked(self.cur)) };

        self.cur += 1;
        ret
    }
}
