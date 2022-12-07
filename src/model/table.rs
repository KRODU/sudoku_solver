use super::{cell::Cell, zone::Zone};
use std::fmt::Display;

pub struct Table {
    pub cells: Vec<Cell>,
    pub size: usize,
}

impl Table {
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

        let mut cells: Vec<Vec<Cell>> = Vec::with_capacity(9);
        for x in 0..9 {
            let mut row: Vec<Cell> = Vec::with_capacity(9);
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

        let cells = cells.into_iter().flatten().collect::<Vec<_>>();
        Table { cells, size: 9 }
    }

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

        let mut cells: Vec<Vec<Cell>> = Vec::with_capacity(16);
        for x in 0..16 {
            let mut row: Vec<Cell> = Vec::with_capacity(16);
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

        let cells = cells.into_iter().flatten().collect::<Vec<_>>();
        Table { cells, size: 16 }
    }
}

impl<'a> IntoIterator for &'a Table {
    type Item = &'a Cell;
    type IntoIter = CellIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CellIter {
            index: 0,
            t: &self.cells,
        }
    }
}

pub struct CellIter<'a> {
    index: usize,
    t: &'a Vec<Cell>,
}

impl<'a> Iterator for CellIter<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.t.get(self.index);
        self.index += 1;

        ret
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        for x in 0..self.size {
            for y in 0..self.size {
                let cell = &self.cells[x * self.size + y];
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

impl std::fmt::Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        for x in 0..self.size {
            for y in 0..self.size {
                let r1 = self.cells[x * self.size + y].chk.read().unwrap();
                let r2 = other.cells[x * self.size + y].chk.read().unwrap();

                if !r1.is_same_note(&r2) {
                    return false;
                }
            }
        }

        true
    }
}

impl Eq for Table {}
