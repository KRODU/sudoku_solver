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

    pub fn next_comb(&mut self) -> Option<&Vec<&T>> {
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
