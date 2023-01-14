use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::hint::unreachable_unchecked;
use std::ops::{Bound, RangeBounds};

/// MaxNum의 값은 0 <= value < N를 보장함.
pub struct MaxNum<const N: usize> {
    num: usize,
}

impl<const N: usize> MaxNum<N> {
    /// num의 값은 num < N을 충족해야 함. 그렇지 않을 경우 panic
    pub fn new(num: usize) -> Self {
        assert!(num < N);
        Self { num }
    }

    /// num의 값은 num < N을 충족해야 함. 그렇지 않을 경우 None
    pub fn new_optional(num: usize) -> Option<Self> {
        if num < N {
            Some(Self { num })
        } else {
            None
        }
    }

    /// MaxNum에 value를 더한 값을 반환합니다. N을 초과하게 될 경우 None
    pub fn offset(&self, value: i64) -> Option<MaxNum<N>> {
        MaxNum::new_optional((self.get_value() as i64 + value) as usize)
    }

    /// # Safety
    ///
    /// num의 값은 num < N을 충족해야 함.
    pub unsafe fn new_unchecked(num: usize) -> Self {
        debug_assert!(num < N);
        Self { num }
    }

    #[inline]
    pub fn get_value(&self) -> usize {
        if self.num >= N {
            unsafe { unreachable_unchecked() }
        }
        self.num
    }

    pub fn get_char(&self) -> char {
        const CHAR_ARR: [char; 27] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G',
            'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
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
        self.num.partial_cmp(&other.num)
    }
}

impl<const N: usize> Ord for MaxNum<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.num.cmp(&other.num)
    }
}

impl<const N: usize> Clone for MaxNum<N> {
    fn clone(&self) -> Self {
        Self { num: self.num }
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

impl<const N: usize> Hash for MaxNum<N> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.num.hash(state);
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
