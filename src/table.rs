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
                *z += 1;
            }
        }

        let mut cells: HashMap<Coordinate, Cell> = HashMap::with_capacity(81);
        for x in 0..9 {
            for y in 0..9 {
                let index = x * 9 + y;
                let cell = Cell::new_single_zone(9, x, y, Zone::new_from_num(zone[index]));
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
