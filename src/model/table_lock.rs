use super::{cell::Cell, max_num::MaxNum, table::Table};
use crate::num_check::NumCheck;
use std::{
    fmt::{Debug, Display},
    ops::{Deref, Range},
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

pub struct TableLock<const N: usize> {
    table: Table<N>,
    cell_addr_range: Range<usize>,
    lock: RwLock<()>,
}

impl<const N: usize> TableLock<N> {
    pub fn new(t: Table<N>) -> TableLock<N> {
        assert_eq!(t.cells.len(), N * N);

        let cell_addr_range: Range<usize>;
        unsafe {
            let base_ptr = t.cells.as_ptr();
            let end_ptr = base_ptr.add(N * N);
            cell_addr_range = base_ptr as usize..end_ptr as usize;
        }

        TableLock {
            table: t,
            cell_addr_range,
            lock: RwLock::new(()),
        }
    }

    pub fn read_lock(&self) -> TableLockReadGuard<N> {
        TableLockReadGuard {
            table_lock: self,
            _read_guard: self.lock.read().unwrap_or_else(|err| err.into_inner()),
        }
    }

    pub fn write_lock(&self) -> TableLockWriteGuard<N> {
        TableLockWriteGuard {
            table_lock: self,
            _write_guard: self.lock.write().unwrap_or_else(|err| err.into_inner()),
        }
    }

    /// cell은 table에 속해있어야만 함. 그렇지 않으면 panic 발생.
    ///
    /// 다른 table에 속한 cell을 여기서 read, write하는 걸 방지하기 위함.
    fn assert_cell_in_table(&self, cell: &Cell<N>) {
        let cell_addr = cell as *const _ as usize;
        assert!(
            self.cell_addr_range.contains(&cell_addr),
            "assert_cell_in_table FAIL",
        );
    }

    pub fn table_num_chk_validater(&self) -> bool {
        let read = self.read_lock();

        for check in self {
            read.read_from_cell(check).validater();
        }

        true
    }

    pub fn get_cell_from_coordinate(&self, x: MaxNum<N>, y: MaxNum<N>) -> &Cell<N> {
        unsafe {
            // MaxNum<N>의 값은 N보다 작은 것이 보장됨
            self.table
                .cells
                .get_unchecked(x.get_value() * N + y.get_value())
        }
    }

    pub fn note_fmt(&self) -> String {
        let rec_size = (N as f64).sqrt().ceil() as usize;
        let mut row_string: Vec<String> = Vec::with_capacity(N * 2);
        let mut row_string_cursor = 0_usize;

        let read = self.read_lock();

        for x in MaxNum::<N>::iter() {
            for _ in 0..rec_size {
                row_string.push(String::with_capacity(N * 2));
            }

            for y in MaxNum::<N>::iter() {
                let cell = read.read_from_coordinate(x, y);
                let mut write_row_cursor = 0;
                for n in MaxNum::<N>::iter() {
                    if cell.get_chk(n) {
                        row_string[row_string_cursor + write_row_cursor].push(n.get_char());
                    } else {
                        row_string[row_string_cursor + write_row_cursor].push(' ');
                    }

                    if (n.get_value() + 1) % rec_size == 0 {
                        write_row_cursor += 1;
                    }
                }

                for row in row_string.iter_mut().skip(row_string_cursor).take(rec_size) {
                    row.push('|');
                }
            }
            row_string.push("-".repeat(N * rec_size + N));
            row_string_cursor += rec_size + 1;
        }

        let mut ret = String::with_capacity(N * N * N * 2);

        for row in row_string {
            ret.push_str(&row);
            ret.push('\n');
        }

        ret
    }
}

impl<const N: usize> Deref for TableLock<N> {
    type Target = [Cell<N>];

    fn deref(&self) -> &Self::Target {
        &self.table.cells
    }
}

impl<const N: usize> Display for TableLock<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let read = self.read_lock();
        let mut ret = String::new();
        for x in MaxNum::<N>::iter() {
            for y in MaxNum::<N>::iter() {
                let final_num = read.read_from_coordinate(x, y).get_final_num();
                if let Some(num) = final_num {
                    ret.push_str((num.get_value() + 1).to_string().as_str());
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

impl<const N: usize> Debug for TableLock<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.note_fmt(), f)
    }
}

impl<const N: usize> PartialEq for TableLock<N> {
    fn eq(&self, other: &Self) -> bool {
        let read_self = self.read_lock();
        let read_other = other.read_lock();
        for x in MaxNum::<N>::iter() {
            for y in MaxNum::<N>::iter() {
                let r1 = read_self.read_from_coordinate(x, y);
                let r2 = read_other.read_from_coordinate(x, y);

                if !r1.is_same_note(r2) {
                    return false;
                }
            }
        }

        true
    }
}

impl<const N: usize> Eq for TableLock<N> {}

impl<'a, const N: usize> IntoIterator for &'a TableLock<N> {
    type Item = &'a Cell<N>;
    type IntoIter = CellIter<'a, N>;

    fn into_iter(self) -> Self::IntoIter {
        CellIter { index: 0, t: self }
    }
}

pub struct CellIter<'a, const N: usize> {
    index: usize,
    t: &'a [Cell<N>],
}

