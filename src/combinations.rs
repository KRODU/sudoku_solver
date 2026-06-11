pub struct Combination<'a, T> {
    arr: &'a [T],
    result: Vec<&'a T>,
    cursor: Vec<usize>,
    len: usize,
}

impl<'a, T> Combination<'a, T> {
    /// combination 알고리즘을 구현합니다.
    ///
    /// arr: 조합에 사용할 배열
    ///
    /// len: 조합할 갯수
    pub fn new(arr: &'a [T], len: usize) -> Self {
        Self {
            arr,
            result: Vec::new(),
            cursor: Vec::new(),
            len,
        }
    }

    pub fn next_comb(&mut self) -> Option<&[&T]> {
        let len = self.len;

        if self.result.is_empty() {
            // 최초 next
            if self.arr.len() < len || len == 0 {
                return None;
            }

            self.cursor.reserve_exact(len);
            self.result.reserve_exact(len);

            let cursor_ptr = self.cursor.as_mut_ptr();
            let result_ptr = self.result.as_mut_ptr();

            unsafe {
                for i in 0..len {
                    *cursor_ptr.add(i) = i;
                    *result_ptr.add(i) = self.arr.get_unchecked(i);
                }

                self.cursor.set_len(len);
                self.result.set_len(len);
            }

            Some(&self.result)
        } else {
            for (i, rev) in (0..len).rev().enumerate() {
                unsafe {
                    let mut this = self.cursor.get_unchecked(rev) + 1;

                    if this + i < self.arr.len() {
                        for j in rev..len {
                            *self.result.get_unchecked_mut(j) = self.arr.get_unchecked(this);
                            *self.cursor.get_unchecked_mut(j) = this;
                            this += 1;
                        }

                        return Some(&self.result);
                    }
                }
            }

            None
        }
    }
}

pub struct TwoGroupCombination<'a, T> {
    mandatory_group: &'a [T],
    optional_group: &'a [T],
    // 반환할 조합을 저장하는 재사용 버퍼입니다.
    // 호출자는 `next_comb`가 반환한 slice를 다음 호출 전까지만 사용해야 합니다.
    result: Vec<&'a T>,
    // mandatory 그룹에서 선택한 원소의 인덱스입니다.
    // 예를 들어 mandatory_cursor = [0, 2]이면 mandatory_group[0], mandatory_group[2]를 선택한 상태입니다.
    mandatory_cursor: Vec<usize>,
    // optional 그룹에서 선택한 원소의 인덱스입니다. 이 cursor의 길이는 0일 수 있습니다.
    optional_cursor: Vec<usize>,
    len: usize,
    // 현재 분배에서 mandatory 그룹이 몇 개를 선택하는지 저장합니다.
    // optional 그룹 선택 개수는 항상 `len - mandatory_select_len`입니다.
    mandatory_select_len: usize,
    started: bool,
    ended: bool,
}

impl<'a, T> TwoGroupCombination<'a, T> {
    /// 두 그룹에서 조합을 생성합니다.
    ///
    /// 각 결과는 `len`개의 요소를 가집니다. `mandatory_group`에서는 반드시 1개 이상을
    /// 선택하고, `optional_group`에서는 0개 선택을 허용합니다.
    /// `mandatory_group`의 요소가 `optional_group`의 요소보다 먼저 반환됩니다.
    #[must_use]
    pub fn new(mandatory_group: &'a [T], optional_group: &'a [T], len: usize) -> Self {
        Self {
            mandatory_group,
            optional_group,
            result: Vec::new(),
            mandatory_cursor: Vec::new(),
            optional_cursor: Vec::new(),
            len,
            mandatory_select_len: 0,
            started: false,
            ended: false,
        }
    }

