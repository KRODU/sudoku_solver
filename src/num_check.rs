pub struct NumCheck {
    chk_list: Vec<bool>,
    ok_cnt: usize,
    size: usize,
}

impl NumCheck {
    #[must_use]
    pub fn new(size: usize) -> NumCheck {
        let mut ret = NumCheck {
            chk_list: Vec::with_capacity(size),
            ok_cnt: size,
            size,
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
        self.chk_list[num]
    }

    pub fn set_chk(&mut self, num: usize, chk: bool) {
        if self.chk_list[num] != chk {
            if chk {
                self.ok_cnt += 1;
            } else {
                self.ok_cnt -= 1;
            }
            self.chk_list[num] = chk;
        }
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
