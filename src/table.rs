use std::fmt::Display;

use hashbrown::HashMap;

use crate::cell::Cell;
use crate::coordinate::Coordinate;
use crate::zone::Zone;

pub struct Table {
    cells: HashMap<Coordinate, Cell>,
    size: usize,
}

impl Table {
    /// 9X9 기본 스도쿠 구조입니다.
    pub fn new_default_nine() -> Self {
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

        let mut cells: HashMap<Coordinate, Cell> = HashMap::with_capacity(81);
        for x in 0..9 {
            for y in 0..9 {
                let index = zone[x * 9 + y];
                let this_zone: Vec<Zone> = vec![
                    Zone {
                        z: index,
                        zone_type: crate::zone::ZoneType::Unique,
                    },
                    Zone {
                        z: x + 10,
                        zone_type: crate::zone::ZoneType::Unique,
                    },
                    Zone {
                        z: y + 19,
                        zone_type: crate::zone::ZoneType::Unique,
                    },
                ];
                let cell = Cell::new(9, x, y, this_zone);
                cells.insert(Coordinate { x, y }, cell);
            }
        }
        Table { cells, size: 9 }
    }

    #[must_use]
    #[inline]
    pub fn get_cell(&self) -> &HashMap<Coordinate, Cell> {
        &self.cells
    }

    #[must_use]
    #[inline]
    pub fn get_cell_coordi(&self, coordi: &Coordinate) -> &Cell {
        &self.cells[coordi]
    }

    /// 스도쿠의 가로, 세로 길이입니다.
    #[must_use]
    #[inline]
    pub fn get_size(&self) -> usize {
        self.size
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        for x in 0..self.size {
            for y in 0..self.size {
                let cell = self.get_cell_coordi(&Coordinate { x, y });
                let final_num = cell.chk.borrow().get_final_num();
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
