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
            Some(self.arr.get_unchecked(self.len).assume_init_read())
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        unsafe { Some(self.arr.get_unchecked(index).assume_init_ref()) }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }

        unsafe { Some(self.arr.get_unchecked_mut(index).assume_init_mut()) }
    }

    pub fn clear(&mut self) {
        unsafe {
            let slice =
                self.arr.get_unchecked_mut(..self.len) as *mut [MaybeUninit<T>] as *mut [T];
            self.len = 0;
            ptr::drop_in_place(slice);
        }
    }

    pub fn swap_remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }

        unsafe {
            let base_ptr = self.arr.as_mut_ptr() as *mut T;
            let dst = base_ptr.add(index);
            let ret = ptr::read(dst);
            self.len -= 1;
            let src = base_ptr.add(self.len);
            ptr::copy(src, dst, 1);
            Some(ret)
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }

        unsafe {
            let base_ptr = self.arr.as_mut_ptr() as *mut T;
            let ptr = base_ptr.add(index);
            let ret = ptr::read(ptr);
            self.len -= 1;
            ptr::copy(ptr.add(1), ptr, self.len - index);
            Some(ret)
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
        unsafe {
            let slice =
                self.arr.get_unchecked_mut(..self.len) as *mut [MaybeUninit<T>] as *mut [T];
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

        unsafe { self.arr.get_unchecked(index).assume_init_ref() }
    }
}

impl<T, const N: usize> IndexMut<usize> for ArrayVector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("ArrayVector_OUT_OF_INDEX");
        }

        unsafe { self.arr.get_unchecked_mut(index).assume_init_mut() }
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
            let ret = self
                .array_vector
                .arr
                .get_unchecked(self.index)
                .assume_init_read();
            self.index += 1;
            Some(ret)
        }
    }
}

