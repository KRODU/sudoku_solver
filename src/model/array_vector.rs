use core::panic;
use std::{
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Index, IndexMut},
    ptr,
};

/// vector처럼 동작하는 배열입니다. 최대 사이즈는 const N을 초과할 수 없습니다.
pub struct ArrayVector<T, const N: usize> {
    arr: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> ArrayVector<T, N> {
    pub fn new() -> Self {
        Self {
            arr: unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() },
            len: 0,
        }
    }

    pub fn push(&mut self, val: T) {
        if self.len == N {
            panic!("ArrayVector의 사이즈는 const N을 초과할 수 없음");
        }

        unsafe {
            self.arr.get_unchecked_mut(self.len).write(val);
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        unsafe {
            self.len -= 1;
            let ptr = self.arr.get_unchecked(self.len).as_ptr();
            Some(ptr::read(ptr))
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        unsafe { Some(&*self.arr.get_unchecked(index).as_ptr()) }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }

        unsafe { Some(&mut *self.arr.get_unchecked_mut(index).as_mut_ptr()) }
    }

    pub fn clear(&mut self) {
        if N == 0 {
            return;
        }

        unsafe {
            let ptr = self.arr.get_unchecked_mut(0).as_mut_ptr();
            let slice = ptr::slice_from_raw_parts_mut(ptr, self.len);
            self.len = 0;
            ptr::drop_in_place(slice);
        }
    }
}

impl<T, const N: usize> Default for ArrayVector<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Drop for ArrayVector<T, N> {
    fn drop(&mut self) {
        if N == 0 {
            return;
        }

        unsafe {
            let ptr = self.arr.get_unchecked_mut(0).as_mut_ptr();
            let slice = ptr::slice_from_raw_parts_mut(ptr, self.len);
            ptr::drop_in_place(slice);
        }
    }
}

impl<T, const N: usize> Index<usize> for ArrayVector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("ArrayVector_OUT_OF_INDEX");
        }

        unsafe { &*self.arr.get_unchecked(index).as_ptr() }
    }
}

impl<T, const N: usize> IndexMut<usize> for ArrayVector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("ArrayVector_OUT_OF_INDEX");
        }

        unsafe { &mut *self.arr.get_unchecked_mut(index).as_mut_ptr() }
    }
}

impl<T, const N: usize> IntoIterator for ArrayVector<T, N> {
    type Item = T;
    type IntoIter = IntoIterArrayVector<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterArrayVector {
            array_vector: ManuallyDrop::new(self),
            index: 0,
        }
    }
}

pub struct IntoIterArrayVector<T, const N: usize> {
    // ManuallyDrop을 하지 않으면 double-free 발생..
    array_vector: ManuallyDrop<ArrayVector<T, N>>,
    index: usize,
}

impl<T, const N: usize> Iterator for IntoIterArrayVector<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.array_vector.len {
            return None;
        }

        unsafe {
            let ptr = self.array_vector.arr.get_unchecked(self.index).as_ptr();
            let ret = ptr::read(ptr);
            self.index += 1;
            Some(ret)
        }
    }
}

impl<T, const N: usize> Drop for IntoIterArrayVector<T, N> {
    fn drop(&mut self) {
        let len = self.array_vector.len - self.index;
        if len == 0 {
            return;
        }

        unsafe {
            let ptr = self
                .array_vector
                .arr
                .get_unchecked_mut(self.index)
                .as_mut_ptr();
            let slice = ptr::slice_from_raw_parts_mut(ptr, len);
            ptr::drop_in_place(slice);
        }
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a ArrayVector<T, N> {
    type Item = &'a T;
    type IntoIter = IterArrayVector<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        IterArrayVector {
            array_vector: self,
            index: 0,
        }
    }
}

pub struct IterArrayVector<'a, T, const N: usize> {
    array_vector: &'a ArrayVector<T, N>,
    index: usize,
}

