use hashbrown::HashSet;

#[derive(Debug)]
pub struct NumCheck {
    chk_list: Vec<(usize, bool)>,
    /// 값은 1부터 들어갑니다.
    true_list: HashSet<usize>,
    true_cnt: usize,
    final_num: Option<usize>,
}

impl NumCheck {
    #[must_use]
    pub fn new(size: usize) -> NumCheck {
        if size <= 1 {
            panic!("스도쿠 퍼즐의 크기는 최소 2이상이어야 합니다.")
        }

        let mut ret = NumCheck {
            chk_list: Vec::with_capacity(size),
            true_list: HashSet::with_capacity(size),
            true_cnt: size,
            final_num: None,
        };

        for n in 1..=size {
            ret.chk_list.push((n, true));
            ret.true_list.insert(n);
        }
        ret
    }

    #[must_use]
    pub fn get_chk(&self, num: usize) -> bool {
        self.chk_list[num - 1].1
    }

    /// 사용 가능한 노트 중 가장 작은 값을 반환.
    /// 만약 사용 가능한 값이 없다면 None을 반환.
    #[must_use]
    pub fn get_minimum_chk(&self) -> Option<usize> {
        self.chk_list.iter().find(|(_, b)| *b).map(|(n, _)| *n)
    }

    pub fn set_chk(&mut self, num: usize, chk: bool) {
        if self.chk_list[num - 1].1 == chk {
            return;
        }

        if chk {
            self.true_cnt += 1;
            self.true_list.insert(num);
        } else {
            self.true_cnt -= 1;
            self.true_list.remove(&num);
        }

        self.chk_list[num - 1].1 = chk;
        self.set_to_final_num();
    }

    /// chk_list에 포함된 노트만 true이며 그 외엔 false입니다.
    pub fn set_to_chk_list(&mut self, chk_list: &Vec<usize>) {
        self.true_cnt = 0;
        self.chk_list.iter_mut().for_each(|(_, b)| *b = false);
        self.true_list.clear();

        for n in chk_list {
            self.chk_list[n - 1].1 = true;
        }

        for (n, b) in &self.chk_list {
            if *b {
                self.true_list.insert(*n);
                self.true_cnt += 1;
            }
        }

        self.set_to_final_num();
    }

    /// 하나의 값으로 이 노트를 확정합니다.
    pub fn set_to_value(&mut self, value: usize) {
        self.chk_list.iter_mut().for_each(|(_, b)| *b = false);
        self.chk_list[value - 1].1 = true;

        self.true_list.clear();
        self.true_list.insert(value);

        self.true_cnt = 1;

        self.set_to_final_num();
    }

    /// 지정된 리스트의 값을 모두 false로 지정합니다.
    pub fn set_to_false_list(&mut self, list: &Vec<usize>) {
        for i in list {
            self.set_chk(*i, false);
        }
    }

    /// 값이 하나만 남은 경우 final_num으로 확정합니다.
    fn set_to_final_num(&mut self) {
        self.final_num = if self.true_cnt == 1 {
            Some(*self.true_list.iter().next().unwrap())
        } else {
            None
        };
    }

    /// 이 노트의 값이 확정된 경우 true, 그렇지 않으면 false 입니다.
    pub fn is_final_num(&self) -> bool {
        self.true_cnt == 1
    }

    /// true인 목록을 복사하여 HashSet으로 반환합니다.
    pub fn clone_chk_list_hash(&self) -> HashSet<usize> {
        self.true_list.iter().copied().collect()
    }

    /// true인 목록을 복사하여 Vec으로 반환합니다.
    pub fn clone_chk_list_vec(&self) -> Vec<usize> {
        self.chk_list
            .iter()
            .filter(|(_, b)| *b)
            .map(|(n, _)| *n)
            .collect()
    }

    /// 최종 값을 반환합니다. 확정되지 않은 경우 None 입니다.
    pub fn get_final_num(&self) -> Option<usize> {
        self.final_num
    }

    /// 이 노트와 다른 노트를 비교하여 완전히 같은 경우 true
    pub fn is_same_note(&self, num_check: &NumCheck) -> bool {
        if self.chk_list.len() != num_check.chk_list.len() {
            return false;
        }

        for n in 0..self.chk_list.len() {
            if self.chk_list[n] != num_check.chk_list[n] {
                return false;
            }
        }

        true
    }

    /// 이 노트와 다른 노트를 비교하여 서로 겹치는 노트만 반환합니다.
    pub fn intersection_note(&self, num_check: &HashSet<usize>) -> HashSet<usize> {
        let mut ret = HashSet::with_capacity(self.true_cnt);
        for true_value in &self.true_list {
            if num_check.contains(true_value) {
                ret.insert(*true_value);
            }
        }

        ret
    }

    /// 이 노트와 다른 노트를 비교하여 합집합 노트를 만듭니다.
    pub fn union_note(&self, num_check: &mut HashSet<usize>) {
        for true_value in &self.true_list {
            num_check.insert(*true_value);
        }
    }

    #[must_use]
    pub fn get_true_list(&self) -> &HashSet<usize> {
        &self.true_list
    }

    #[must_use]
    pub fn get_true_cnt(&self) -> usize {
        self.true_cnt
    }
}
