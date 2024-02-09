use std::cell::UnsafeCell;

pub struct NonAtomicBool {
    bool: UnsafeCell<u8>,
}

impl NonAtomicBool {
    #[inline]
    pub const fn new(value: bool) -> Self {
        Self {
            bool: UnsafeCell::new(value as u8),
        }
    }

    #[inline]
    #[must_use]
    pub fn get(&self) -> bool {
        unsafe { std::ptr::read_volatile(self.bool.get()) != 0 }
    }

    #[inline]
    pub fn set(&self, value: bool) {
        unsafe {
            std::ptr::write_volatile(self.bool.get(), value as u8);
        }
    }
}

unsafe impl Sync for NonAtomicBool {}
