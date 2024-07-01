use std::sync::atomic::{AtomicBool, Ordering};

pub struct RelaxedBool {
    bool: AtomicBool,
}

impl RelaxedBool {
    #[inline]
    pub const fn new(value: bool) -> Self {
        Self {
            bool: AtomicBool::new(value),
        }
    }

    #[inline]
    #[must_use]
    pub fn get(&self) -> bool {
        self.bool.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn set(&self, value: bool) {
        self.bool.store(value, Ordering::Relaxed);
    }
}
