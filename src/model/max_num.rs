use std::fmt::{Debug, Display, Formatter};
use std::hint::unreachable_unchecked;
use std::ops::{Bound, RangeBounds};

/// MaxNum의 값은 0 <= value < N를 보장함.
pub struct MaxNum<const N: usize> {
    num: usize,
}

impl<const N: usize> MaxNum<N> {
    pub const MAX_VAL: usize = N - 1;
    pub const MAX: MaxNum<N> = MaxNum::new(Self::MAX_VAL);
    pub const MIN: MaxNum<N> = MaxNum::new(0);

    /// num의 값은 num < N을 충족해야 함. 그렇지 않을 경우 panic
    #[must_use]
    pub const fn new(num: usize) -> Self {
        assert!(num < N);
        Self { num }
    }

    /// num의 값은 num < N을 충족해야 함. 그렇지 않을 경우 None
    #[must_use]
    pub const fn new_optional(num: usize) -> Option<Self> {
        if num < N { Some(Self { num }) } else { None }
    }

    /// MaxNum에 value를 더한 값을 반환합니다. N을 초과하거나 오버플로가 발생한 경우 None
    #[must_use]
    pub fn offset(&self, value: i64) -> Option<MaxNum<N>> {
        let this_val: i64 = self.get_value().try_into().ok()?;
        let next_val: i64 = this_val.checked_add(value)?;
        MaxNum::new_optional(next_val.try_into().ok()?)
    }

    /// # Safety
    ///
    /// num의 값은 num < N을 충족해야 함.
    #[must_use]
    pub const unsafe fn new_unchecked(num: usize) -> Self {
        debug_assert!(num < N);
        Self { num }
    }

    #[must_use]
    #[inline]
    pub const fn get_value(&self) -> usize {
        if self.num >= N {
            unsafe { unreachable_unchecked() }
        }
        self.num
    }

    #[must_use]
    #[inline]
    pub const fn get_char(&self) -> char {
        const CHAR_ARR: [char; 35] = [
            '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
            'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
            'Z',
        ];
        CHAR_ARR[self.get_value()]
    }

    /// MaxNum<N>에서 0부터 N - 1까지의 전체 구간에 대한 Iterator를 반환합니다.
    pub fn iter() -> MaxNumIter<N> {
        MaxNumIter { cur: 0, end: N }
    }

    /// MaxNum<N>에서 일부 Range 구간에 대한 Iterator를 반환합니다. Range는 N이상일 수 없습니다.
    pub fn iter_range<I: RangeBounds<MaxNum<N>>>(range: I) -> MaxNumIter<N> {
        // RangeBounds를 다음과 같이 변환
        // .. => 0..N
        // 1.. => 1..N
        // ..3 => 0..3
        // ..=2 => 0..2 + 1
        // 1..3 => 1..3
        // 1..=2 => 1..2 + 1
        let start = match range.start_bound() {
            Bound::Included(i) => i.get_value(),
            Bound::Excluded(e) => {
                let value = e.get_value().checked_add(1).expect("overflow occurred");
                assert!(value < N);
                value
            }
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(i) => {
                let value = i.get_value().checked_add(1).expect("overflow occurred");
                assert!(value < N);
                value
            }
            Bound::Excluded(e) => e.get_value(),
            Bound::Unbounded => N,
        };

        MaxNumIter { cur: start, end }
    }
}

impl<const N: usize> PartialOrd for MaxNum<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for MaxNum<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.num.cmp(&other.num)
    }
}

impl<const N: usize> Clone for MaxNum<N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize> Copy for MaxNum<N> {}

impl<const N: usize> PartialEq for MaxNum<N> {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

impl<const N: usize> Eq for MaxNum<N> {}

impl<const N: usize> Debug for MaxNum<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MaxNum")
            .field("num", &self.get_char())
            .finish()
    }
}

impl<const N: usize> Display for MaxNum<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
    }
}

pub struct MaxNumIter<const N: usize> {
    cur: usize,
    end: usize,
}

impl<const N: usize> Iterator for MaxNumIter<N> {
    type Item = MaxNum<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.end {
            return None;
        }
        let ret = unsafe { Some(MaxNum::new_unchecked(self.cur)) };

        self.cur += 1;
        ret
    }
}