    /// 다음 두 그룹 조합을 반환합니다.
    ///
    /// 내부 result 버퍼를 재사용하므로 반환된 slice는 다음 `next_comb` 호출 전까지만
    /// 유효하게 사용해야 합니다.
    pub fn next_comb(&mut self) -> Option<&[&T]> {
        if self.ended {
            return None;
        }

        if !self.started {
            // 첫 호출에서는 가능한 가장 작은 mandatory 선택 개수부터 시작합니다.
            // 이렇게 하면 모든 분배를 mandatory 선택 개수 오름차순으로 순회합니다.
            self.mandatory_select_len = self.first_mandatory_select_len()?;
            self.started = true;
            self.init_current_select_len();
            return Some(&self.result);
        }

        // 같은 mandatory 선택 상태에서는 optional 조합을 먼저 모두 순회합니다.
        // mandatory 결과 영역은 그대로 두고 optional 영역만 갱신하면 됩니다.
        if self.advance_optional() {
            self.write_optional_result();
            return Some(&self.result);
        }

        // optional을 더 전진할 수 없으면 mandatory 조합을 한 칸 전진시키고,
        // 새 mandatory 조합에 대해 optional은 다시 첫 조합부터 시작합니다.
        if self.advance_mandatory() {
            self.reset_optional_cursor();
            self.write_mandatory_result();
            self.write_optional_result();
            return Some(&self.result);
        }

        // 현재 분배의 모든 조합을 소진했으면 mandatory 선택 개수를 늘립니다.
        // 예: 두 그룹 길이가 충분하고 len=3이면 mandatory 1개/optional 2개부터
        // mandatory 3개/optional 0개까지 이동합니다.
        if self.advance_select_len() {
            self.init_current_select_len();
            return Some(&self.result);
        }

        self.ended = true;
        None
    }

    /// 가능한 첫 mandatory 선택 개수를 계산합니다.
    ///
    /// mandatory 그룹에서 1개 이상 선택한다는 제약과 각 그룹의 길이를 모두 반영합니다.
    #[inline]
    fn first_mandatory_select_len(&mut self) -> Option<usize> {
        // mandatory 그룹에서 최소 1개를 선택해야 하므로 전체 선택 개수와 mandatory 그룹이
        // 모두 비어 있지 않아야 합니다.
        if self.len == 0 || self.mandatory_group.is_empty() {
            self.ended = true;
            return None;
        }

        // optional이 감당할 수 없는 나머지는 반드시 mandatory에서 뽑아야 합니다.
        // optional이 전부 감당할 수 있더라도 mandatory는 최소 1개를 뽑습니다.
        let mandatory_min = 1.max(self.len.saturating_sub(self.optional_group.len()));

        // optional을 0개 선택할 수 있으므로 mandatory는 최대 len개까지 뽑을 수 있습니다.
        let mandatory_max = self.mandatory_group.len().min(self.len);

        if mandatory_min > mandatory_max {
            self.ended = true;
            None
        } else {
            Some(mandatory_min)
        }
    }

    /// 현재 조건에서 mandatory 그룹이 선택할 수 있는 최대 개수를 반환합니다.
    #[inline]
    fn mandatory_max_select_len(&self) -> usize {
        self.mandatory_group.len().min(self.len)
    }

    /// 현재 분배에서 optional 그룹이 선택해야 하는 개수를 반환합니다.
    #[inline]
    fn optional_select_len(&self) -> usize {
        self.len - self.mandatory_select_len
    }

