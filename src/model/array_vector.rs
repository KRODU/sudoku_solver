use core::panic;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr,
};

/// vector처럼 동작하는 배열입니다. 최대 사이즈는 const N을 초과할 수 없습니다.
pub struct ArrayVector<T, const N: usize> {
    arr: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> ArrayVector<T, N> {
    pub const fn new() -> Self {
        Self {
            arr: unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() },
            len: 0,
        }
    }

    #[inline]
    pub fn push(&mut self, val: T) {
        if self.len == N {
            panic!("ArrayVector의 사이즈는 const N을 초과할 수 없음");
        }

        unsafe {
            self.push_unchecked(val);
        }
    }

    /// # Safety
    ///
    /// len == N 상태일 때 호출하면 UB
    #[inline]
    pub unsafe fn push_unchecked(&mut self, val: T) {
        debug_assert!(self.len < N);

        unsafe {
            self.arr.get_unchecked_mut(self.len).write(val);
            self.len += 1;
        }
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        unsafe {
            self.len -= 1;
            Some(self.arr.get_unchecked(self.len).assume_init_read())
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        unsafe { Some(self.get_unchecked(index)) }
    }

    /// # Safety
    ///
    /// index는 len보다 작아야 함.
    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        debug_assert!(index < self.len);
        unsafe { self.arr.get_unchecked(index).assume_init_ref() }
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }

        unsafe { Some(self.arr.get_unchecked_mut(index).assume_init_mut()) }
    }

    #[inline]
    pub fn clear(&mut self) {
        unsafe {
            let ptr = self.get_mut_slice_ptr();
            self.len = 0;
            // len을 0으로 설정한 다음 drop해야 함..
            // drop후 len을 0으로 설정할 경우 drop에서 panic 발생시 double-free
            ptr::drop_in_place(ptr);
        }
    }

    #[inline]
    pub fn get_slice(&self) -> &[T] {
        unsafe { &*(self.arr.get_unchecked(..self.len) as *const [MaybeUninit<T>] as *const [T]) }
    }

    #[inline]
    pub fn get_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            &mut *(self.arr.get_unchecked_mut(..self.len) as *mut [MaybeUninit<T>] as *mut [T])
        }
    }

    #[inline]
    pub fn get_mut_slice_ptr(&mut self) -> *mut [T] {
        unsafe { self.arr.get_unchecked_mut(..self.len) as *mut [MaybeUninit<T>] as *mut [T] }
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

    /// 매개변수로 함수 f를 이용하여 배열을 오른쪽에서부터 테스트하며, f가 true를 반환할 경우 해당 요소를 배열에서 제거합니다.
    /// 이때 제거는 swap_remove로 동작합니다.
    pub fn r_loop_swap_remove(&mut self, mut f: impl FnMut(&T) -> bool) {
        for index in (0..self.len).rev() {
            unsafe {
                if f(self.get_unchecked(index)) {
                    let base_ptr = self.arr.as_mut_ptr() as *mut T;
                    let dst = base_ptr.add(index);
                    // 여기서 바로 drop하는 대신 일부러 drop을 늦춤..
                    // 그렇지 않으면 drop에서 panic이 발생시 double-free가 발생할 수 있음
                    let _late_drop = ptr::read(dst);
                    self.len -= 1;
                    let src = base_ptr.add(self.len);
                    ptr::copy(src, dst, 1);
                }
            }
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
            ptr::drop_in_place(self.get_mut_slice_ptr());
        }
    }
}

impl<T, const N: usize> Clone for ArrayVector<T, N>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let mut ret = ArrayVector::new();

        for element in self {
            // 반환할 ArrayVector의 N과 self의 N은 무조건 동일하므로 safe
            unsafe {
                ret.push_unchecked(element.clone());
            }
        }

        ret
    }
}

impl<T, const N: usize> Debug for ArrayVector<T, N>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArrayVector")
            .field("arr", &self.get_slice())
            .field("len", &self.len)
            .finish()
    }
}

impl<T, const N: usize> Display for ArrayVector<T, N>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut arr_str = String::new();
        arr_str.push('[');
        for ele in self {
            arr_str.push_str(&ele.to_string());
            arr_str.push_str(", ");
        }

        if arr_str.len() > 1 {
            arr_str.pop();
            arr_str.pop();
        }

        arr_str.push(']');
        write!(f, "{}", &arr_str)
    }
}