impl<'a, T, const N: usize> Iterator for IterArrayVector<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.array_vector.len {
            return None;
        }

        unsafe {
            let ptr = self.array_vector.arr.get_unchecked(self.index).as_ptr();
            self.index += 1;
            Some(&*ptr)
        }
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut ArrayVector<T, N> {
    type Item = &'a mut T;
    type IntoIter = IterMutArrayVector<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        IterMutArrayVector {
            array_vector: self,
            index: 0,
        }
    }
}

pub struct IterMutArrayVector<'a, T, const N: usize> {
    array_vector: &'a mut ArrayVector<T, N>,
    index: usize,
}

impl<'a, T, const N: usize> Iterator for IterMutArrayVector<'a, T, N> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.array_vector.len {
            return None;
        }

        unsafe {
            let ptr = self
                .array_vector
                .arr
                .get_unchecked_mut(self.index)
                .as_mut_ptr();
            self.index += 1;
            Some(&mut *ptr)
        }
    }
}

#[cfg(test)]
mod array_vector_test {
    use crate::model::array_vector::ArrayVector;
    use std::cell::RefCell;
    use std::rc::Rc;

    struct DropTest {
        pub drop_order: Rc<RefCell<String>>,
        pub num: usize,
    }

    impl Drop for DropTest {
        fn drop(&mut self) {
            let mut num_str = self.num.to_string();
            num_str.push(',');
            self.drop_order.borrow_mut().push_str(num_str.as_str());
        }
    }

    fn get_test_vec(drop_order: &Rc<RefCell<String>>) -> ArrayVector<DropTest, 15> {
        let mut vec = ArrayVector::<DropTest, 15>::new();

        for num in 0..=6 {
            vec.push(DropTest {
                drop_order: Rc::clone(drop_order),
                num,
            });
        }

        vec
    }

    #[test]
    fn array_vector_drop_test() {
        let drop_order = Rc::new(RefCell::new(String::new()));
        let vec = get_test_vec(&drop_order);
        drop(vec);
        assert_eq!(*drop_order.borrow(), "0,1,2,3,4,5,6,");
        drop_order.borrow_mut().clear();

        let vec = get_test_vec(&drop_order);
        let mut loop_cnt = 0;
        for _t in vec {
            loop_cnt += 1;
        }
        assert_eq!(*drop_order.borrow(), "0,1,2,3,4,5,6,");
        assert_eq!(loop_cnt, 7);
        drop_order.borrow_mut().clear();

        let mut vec = get_test_vec(&drop_order);
        vec.clear();
        assert_eq!(*drop_order.borrow(), "0,1,2,3,4,5,6,");
        drop_order.borrow_mut().clear();
        drop(vec);

        let mut vec = get_test_vec(&drop_order);
        vec.clear();
        drop(vec);
        assert_eq!(*drop_order.borrow(), "0,1,2,3,4,5,6,");
        drop_order.borrow_mut().clear();

        let mut vec = get_test_vec(&drop_order);
        loop_cnt = 0;
        while let Some(_t) = vec.pop() {
            loop_cnt += 1;
        }
        drop(vec);
        assert_eq!(*drop_order.borrow(), "6,5,4,3,2,1,0,");
        assert_eq!(loop_cnt, 7);
        drop_order.borrow_mut().clear();

        let vec = get_test_vec(&drop_order);
        let mut into_iter = vec.into_iter();
        assert_eq!(into_iter.next().unwrap().num, 0);
        assert_eq!(into_iter.next().unwrap().num, 1);
        assert_eq!(into_iter.next().unwrap().num, 2);
        assert_eq!(*drop_order.borrow(), "0,1,2,");
        drop_order.borrow_mut().clear();
        drop(into_iter); // 여기서 3,4,5,6이 drop되어야 함.
        assert_eq!(*drop_order.borrow(), "3,4,5,6,");
        drop_order.borrow_mut().clear();
    }
}
