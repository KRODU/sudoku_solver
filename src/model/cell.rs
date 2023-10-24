use super::{
    index_key_map::IndexKeySet, max_num::MaxNum, unsafe_cell_sync::UnsafeCellSync, zone::Zone,
};
use crate::num_check::NumCheck;
use std::{fmt::Debug, marker::PhantomPinned};

pub struct Cell<const N: usize> {
    pub(crate) chk_unsafe: UnsafeCellSync<NumCheck<N>>,
    pub(crate) zone_set: IndexKeySet<Zone>,
    pub(crate) zone_vec: Vec<Zone>,
    x: MaxNum<N>,
    y: MaxNum<N>,
    index: usize,
    _pin: PhantomPinned,
}

impl<const N: usize> Cell<N> {
    #[must_use]
    pub fn new(x: usize, y: usize, zone: Vec<Zone>) -> Self {
        let x = MaxNum::new(x);
        let y = MaxNum::new(y);

        Cell {
            chk_unsafe: UnsafeCellSync::new(NumCheck::<N>::new_with_true()),
            zone_set: zone.iter().cloned().collect(),
            zone_vec: zone,
            x,
            y,
            index: x.get_value() * N + y.get_value(),
            _pin: PhantomPinned,
        }
    }

    #[must_use]
    #[inline]
    pub fn get_coordinate(&self) -> (MaxNum<N>, MaxNum<N>) {
        (self.x, self.y)
    }

    #[must_use]
    #[inline]
    pub fn get_zone(&self) -> &[Zone] {
        &self.zone_vec
    }
}

impl<const N: usize> Debug for Cell<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cell")
            .field("zone_vec", &self.zone_vec)
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl<const N: usize> PartialEq for Cell<N> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // eq는 포인터 주소가 일치하는지를 비교
        // 좌표가 같더라도 다른 테이블에 속한 cell과는 동일하지 않으므로 일부러 이렇게 함..
        std::ptr::eq(self, other)
    }
}

impl<const N: usize> Eq for Cell<N> {}

impl<const N: usize> PartialOrd for Cell<N> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for Cell<N> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl<const N: usize> std::hash::Hash for Cell<N> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}