    /// 현재 mandatory/optional 선택 개수 분배의 첫 조합으로 내부 상태를 초기화합니다.
    fn init_current_select_len(&mut self) {
        let mandatory_select_len = self.mandatory_select_len;
        let optional_select_len = self.optional_select_len();

        // 새 분배로 이동할 때는 두 커서를 모두 첫 조합인 [0, 1, 2, ...]로 되돌립니다.
        // Vec 자체는 재사용해서 반복 호출 중 할당을 줄입니다.
        self.mandatory_cursor.clear();
        self.optional_cursor.clear();
        self.result.clear();

        if self.mandatory_cursor.capacity() < mandatory_select_len {
            self.mandatory_cursor.reserve_exact(mandatory_select_len);
        }
        if self.optional_cursor.capacity() < optional_select_len {
            self.optional_cursor.reserve_exact(optional_select_len);
        }
        if self.result.capacity() < self.len {
            self.result.reserve_exact(self.len);
        }

        let mandatory_cursor_ptr = self.mandatory_cursor.as_mut_ptr();
        let optional_cursor_ptr = self.optional_cursor.as_mut_ptr();
        let result_ptr = self.result.as_mut_ptr();

        debug_assert!(mandatory_select_len <= self.mandatory_group.len());
        debug_assert!(optional_select_len <= self.optional_group.len());
        debug_assert_eq!(mandatory_select_len + optional_select_len, self.len);
        debug_assert!(self.mandatory_cursor.capacity() >= mandatory_select_len);
        debug_assert!(self.optional_cursor.capacity() >= optional_select_len);
        debug_assert!(self.result.capacity() >= self.len);

        // SAFETY: `mandatory_select_len`과 `optional_select_len`은 가능한 범위에서만
        // 선택되므로 모든 원본 slice 인덱스가 범위 안에 있습니다. Vec들은 clear된 뒤
        // 아래에서 설정할 길이 이상으로 reserve됩니다. `set_len` 전에 `0..len`의 모든
        // 슬롯을 초기화합니다.
        unsafe {
            for i in 0..mandatory_select_len {
                *mandatory_cursor_ptr.add(i) = i;
                *result_ptr.add(i) = self.mandatory_group.get_unchecked(i);
            }

            for i in 0..optional_select_len {
                *optional_cursor_ptr.add(i) = i;
                *result_ptr.add(mandatory_select_len + i) = self.optional_group.get_unchecked(i);
            }

            self.mandatory_cursor.set_len(mandatory_select_len);
            self.optional_cursor.set_len(optional_select_len);
            self.result.set_len(self.len);
        }
    }

    /// 현재 mandatory 조합에 대해 optional 커서를 첫 조합으로 되돌립니다.
    #[inline]
    fn reset_optional_cursor(&mut self) {
        let optional_select_len = self.optional_select_len();
        let optional_cursor_ptr = self.optional_cursor.as_mut_ptr();

        debug_assert_eq!(self.optional_cursor.len(), optional_select_len);

        // mandatory 조합이 바뀌면 optional은 다시 첫 조합부터 붙여야
        // 같은 분배 안의 모든 Cartesian product 조합을 빠짐없이 만들 수 있습니다.
        // SAFETY: `optional_cursor`는 현재 분배에 대해 정확히 `optional_select_len`개의
        // 요소로 초기화되어 있습니다. 이 루프는 초기화된 범위 안에만 쓰며, 각 값은
        // 유효한 optional 인덱스입니다.
        unsafe {
            for i in 0..optional_select_len {
                *optional_cursor_ptr.add(i) = i;
            }
        }
    }

    /// mandatory 커서를 다음 조합으로 전진시킵니다.
    #[inline]
    fn advance_mandatory(&mut self) -> bool {
        Self::advance_cursor(&mut self.mandatory_cursor, self.mandatory_group.len())
    }

    /// optional 커서를 다음 조합으로 전진시킵니다.
    #[inline]
    fn advance_optional(&mut self) -> bool {
        Self::advance_cursor(&mut self.optional_cursor, self.optional_group.len())
    }

    /// 현재 분배를 소진한 뒤 다음 mandatory/optional 선택 개수 분배로 이동합니다.
    fn advance_select_len(&mut self) -> bool {
        let next_mandatory_select_len = self.mandatory_select_len + 1;

        // mandatory 선택 개수를 하나 늘리면 optional 선택 개수는 자동으로 하나 줄어듭니다.
        // optional을 0개 선택하는 마지막 분배까지 허용합니다.
        if next_mandatory_select_len <= self.mandatory_max_select_len() {
            self.mandatory_select_len = next_mandatory_select_len;
            true
        } else {
            false
        }
    }

