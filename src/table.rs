pub mod naked;

use hashbrown::HashMap;

use crate::cell::Cell;
use crate::coordinate::Coordinate;

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
                let cell = Cell::new_single_zone(9, x, y, zone[index]);
                cells.insert(Coordinate { x, y }, cell);
            }
        }

        Table { cells, size: 9 }
    }

    pub fn get_zone_ref(&self) -> HashMap<usize, Vec<Coordinate>> {
        let size: usize = self.size;
        let mut zone_ref: HashMap<usize, Vec<Coordinate>> = HashMap::with_capacity(size * size);
        for x in 0..size {
            for y in 0..size {
                let coordi = Coordinate { x, y };
                let this_cell = &self.cells[&coordi];
                for z in this_cell.get_zone() {
                    let row: &mut Vec<Coordinate> = zone_ref
                        .entry(*z)
                        .or_insert_with(|| Vec::with_capacity(size));
                    row.push(coordi);
                }
            }
        }

        zone_ref
    }

    #[must_use]
    #[inline]
    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[&Coordinate { x, y }]
    }

    #[must_use]
    #[inline]
    pub fn get_mut_cell(&mut self, x: usize, y: usize) -> &mut Cell {
        self.cells.get_mut(&Coordinate { x, y }).unwrap()
    }

    /// 스도쿠의 가로, 세로 길이입니다.
    pub fn get_size(&self) -> usize {
        self.size
    }
}