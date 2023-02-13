use std::cell::UnsafeCell;

pub struct NonAtomicBool {
    bool: UnsafeCell<u8>,
}

impl NonAtomicBool {
    pub fn new(value: bool) -> Self {
        Self {
            bool: UnsafeCell::new(if value { 1 } else { 0 }),
        }
    }

    #[inline]
    #[must_use]
    pub fn get(&self) -> bool {
        unsafe { *self.bool.get() != 0 }
    }

    #[inline]
    pub fn set(&self, value: bool) {
        unsafe {
            *self.bool.get() = if value { 1 } else { 0 };
        }
    }
}

unsafe impl Sync for NonAtomicBool {}
