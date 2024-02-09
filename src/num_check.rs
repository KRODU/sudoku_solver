use crate::model::{
    array_note::ArrayNote,
    array_vector::{ArrayVector, IntoIterArrayVector},
    max_num::MaxNum,
};
use std::ops::Deref;

#[derive(Debug)]
pub struct NumCheck<const N: usize> {
    chk_list: ArrayNote<Option<usize>, N>,
    /// true_list 내의 값 순서는 무작위로 섞일 수 있습니다.
    true_list: ArrayVector<MaxNum<N>, N>,
    true_cnt: usize,
    final_num: Option<MaxNum<N>>,
    /// 완성된 스도쿠에 구멍을 뚫는 경우, 본래 정답을 fixed_final_num에 저장해놓음.
    fixed_final_num: Option<MaxNum<N>>,
}

impl<const N: usize> NumCheck<N> {
    #[must_use]
    pub fn new_with_true() -> NumCheck<N> {
        if N <= 1 {
            panic!("스도쿠 퍼즐의 크기는 최소 2이상이어야 합니다.")
        }

        let mut chk_list = [None; N];
        let mut true_list = ArrayVector::new();

        for (n, item) in chk_list.iter_mut().enumerate() {
            *item = Some(n);
            true_list.push(MaxNum::new(n));
        }

        NumCheck {
            chk_list: ArrayNote::new(chk_list),
            true_list,
            true_cnt: N,
            final_num: None,
            fixed_final_num: None,
        }
    }

    #[must_use]
    pub const fn new_with_false() -> NumCheck<N> {
        if N <= 1 {
            panic!("스도쿠 퍼즐의 크기는 최소 2이상이어야 합니다.")
        }

        let true_list = ArrayVector::new();

        NumCheck {
            chk_list: ArrayNote::new([None; N]),
            true_list,
            true_cnt: N,
            final_num: None,
            fixed_final_num: None,
        }
    }

    /// num의 노트값이 true인지를 반환합니다.
    #[must_use]
    #[inline]
    pub fn get_chk(&self, num: MaxNum<N>) -> bool {
        self.chk_list[num].is_some()
    }

    /// 사용 가능한 노트 중 가장 작은 값을 반환.
    /// 만약 사용 가능한 값이 없다면 None을 반환.
    #[must_use]
    pub fn get_minimum_chk(&self) -> Option<MaxNum<N>> {
        self.chk_list
            .iter()
            .enumerate()
            .find(|(_, b)| b.is_some())
            .map(|(n, _)| MaxNum::new(n))
    }

    #[must_use]
    #[inline]
    pub fn get_true_list(&self) -> &[MaxNum<N>] {
        &self.true_list
    }

    #[inline]
    pub fn set_chk(&mut self, num: MaxNum<N>, chk: bool) {
        if chk {
            self.set_true(num);
        } else {
            self.set_false(num);
        }
    }

    pub fn set_true(&mut self, num: MaxNum<N>) {
        if self.chk_list[num].is_some() {
            return;
        }

        self.true_cnt += 1;
        unsafe {
            // 이미 true인 경우 위에서 return하므로 괜찮음
            self.true_list.push_unchecked(num);
        }
        self.chk_list[num] = Some(self.true_list.len() - 1);

        self.set_to_final_num();
    }

    pub fn set_false(&mut self, num: MaxNum<N>) {
        let Some(remove_index) = self.chk_list[num] else {
            return;
        };

        self.true_cnt -= 1;
        unsafe {
            self.true_list.swap_remove_unchecked(remove_index);
        }
        if let Some(swap_node) = self.true_list.get(remove_index) {
            self.chk_list[*swap_node] = Some(remove_index);
        }
        self.chk_list[num] = None;

        self.set_to_final_num();
    }

    /// chk_list에 포함된 노트만 true이며 그 외엔 false입니다.
    pub fn set_to_chk_list(&mut self, chk_list: &[MaxNum<N>]) {
        self.true_cnt = 0;
        self.chk_list = ArrayNote::new([None; N]);
        self.true_list.clear();

        for &n in chk_list {
            if self.chk_list[n].is_some() {
                continue;
            }

            self.chk_list[n] = Some(self.true_cnt);
            unsafe {
                // true인 경우 위에서 continue하므로 괜찮음
                self.true_list.push_unchecked(n);
            }
            self.true_cnt += 1;
        }

        self.set_to_final_num();
    }

