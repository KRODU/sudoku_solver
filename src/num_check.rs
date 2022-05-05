use hashbrown::HashSet;

pub struct NumCheck {
    chk_list: Vec<bool>,
    true_list: HashSet<usize>,
    false_list: HashSet<usize>,
    ok_cnt: usize,
    size: usize,
    final_num: Option<usize>,
}

impl NumCheck {
    #[must_use]
    pub fn new(size: usize) -> NumCheck {
        let mut ret = NumCheck {
            chk_list: Vec::with_capacity(size),
            true_list: HashSet::with_capacity(size),
            false_list: HashSet::with_capacity(size),
            ok_cnt: size,
            size,
            final_num: None,
        };

        for b in ret.chk_list.iter_mut() {
            *b = true;
        }
        ret
    }

    #[must_use]
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    #[must_use]
    #[inline]
    pub fn get_chk(&self, num: usize) -> bool {
        self.chk_list[num - 1]
    }

    pub fn set_chk(&mut self, num: usize, chk: bool) {
        if self.chk_list[num - 1] == chk {
            return;
        }

        if chk {
            self.ok_cnt += 1;
            self.true_list.insert(num);
            self.false_list.remove(&num);
        } else {
            self.ok_cnt -= 1;
            self.true_list.remove(&num);
            self.false_list.insert(num);
        }

        // 값이 하나만 남은 경우 final_num으로 확정
        self.final_num = if self.ok_cnt == 1 {
            Some(*self.true_list.iter().next().unwrap())
        } else {
            None
        };

        self.chk_list[num - 1] = chk;
    }

    /// chk_list에 포함된 노트만 true이며 그 외엔 false입니다.
    pub fn set_to_chk_list(&mut self, chk_list: &[usize]) {
        self.ok_cnt = 0;
        self.chk_list.fill(false);
        self.true_list.clear();
        self.false_list.clear();

        for n in chk_list {
            self.chk_list[n - 1] = true;
        }

        for (i, b) in self.chk_list.iter().enumerate() {
            if *b {
                self.true_list.insert(i + 1);
                self.ok_cnt += 1;
            } else {
                self.false_list.insert(i + 1);
            }
        }

        // 값이 하나만 남은 경우 final_num으로 확정
        self.final_num = if self.ok_cnt == 1 {
            Some(*self.true_list.iter().next().unwrap())
        } else {
            None
        };
    }

    /// true인 목록을 복사하여 반환합니다.
    pub fn clone_chk_list(&self) -> Vec<usize> {
        let mut ret: Vec<usize> = Vec::with_capacity(self.size);
        for n in &self.true_list {
            ret.push(*n);
        }

        ret
    }

    /// 최종 값을 반환합니다. 확정되지 않은 경우 None 입니다.
    pub fn get_final_num(&self) -> Option<usize> {
        self.final_num
    }

    #[must_use]
    #[inline]
    pub fn get_true_list(&self) -> &HashSet<usize> {
        &self.true_list
    }

    #[must_use]
    #[inline]
    pub fn get_false_list(&self) -> &HashSet<usize> {
        &self.false_list
    }

    #[must_use]
    #[inline]
    pub fn get_true_cnt(&self) -> usize {
        self.ok_cnt
    }

    #[must_use]
    #[inline]
    pub fn get_false_cnt(&self) -> usize {
        self.size - self.ok_cnt
    }
}

impl Clone for NumCheck {
    fn clone(&self) -> Self {
        Self {
            chk_list: self.chk_list.clone(),
            true_list: self.true_list.clone(),
            false_list: self.false_list.clone(),
            ok_cnt: self.ok_cnt,
            size: self.size,
            final_num: self.final_num,
        }
    }
}
