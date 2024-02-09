use super::{cell::Cell, max_num::MaxNum, table_lock::TableLock, zone::Zone};
use std::pin::Pin;

pub struct Table<const N: usize> {
    pub(crate) cells: Pin<Box<[Cell<N>]>>,
}

impl Table<9> {
    /// 9X9 기본 스도쿠 구조입니다.0
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
        for y in 0..9 {
            let mut row: Vec<Cell<9>> = Vec::with_capacity(9);
            for x in 0..9 {
                let index = zone[x + y * 9];

                let this_zone = vec![
                    Zone::new_unique_from_usize(index),
                    Zone::new_unique_from_usize(x + 10),
                    Zone::new_unique_from_usize(y + 19),
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
        for y in 0..16 {
            let mut row: Vec<Cell<16>> = Vec::with_capacity(16);
            for x in 0..16 {
                let index = zone[x + y * 16];

                let this_zone = vec![
                    Zone::new_unique_from_usize(index),
                    Zone::new_unique_from_usize(x + 17),
                    Zone::new_unique_from_usize(y + 33),
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
        let mut ret: Vec<Cell<N>> = Vec::with_capacity(N * N);
        let mut index_cursor: i32 = -1;

        for row in cells {
            for cell in row {
                let this_index = cell.index as i32;
                assert!(index_cursor < this_index, "Cell은 순서대로 들어와야 함");
                index_cursor = this_index;
                ret.push(cell);
            }
        }

        TableLock::new(Table {
            cells: Box::into_pin(ret.into_boxed_slice()),
        })
    }

    pub fn get_from_coordi(&self, x: MaxNum<N>, y: MaxNum<N>) -> &Cell<N> {
        let index = y.get_value() * N + x.get_value();
        // MaxNum<N>의 값은 N보다 작은 것이 보장됨
        let cell = unsafe { self.cells.get_unchecked(index) };
        debug_assert_eq!(index, cell.index);
        cell
    }
}
