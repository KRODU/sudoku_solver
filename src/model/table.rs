use std::marker::PhantomPinned;

use super::{cell::Cell, table_lock::TableLock, zone::Zone};

pub struct Table<const N: usize> {
    pub cells: [[Cell<N>; N]; N],
    _pin: PhantomPinned,
}

impl Table<9> {
    /// 9X9 기본 스도쿠 구조입니다.
    pub fn new_default_9() -> TableLock<9> {
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
                let cell = Cell::new(x, y, this_zone);

                row.push(cell);
            }
            cells.push(row);
        }

        Table::new_with_vec_cells(cells)
    }
}

impl Table<16> {
    /// 16X16 스도쿠 구조입니다.
    pub fn new_default_16() -> TableLock<16> {
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

                let cell = Cell::new(x, y, this_zone);
                row.push(cell);
            }
            cells.push(row);
        }

        Table::new_with_vec_cells(cells)
    }
}

impl<const N: usize> Table<N> {
    pub fn new_with_vec_cells(cells: Vec<Vec<Cell<N>>>) -> TableLock<N> {
        let cells = cells
            .into_iter()
            .map(|c| TryInto::<[Cell<N>; N]>::try_into(c).expect("SIZE_NOT_SAME"))
            .collect::<Vec<_>>();

        let cells = TryInto::<[[Cell<N>; N]; N]>::try_into(cells).expect("SIZE_NOT_SAME");

        TableLock::new(Table {
            cells,
            _pin: PhantomPinned,
        })
    }
}
