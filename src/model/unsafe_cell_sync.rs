use std::{cell::UnsafeCell, ops::Deref};

/// UnsafeCell에 Sync를 구현하여 래핑한 구조체입니다.
#[derive(Debug)]
pub struct UnsafeCellSync<T> {
    unsafe_cell: UnsafeCell<T>,
}

impl<T> UnsafeCellSync<T> {
    pub fn new(value: T) -> Self {
        Self {
            unsafe_cell: UnsafeCell::new(value),
        }
    }
}

impl<T> Deref for UnsafeCellSync<T> {
    type Target = UnsafeCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.unsafe_cell
    }
}

// UnsafeCell을 Sync로 만들더라도 unsafe 코드를 잘못 사용하지 않는 한 문제가 되지는 않는다..
unsafe impl<T> Sync for UnsafeCellSync<T> {}