impl<T, const N: usize> Drop for IntoIterArrayVector<T, N> {
    fn drop(&mut self) {
        let len = self.array_vector.len;

        unsafe {
            let slice = self.array_vector.arr.get_unchecked_mut(self.index..len)
                as *mut [MaybeUninit<T>] as *mut [T];
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
            let ret = self
                .array_vector
                .arr
                .get_unchecked(self.index)
                .assume_init_ref();
            self.index += 1;
            Some(ret)
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

    fn get_drop_test_vec(drop_order: &Rc<RefCell<String>>) -> ArrayVector<DropTest, 15> {
        let mut vec = ArrayVector::<DropTest, 15>::new();

        for num in 0..=6 {
            vec.push(DropTest {
                drop_order: Rc::clone(drop_order),
                num,
            });
        }

        vec
    }

    fn get_test_vec() -> ArrayVector<usize, 15> {
        let mut vec = ArrayVector::<usize, 15>::new();

        for num in 0..=6 {
            vec.push(num);
        }

        vec
    }

    #[test]
    fn array_vector_drop_test() {
        // drop시 메모리 해제 테스트
        let drop_order = Rc::new(RefCell::new(String::new()));
        let vec = get_drop_test_vec(&drop_order);
        drop(vec);
        assert_eq!(*drop_order.borrow(), "0,1,2,3,4,5,6,");
        drop_order.borrow_mut().clear();

        //into_iter시 메모리 해제 테스트
        let vec = get_drop_test_vec(&drop_order);
        let mut loop_cnt = 0;
        for _t in vec {
            loop_cnt += 1;
        }
        assert_eq!(*drop_order.borrow(), "0,1,2,3,4,5,6,");
        assert_eq!(loop_cnt, 7);
        drop_order.borrow_mut().clear();

        //into_iter drop시 메모리 해제 테스트
        let vec = get_drop_test_vec(&drop_order);
        let into_iter = vec.into_iter();
        drop(into_iter);
        assert_eq!(*drop_order.borrow(), "0,1,2,3,4,5,6,");
        drop_order.borrow_mut().clear();

        //clear시 메모리 해제 테스트
        let mut vec = get_drop_test_vec(&drop_order);
        vec.clear();
        assert_eq!(*drop_order.borrow(), "0,1,2,3,4,5,6,");
        drop_order.borrow_mut().clear();
        drop(vec);

        // clear 후 drop시 메모리 해제 테스트
        let mut vec = get_drop_test_vec(&drop_order);
        vec.clear();
        drop(vec);
        assert_eq!(*drop_order.borrow(), "0,1,2,3,4,5,6,");
        drop_order.borrow_mut().clear();

        // pop을 할 경우 메모리 해제 테스트
        let mut vec = get_drop_test_vec(&drop_order);
        loop_cnt = 0;
        while let Some(_t) = vec.pop() {
            loop_cnt += 1;
        }
        assert_eq!(*drop_order.borrow(), "6,5,4,3,2,1,0,");
        assert_eq!(loop_cnt, 7);
        drop_order.borrow_mut().clear();
        drop(vec);
        assert_eq!(*drop_order.borrow(), "");
        drop_order.borrow_mut().clear();

        // pop하다가 clear할 경우의 메모리 해제 테스트
        let mut vec = get_drop_test_vec(&drop_order);
        assert_eq!(vec.pop().unwrap().num, 6);
        assert_eq!(vec.pop().unwrap().num, 5);
        assert_eq!(vec.pop().unwrap().num, 4);
        assert_eq!(*drop_order.borrow(), "6,5,4,");
        drop_order.borrow_mut().clear();
        vec.clear();
        assert_eq!(*drop_order.borrow(), "0,1,2,3,");
        drop_order.borrow_mut().clear();
        drop(vec);
        assert_eq!(*drop_order.borrow(), "");
        drop_order.borrow_mut().clear();

        // into_iter에서 next하다가 into_iter가 drop될 경우의 메모리 해제 테스트
        let vec = get_drop_test_vec(&drop_order);
        let mut into_iter = vec.into_iter();
        assert_eq!(into_iter.next().unwrap().num, 0);
        assert_eq!(into_iter.next().unwrap().num, 1);
        assert_eq!(into_iter.next().unwrap().num, 2);
        assert_eq!(*drop_order.borrow(), "0,1,2,");
        drop_order.borrow_mut().clear();
        drop(into_iter); // 여기서 3,4,5,6이 drop되어야 함.
        assert_eq!(*drop_order.borrow(), "3,4,5,6,");
        drop_order.borrow_mut().clear();

        // swap_remove를 할 경우의 drop order 테스트
        let mut vec = get_drop_test_vec(&drop_order);
        vec.swap_remove(6);
        vec.swap_remove(3);
        drop(vec);
        assert_eq!(*drop_order.borrow(), "6,3,0,1,2,5,4,");
        drop_order.borrow_mut().clear();

        // remove를 할 경우의 drop order 테스트
        let mut vec = get_drop_test_vec(&drop_order);
        vec.remove(6);
        vec.remove(3);
        drop(vec);
        assert_eq!(*drop_order.borrow(), "6,3,0,1,2,4,5,");
        drop_order.borrow_mut().clear();

        // 0 크기의 빈 배열에서의 메모리 해제 테스트
        let vec = ArrayVector::<DropTest, 0>::new();
        drop(vec);
        assert_eq!(*drop_order.borrow(), "");
        drop_order.borrow_mut().clear();

        // 15 크기의 빈 배열에서의 메모리 해제 테스트
        let vec = ArrayVector::<DropTest, 15>::new();
        drop(vec);
        assert_eq!(*drop_order.borrow(), "");
        drop_order.borrow_mut().clear();

        // 빈 into_iter를 drop시 메모리 해제 테스트
        let vec = ArrayVector::<DropTest, 15>::new();
        let into_iter = vec.into_iter();
        drop(into_iter);
        assert_eq!(*drop_order.borrow(), "");
        drop_order.borrow_mut().clear();
    }

    #[test]
    #[should_panic(expected = "ArrayVector의 사이즈는 const N을 초과할 수 없음")]
    fn zero_size_panic() {
        let mut vec = ArrayVector::<usize, 0>::new();
        vec.push(0); // 여기서 panic
    }

    #[test]
    fn iter_test() {
        let mut vec = get_test_vec();
        let mut comp_value = 0_usize;
        for &value in &vec {
            assert_eq!(comp_value, value);
            comp_value += 1;
        }
        assert_eq!(comp_value - 1, 6);

        // mut 로 변경 후 비교
        for value in &mut vec {
            *value += 3;
        }

        comp_value = 3;
        for &value in &vec {
            assert_eq!(comp_value, value);
            comp_value += 1;
        }
        assert_eq!(comp_value - 1, 9);
    }
}