impl<'a, const N: usize> Iterator for CellIter<'a, N> {
    type Item = &'a Cell<N>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.t.get(self.index);
        self.index += 1;
        ret
    }
}

pub struct TableLockReadGuard<'a, 'b, const N: usize> {
    table_lock: &'a TableLock<N>,
    _read_guard: RwLockReadGuard<'b, ()>,
}

impl<'a, 'b, const N: usize> TableLockReadGuard<'a, 'b, N> {
    pub fn read_from_cell(&self, cell: &Cell<N>) -> &NumCheck<N> {
        self.table_lock.assert_cell_in_table(cell);
        unsafe { &*cell.chk_unsafe.get() }
    }

    pub fn read_from_coordinate(&self, x: MaxNum<N>, y: MaxNum<N>) -> &NumCheck<N> {
        unsafe {
            &*self
                .table_lock
                .get_cell_from_coordinate(x, y)
                .chk_unsafe
                .get()
        }
    }

    /// ReadLock을 WriteLock으로 업그레이드합니다. ReadLock을 Drop한 뒤 다시 WriteLock을 얻는 것과 같습니다.
    pub fn upgrade_to_write<'c, 'd>(self) -> TableLockWriteGuard<'c, 'd, N>
    where
        'a: 'c + 'd,
    {
        drop(self._read_guard);
        self.table_lock.write_lock()
    }
}

pub struct TableLockWriteGuard<'a, 'b, const N: usize> {
    table_lock: &'a TableLock<N>,
    _write_guard: RwLockWriteGuard<'b, ()>,
}

impl<'a, 'b, const N: usize> TableLockWriteGuard<'a, 'b, N> {
    pub fn read_from_cell(&self, cell: &Cell<N>) -> &NumCheck<N> {
        self.table_lock.assert_cell_in_table(cell);
        unsafe { &*cell.chk_unsafe.get() }
    }

    pub fn read_from_coordinate(&self, x: MaxNum<N>, y: MaxNum<N>) -> &NumCheck<N> {
        unsafe {
            &*self
                .table_lock
                .get_cell_from_coordinate(x, y)
                .chk_unsafe
                .get()
        }
    }

    pub fn write_from_cell(&mut self, cell: &Cell<N>) -> &mut NumCheck<N> {
        self.table_lock.assert_cell_in_table(cell);
        unsafe { &mut *cell.chk_unsafe.get() }
    }

    pub fn write_from_coordinate(&mut self, x: MaxNum<N>, y: MaxNum<N>) -> &mut NumCheck<N> {
        unsafe {
            &mut *self
                .table_lock
                .get_cell_from_coordinate(x, y)
                .chk_unsafe
                .get()
        }
    }
}
