use super::max_num::MaxNum;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
pub struct ArrayNote<T, const N: usize> {
    arr: [T; N],
}

impl<T, const N: usize> ArrayNote<T, N> {
    pub const fn new(param: [T; N]) -> Self {
        Self { arr: param }
    }

    pub fn set(&mut self, param: [T; N]) {
        self.arr = param;
    }
}

impl<T, const N: usize> Deref for ArrayNote<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.arr
    }
}

impl<T, const N: usize> DerefMut for ArrayNote<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.arr
    }
}

impl<T, const N: usize> Index<MaxNum<N>> for ArrayNote<T, N> {
    type Output = T;

    fn index(&self, index: MaxNum<N>) -> &Self::Output {
        // MaxNum<N>은 0 <= get_zero_offset < N이 보장됨
        unsafe { self.arr.get_unchecked(index.get_zero_offset()) }
    }
}

impl<T, const N: usize> IndexMut<MaxNum<N>> for ArrayNote<T, N> {
    fn index_mut(&mut self, index: MaxNum<N>) -> &mut Self::Output {
        // MaxNum<N>은 0 <= get_zero_offset < N이 보장됨
        unsafe { self.arr.get_unchecked_mut(index.get_zero_offset()) }
    }
}

impl<T, const N: usize> PartialEq for ArrayNote<T, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.arr == other.arr
    }
}

impl<T, const N: usize> Eq for ArrayNote<T, N> where T: Eq {}