    /// 주어진 조합 커서를 같은 길이의 다음 조합으로 전진시킵니다.
    #[inline]
    fn advance_cursor(cursor: &mut [usize], arr_len: usize) -> bool {
        let len = cursor.len();
        if len > arr_len {
            return false;
        }

        // 일반적인 조합 생성 방식입니다.
        // 뒤쪽 인덱스부터 증가 가능한 위치를 찾고, 그 뒤는 연속된 값으로 채웁니다.
        // 예: arr_len=5, cursor=[0, 1, 4] -> [0, 2, 3]
        for (i, rev) in (0..len).rev().enumerate() {
            debug_assert!(i < arr_len);
            debug_assert!(cursor.iter().all(|&idx| idx < arr_len));

            // SAFETY: `rev`와 `rev..len`의 모든 `j`는 `cursor`의 유효한
            // 인덱스입니다. cursor는 증가하는 순서로 초기화되고 이 함수로만
            // 전진하므로 저장된 값은 `arr_len`보다 작게 유지됩니다.
            // `this < arr_len - i`는 이번 단계에서 쓰는 가장 큰 값도 `arr_len`보다
            // 작다는 것을 보장합니다.
            unsafe {
                let mut this = *cursor.get_unchecked(rev) + 1;

                if this < arr_len - i {
                    for j in rev..len {
                        *cursor.get_unchecked_mut(j) = this;
                        this += 1;
                    }

                    return true;
                }
            }
        }

        false
    }

    /// mandatory_cursor가 가리키는 mandatory 원소들을 result 앞쪽 영역에 씁니다.
    #[inline]
    fn write_mandatory_result(&mut self) {
        debug_assert_eq!(self.mandatory_cursor.len(), self.mandatory_select_len);
        debug_assert_eq!(self.result.len(), self.len);
        debug_assert!(self.result.len() >= self.mandatory_select_len);
        debug_assert!(
            self.mandatory_cursor
                .iter()
                .all(|&idx| idx < self.mandatory_group.len())
        );

        // SAFETY: `mandatory_cursor`는 선택된 mandatory 요소마다 유효한 mandatory
        // 인덱스를 하나씩 가지고 있습니다. `result`의 길이는 `self.len`이고 mandatory
        // 영역은 `0..mandatory_select_len`이므로 모든 쓰기가 범위 안에 있습니다.
        unsafe {
            for i in 0..self.mandatory_select_len {
                *self.result.get_unchecked_mut(i) = self
                    .mandatory_group
                    .get_unchecked(*self.mandatory_cursor.get_unchecked(i));
            }
        }
    }

    /// optional_cursor가 가리키는 optional 원소들을 result 뒤쪽 영역에 씁니다.
    #[inline]
    fn write_optional_result(&mut self) {
        let optional_select_len = self.optional_select_len();

        debug_assert_eq!(self.optional_cursor.len(), optional_select_len);
        debug_assert_eq!(
            self.mandatory_select_len + optional_select_len,
            self.result.len()
        );
        debug_assert!(
            self.optional_cursor
                .iter()
                .all(|&idx| idx < self.optional_group.len())
        );

        // SAFETY: `optional_cursor`는 선택된 optional 요소마다 유효한 optional
        // 인덱스를 하나씩 가지고 있습니다. optional 영역은 `mandatory_select_len`에서
        // 시작하고 `optional_select_len`개의 요소를 가지므로 정확히 `result.len()`까지 닿습니다.
        unsafe {
            for i in 0..optional_select_len {
                *self.result.get_unchecked_mut(self.mandatory_select_len + i) = self
                    .optional_group
                    .get_unchecked(*self.optional_cursor.get_unchecked(i));
            }
        }
    }
}

#[cfg(test)]
fn make_comb_str(mut comb_iter: Combination<usize>) -> String {
    let mut print_str: String = String::new();

    while let Some(arr) = comb_iter.next_comb() {
        for c in arr {
            print_str.push_str(c.to_string().as_str());
            print_str.push_str(",");
        }
        assert_eq!(print_str.pop().unwrap(), ',');
        print_str.push('\t');
    }
    if let Some(str) = print_str.pop() {
        assert_eq!(str, '\t');
    }

    print_str
}

