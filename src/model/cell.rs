use super::{max_num::MaxNum, unsafe_cell_sync::UnsafeCellSync, zone::Zone};
use crate::num_check::NumCheck;
use hashbrown::HashSet;
use std::marker::PhantomPinned;

#[derive(Debug)]
pub struct Cell<const N: usize> {
    pub(crate) chk_unsafe: UnsafeCellSync<NumCheck<N>>,
    pub(crate) zone_set: HashSet<Zone>,
    pub(crate) zone_vec: Vec<Zone>,
    x: MaxNum<N>,
    y: MaxNum<N>,
    _pin: PhantomPinned,
}

impl<const N: usize> Cell<N> {
    #[must_use]
    pub fn new(x: usize, y: usize, zone: Vec<Zone>) -> Self {
        Cell {
            chk_unsafe: UnsafeCellSync::new(NumCheck::<N>::new_with_true()),
            zone_set: zone.iter().cloned().collect(),
            zone_vec: zone,
            x: MaxNum::new(x),
            y: MaxNum::new(y),
            _pin: PhantomPinned,
        }
    }

    pub fn get_coordinate(&self) -> (MaxNum<N>, MaxNum<N>) {
        (self.x, self.y)
    }
}

impl<const N: usize> PartialEq for Cell<N> {
    fn eq(&self, other: &Self) -> bool {
        // eq는 포인터 주소가 일치하는지를 비교
        // 좌표가 같더라도 다른 테이블에 속한 cell과는 동일하지 않으므로 일부러 이렇게 함..
        std::ptr::eq(self, other)
    }
}

impl<const N: usize> Eq for Cell<N> {}

impl<const N: usize> std::hash::Hash for Cell<N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