    // 모든 노트를 false로 설정합니다.
    pub fn set_all_false(&mut self) {
        self.true_cnt = 0;
        self.chk_list.set([None; N]);
        self.true_list.clear();
        self.final_num = None;
    }

    /// 하나의 값으로 이 노트를 확정합니다.
    pub fn set_to_value(&mut self, value: MaxNum<N>) {
        self.chk_list.set([None; N]);
        self.chk_list[value] = Some(0);

        self.true_list.clear();
        self.true_list.push(value);

        self.true_cnt = 1;

        self.final_num = Some(value);
    }

    /// 지정된 리스트의 값을 모두 false로 지정합니다.
    pub fn set_to_false_list(&mut self, list: &[MaxNum<N>]) {
        for i in list {
            self.set_false(*i);
        }
    }

    /// 값이 하나만 남은 경우 final_num으로 확정합니다.
    #[inline]
    fn set_to_final_num(&mut self) {
        self.final_num = if self.true_cnt == 1 {
            unsafe { Some(*self.true_list.get_unchecked(0)) }
        } else {
            None
        };
    }

    /// 이 노트의 값이 확정된 경우 true, 그렇지 않으면 false 입니다.
    #[must_use]
    #[inline]
    pub fn is_final_num(&self) -> bool {
        self.true_cnt == 1
    }

    /// true인 목록을 복사하여 반환합니다. 데이터는 무작위로 섞여있을 수 있습니다.
    #[must_use]
    #[inline]
    pub fn clone_chk_list_rand(&self) -> ArrayVector<MaxNum<N>, N> {
        self.true_list.clone()
    }

    /// 최종 값을 반환합니다. 확정되지 않은 경우 None 입니다.
    #[must_use]
    pub fn get_final_num(&self) -> Option<MaxNum<N>> {
        self.final_num
    }

    /// 이 노트와 다른 노트를 비교하여 완전히 같은 경우 true
    #[must_use]
    pub fn is_same_note(&self, num_check: &NumCheck<N>) -> bool {
        for n in MaxNum::<N>::iter() {
            if self.chk_list[n].is_some() != num_check.chk_list[n].is_some() {
                return false;
            }
        }

        true
    }

    /// 이 노트와 다른 노트를 비교하여 서로 겹치는 노트만 반환합니다.
    #[must_use]
    pub fn intersection_num_check(&self, num_check: &NumCheck<N>) -> NumCheck<N> {
        let mut ret = NumCheck::new_with_false();

        for &true_value in &self.true_list {
            if num_check.chk_list[true_value].is_some() {
                ret.set_true(true_value);
            }
        }

        ret
    }

    /// 이 노트와 다른 노트를 비교하여 합집합 노트를 만듭니다.
    pub fn union_note_num_check(&self, num_check: &mut NumCheck<N>) {
        for true_value in &self.true_list {
            num_check.set_true(*true_value);
        }
    }

    #[must_use]
    #[inline]
    pub fn get_true_cnt(&self) -> usize {
        self.true_cnt
    }

    pub fn validater(&self) {
        assert_eq!(self.true_cnt, self.true_list.len());
        assert_eq!(
            self.true_cnt,
            self.chk_list
                .iter()
                .fold(0, |acc, v| if v.is_some() { acc + 1 } else { acc })
        );
        if self.true_cnt == 1 {
            assert_eq!(self.get_minimum_chk().unwrap(), self.final_num.unwrap());
        }

        for n in MaxNum::<N>::iter() {
            if let Some(index) = self.chk_list[n] {
                assert_eq!(self.true_list[index], n);
            }
        }
    }

    pub fn fixed_final_num(&self) -> Option<MaxNum<N>> {
        self.fixed_final_num
    }

    pub fn fixed_final_num_set_none(&mut self) {
        self.fixed_final_num = None;
    }

    pub fn fixed_final_num_set_dup(&mut self) {
        self.fixed_final_num = self.final_num;
    }
}

impl<const N: usize> Default for NumCheck<N> {
    fn default() -> Self {
        Self::new_with_true()
    }
}

impl<const N: usize> IntoIterator for NumCheck<N> {
    type Item = MaxNum<N>;
    type IntoIter = IntoIterArrayVector<MaxNum<N>, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.true_list.into_iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a NumCheck<N> {
    type Item = &'a MaxNum<N>;
    type IntoIter = std::slice::Iter<'a, MaxNum<N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.true_list.iter()
    }
}

impl<const N: usize> Deref for NumCheck<N> {
    type Target = [MaxNum<N>];

    fn deref(&self) -> &Self::Target {
        self.true_list.get_slice()
    }
}