/// `TwoGroupCombination` 결과를 테스트 비교용 문자열로 변환합니다.
#[cfg(test)]
fn make_two_group_comb_str(mut comb_iter: TwoGroupCombination<usize>) -> String {
    let mut print_str: String = String::new();

    while let Some(arr) = comb_iter.next_comb() {
        for c in arr {
            print_str.push_str(c.to_string().as_str());
            print_str.push_str(",");
        }
        assert_eq!(print_str.pop().unwrap(), ',');
        print_str.push('\t');
    }
    if let Some(str) = print_str.pop() {
        assert_eq!(str, '\t');
    }

    print_str
}

#[test]
fn combination_test() {
    let v = vec![1, 2, 3, 4, 5];

    assert_eq!(
        make_comb_str(Combination::new(&v, 3)),
        "1,2,3	1,2,4	1,2,5	1,3,4	1,3,5	1,4,5	2,3,4	2,3,5	2,4,5	3,4,5"
    );

    assert_eq!(make_comb_str(Combination::new(&v, 1)), "1	2	3	4	5");
    assert_eq!(make_comb_str(Combination::new(&v, 5)), "1,2,3,4,5");
    assert_eq!(make_comb_str(Combination::new(&v, 0)), "");
    assert_eq!(make_comb_str(Combination::new(&v, 6)), "");
    assert_eq!(make_comb_str(Combination::new(&vec![], 0)), "");
    assert_eq!(make_comb_str(Combination::new(&vec![], 5)), "");
    assert_eq!(make_comb_str(Combination::new(&vec![1], 1)), "1");
}

/// 두 그룹 조합의 분배 순회와 경계 조건을 검증합니다.
#[test]
fn two_group_combination_test() {
    let mandatory_values = vec![1, 2, 3];
    let optional_values = vec![4, 5];

    assert_eq!(
        make_two_group_comb_str(TwoGroupCombination::new(
            &mandatory_values,
            &optional_values,
            3
        )),
        "1,4,5\t2,4,5\t3,4,5\t1,2,4\t1,2,5\t1,3,4\t1,3,5\t2,3,4\t2,3,5\t1,2,3"
    );

    assert_eq!(
        make_two_group_comb_str(TwoGroupCombination::new(
            &mandatory_values,
            &optional_values,
            2
        )),
        "1,4\t1,5\t2,4\t2,5\t3,4\t3,5\t1,2\t1,3\t2,3"
    );
    assert_eq!(
        make_two_group_comb_str(TwoGroupCombination::new(
            &mandatory_values,
            &optional_values,
            4
        )),
        "1,2,4,5\t1,3,4,5\t2,3,4,5\t1,2,3,4\t1,2,3,5"
    );
    assert_eq!(
        make_two_group_comb_str(TwoGroupCombination::new(
            &mandatory_values,
            &optional_values,
            5
        )),
        "1,2,3,4,5"
    );

    assert_eq!(
        make_two_group_comb_str(TwoGroupCombination::new(
            &mandatory_values,
            &optional_values,
            1
        )),
        "1\t2\t3"
    );
    assert_eq!(
        make_two_group_comb_str(TwoGroupCombination::new(
            &mandatory_values,
            &optional_values,
            6
        )),
        ""
    );
    assert_eq!(
        make_two_group_comb_str(TwoGroupCombination::new(&[], &optional_values, 2)),
        ""
    );
    assert_eq!(
        make_two_group_comb_str(TwoGroupCombination::new(&mandatory_values, &[], 2)),
        "1,2\t1,3\t2,3"
    );
    assert_eq!(
        make_two_group_comb_str(TwoGroupCombination::new(
            &mandatory_values,
            &optional_values,
            0
        )),
        ""
    );
}
