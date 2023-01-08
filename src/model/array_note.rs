use super::{array_vector::ArrayVector, max_num::MaxNum};
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
        // MaxNum<N>의 값은 N보다 작은 것이 보장됨
        unsafe { self.arr.get_unchecked(index.get_value()) }
    }
}

impl<T, const N: usize> IndexMut<MaxNum<N>> for ArrayNote<T, N> {
    fn index_mut(&mut self, index: MaxNum<N>) -> &mut Self::Output {
        // MaxNum<N>의 값은 N보다 작은 것이 보장됨
        unsafe { self.arr.get_unchecked_mut(index.get_value()) }
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

impl<const N: usize> ArrayNote<bool, N> {
    pub fn bool_array_note_to_array_vec(&self) -> ArrayVector<MaxNum<N>, N> {
        let mut ret: ArrayVector<MaxNum<N>, N> = ArrayVector::new();

        for (i, b) in self.iter().enumerate() {
            if *b {
                unsafe {
                    // ret의 N과 self의 N은 같은것이 보장됨.
                    ret.push_unchecked(MaxNum::new_unchecked(i));
                }
            }
        }

        ret
    }
}
