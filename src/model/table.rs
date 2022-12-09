use super::{cell::Cell, zone::Zone};
use std::fmt::Display;

pub struct Table<const N: usize> {
    pub cells: [[Cell<N>; N]; N],
}

impl Table<9> {
    /// 9X9 기본 스도쿠 구조입니다.
    pub fn new_default_9() -> Self {
        let mut zone: Vec<usize> = Vec::with_capacity(81);
        let mut zone_row = [1, 1, 1, 2, 2, 2, 3, 3, 3];
        for _ in 0..3 {
            for _ in 0..3 {
                zone.extend_from_slice(&zone_row);
            }

            for z in zone_row.iter_mut() {
                *z += 3;
            }
        }

        let mut cells: Vec<Vec<Cell<9>>> = Vec::with_capacity(9);
        for x in 0..9 {
            let mut row: Vec<Cell<9>> = Vec::with_capacity(9);
            for y in 0..9 {
                let index = zone[x * 9 + y];

                let this_zone = vec![
                    Zone::new_unique_from_num(index),
                    Zone::new_unique_from_num(x + 10),
                    Zone::new_unique_from_num(y + 19),
                ];
                let cell = Cell::new(9, x, y, this_zone);

                row.push(cell);
            }
            cells.push(row);
        }

        let cells = cells
            .into_iter()
            .map(|c| TryInto::<[Cell<9>; 9]>::try_into(c).expect("SIZE_NOT_SAME"))
            .collect::<Vec<_>>();
        let cells = TryInto::<[[Cell<9>; 9]; 9]>::try_into(cells).expect("SIZE_NOT_SAME");
        Table { cells }
    }
}

impl Table<16> {
    /// 16X16 스도쿠 구조입니다.
    pub fn new_default_16() -> Self {
        let mut zone: Vec<usize> = Vec::with_capacity(256);
        let mut zone_row = [1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
        for _ in 0..4 {
            for _ in 0..4 {
                zone.extend_from_slice(&zone_row);
            }

            for z in zone_row.iter_mut() {
                *z += 4;
            }
        }

        let mut cells: Vec<Vec<Cell<16>>> = Vec::with_capacity(16);
        for x in 0..16 {
            let mut row: Vec<Cell<16>> = Vec::with_capacity(16);
            for y in 0..16 {
                let index = zone[x * 16 + y];

                let this_zone = vec![
                    Zone::new_unique_from_num(index),
                    Zone::new_unique_from_num(x + 17),
                    Zone::new_unique_from_num(y + 33),
                ];

                let cell = Cell::new(16, x, y, this_zone);
                row.push(cell);
            }
            cells.push(row);
        }

        let cells = cells
            .into_iter()
            .map(|c| TryInto::<[Cell<16>; 16]>::try_into(c).expect("SIZE_NOT_SAME"))
            .collect::<Vec<_>>();
        let cells = TryInto::<[[Cell<16>; 16]; 16]>::try_into(cells).expect("SIZE_NOT_SAME");
        Table { cells }
    }
}

impl<const N: usize> Table<N> {
    pub fn num_check_validater(&self) -> bool {
        for check in self {
            check.chk.read().unwrap().validater();
        }

        true
    }
}

impl<'a, const N: usize> IntoIterator for &'a Table<N> {
    type Item = &'a Cell<N>;
    type IntoIter = CellIter<'a, N>;

    fn into_iter(self) -> Self::IntoIter {
        CellIter {
            x: 0,
            y: 0,
            t: &self.cells,
        }
    }
}

pub struct CellIter<'a, const N: usize> {
    x: usize,
    y: usize,
    t: &'a [[Cell<N>; N]; N],
}

impl<'a, const N: usize> Iterator for CellIter<'a, N> {
    type Item = &'a Cell<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= N {
            self.x = 0;
            self.y += 1;
        }

        let ret = if self.y >= N {
            None
        } else {
            Some(&self.t[self.x][self.y])
        };

        self.x += 1;

        ret
    }
}

impl<const N: usize> Display for Table<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        for x in 0..N {
            for y in 0..N {
                let cell = &self.cells[x][y];
                let final_num = cell.chk.read().unwrap().get_final_num();
                if let Some(num) = final_num {
                    ret.push_str(num.to_string().as_str());
                } else {
                    ret.push(' ');
                }
                ret.push('\t');
            }
            ret.pop();
            ret.push('\n');
        }
        ret.pop();

        write!(f, "{}", ret)
    }
}

impl<const N: usize> std::fmt::Debug for Table<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl<const N: usize> PartialEq for Table<N> {
    fn eq(&self, other: &Self) -> bool {
        for x in 0..N {
            for y in 0..N {
                let r1 = self.cells[x][y].chk.read().unwrap();
                let r2 = other.cells[x][y].chk.read().unwrap();

                if !r1.is_same_note(&r2) {
                    return false;
                }
            }
        }

        true
    }
}

impl<const N: usize> Eq for Table<N> {}
