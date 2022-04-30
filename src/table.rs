pub mod naked;

use hashbrown::HashMap;

use crate::cell::Cell;
use crate::coordinate::Coordinate;
use crate::zone::Zone;

pub struct Table {
    cells: HashMap<Coordinate, Cell>,
    size: usize,
    zone_list: Vec<Zone>,
    zone_coordi: HashMap<Zone, Vec<Coordinate>>,
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
        let zone_list = Self::get_zone_list(&cells, 9);
        let zone_coordi = Self::get_zone_coordi(&cells, 9);
        Table {
            cells,
            size: 9,
            zone_list,
            zone_coordi,
        }
    }

    fn get_zone_list(cells: &HashMap<Coordinate, Cell>, size: usize) -> Vec<Zone> {
        let mut ret: Vec<Zone> = Vec::with_capacity(size);

        for x in 0..size {
            for y in 0..size {
                let coordi = Coordinate { x, y };
                let this_cell = &cells[&coordi];
                for z in this_cell.get_zone() {
                    if !ret.contains(&z) {
                        ret.push(*z);
                    }
                }
            }
        }
        ret
    }

    pub fn get_zone_coordi(
        cells: &hashbrown::HashMap<Coordinate, Cell>,
        size: usize,
    ) -> HashMap<Zone, Vec<Coordinate>> {
        let size: usize = size;
        let mut zone_coordi: HashMap<Zone, Vec<Coordinate>> = HashMap::with_capacity(size * size);
        for x in 0..size {
            for y in 0..size {
                let coordi = Coordinate { x, y };
                let this_cell = &cells[&coordi];
                for z in this_cell.get_zone() {
                    let row: &mut Vec<Coordinate> = zone_coordi
                        .entry(*z)
                        .or_insert_with(|| Vec::with_capacity(size));
                    row.push(coordi);
                }
            }
        }

        zone_coordi
    }

    pub fn get_zone_ref(&self) -> HashMap<Zone, Vec<&Cell>> {
        let size: usize = self.size;
        let mut zone_ref: HashMap<Zone, Vec<&Cell>> = HashMap::with_capacity(size * size);
        for x in 0..size {
            for y in 0..size {
                let coordi = Coordinate { x, y };
                let this_cell = &self.cells[&coordi];
                for z in this_cell.get_zone() {
                    let row: &mut Vec<&Cell> = zone_ref
                        .entry(*z)
                        .or_insert_with(|| Vec::with_capacity(size));
                    row.push(this_cell);
                }
            }
        }

        zone_ref
    }

    #[must_use]
    #[inline]
    pub fn zone_list_iter(&self) -> std::slice::Iter<'_, Zone> {
        self.zone_list.iter()
    }

    pub fn zone_iter(&self, zone: &Zone) -> ZoneIter {
        ZoneIter {
            coordi_iter: self.zone_coordi[zone].iter(),
            cells: &self.cells,
        }
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

pub struct ZoneIter<'a> {
    coordi_iter: std::slice::Iter<'a, Coordinate>,
    cells: &'a HashMap<Coordinate, Cell>,
}

impl<'a> Iterator for ZoneIter<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        match self.coordi_iter.next() {
            Some(value) => Some(&self.cells[value]),
            None => None,
        }
    }
}