impl<T, const N: usize> Hash for ArrayVector<T, N>
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_slice().hash(state);
        self.len.hash(state);
    }
}

impl<T, const N: usize> FromIterator<T> for ArrayVector<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ret = ArrayVector::new();

        for val in iter {
            ret.push(val);
        }

        ret
    }
}

impl<T, const N: usize> Deref for ArrayVector<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.get_slice()
    }
}

impl<T, const N: usize> DerefMut for ArrayVector<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut_slice()
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
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.get_slice().iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut ArrayVector<T, N> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.get_mut_slice().iter_mut()
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

        // r_loop_remove로 짝수를 제거할 경우의 drop order 테스트
        let mut vec = get_drop_test_vec(&drop_order);
        vec.r_loop_swap_remove(|v| v.num % 2 == 0);
        assert_eq!(*drop_order.borrow(), "6,4,2,0,");
        drop_order.borrow_mut().clear();
        drop(vec);
        assert_eq!(*drop_order.borrow(), "3,1,5,");
        drop_order.borrow_mut().clear();

        // r_loop_remove로 홀수를 제거할 경우의 drop order 테스트
        let mut vec = get_drop_test_vec(&drop_order);
        vec.r_loop_swap_remove(|v| v.num % 2 != 0);
        assert_eq!(*drop_order.borrow(), "5,3,1,");
        drop_order.borrow_mut().clear();
        drop(vec);
        assert_eq!(*drop_order.borrow(), "0,4,2,6,");
        drop_order.borrow_mut().clear();

        // r_loop_remove를 할 경우의 drop order 테스트
        let mut vec = get_drop_test_vec(&drop_order);
        vec.r_loop_swap_remove(|_| true);
        assert_eq!(*drop_order.borrow(), "6,5,4,3,2,1,0,");
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

        // 0 크기의 빈 into_iter를 drop시 메모리 해제 테스트
        let vec = ArrayVector::<DropTest, 0>::new();
        let into_iter = vec.into_iter();
        drop(into_iter);
        assert_eq!(*drop_order.borrow(), "");
        drop_order.borrow_mut().clear();

        // 15 크기의 빈 into_iter를 drop시 메모리 해제 테스트
        let vec = ArrayVector::<DropTest, 15>::new();
        let into_iter = vec.into_iter();
        drop(into_iter);
        assert_eq!(*drop_order.borrow(), "");
        drop_order.borrow_mut().clear();
    }

    #[test]
    #[should_panic(expected = "ArrayVector의 사이즈는 const N을 초과할 수 없음")]
    fn zero_size_push_panic() {
        let mut vec = ArrayVector::<usize, 0>::new();
        vec.push(0); // 여기서 panic
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
    fn zero_size_index_panic() {
        let vec = ArrayVector::<usize, 15>::new();
        let _m = vec[0]; // 여기서 panic
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
    fn zero_size_index_mut_panic() {
        let mut vec = ArrayVector::<usize, 15>::new();
        vec[0] += 5; // 여기서 panic
    }

    #[test]
    fn fmt_test() {
        let vec = get_test_vec();
        assert_eq!(
            format!("{:?}", vec),
            "ArrayVector { arr: [0, 1, 2, 3, 4, 5, 6], len: 7 }"
        );
        assert_eq!(vec.to_string(), "[0, 1, 2, 3, 4, 5, 6]");

        let empty_vec = ArrayVector::<usize, 15>::new();
        assert_eq!(
            format!("{:?}", empty_vec),
            "ArrayVector { arr: [], len: 0 }"
        );
        assert_eq!(empty_vec.to_string(), "[]");
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

        comp_value = 0;
        for index in 0..=6 {
            assert_eq!(vec[index], comp_value);
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

        for index in 0..=6 {
            vec[index] += 3;
        }

        comp_value = 6;
        for index in 0..=6 {
            assert_eq!(vec[index], comp_value);
            comp_value += 1;
        }
        assert_eq!(comp_value - 1, 12);
    }

    #[test]
    fn deref_test() {
        let mut vec = ArrayVector::<usize, 15>::new();
        vec.push(8);
        vec.push(0);
        vec.push(3);
        vec.push(7);
        vec.sort_unstable();
        assert_eq!(vec.to_string(), "[0, 3, 7, 8]");
        let (left, right) = vec.split_at(2);
        assert_eq!(left, &[0, 3]);
        assert_eq!(right, &[7, 8]);
    }
}
