use super::{cell::Cell, table::Table};
use crate::num_check::NumCheck;
use std::{
    fmt::{Debug, Display},
    ops::{Deref, Range},
    pin::Pin,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

pub struct TableLock<const N: usize> {
    table: Pin<Box<Table<N>>>,
    cell_addr_range: Range<usize>,
    lock: RwLock<()>,
}

impl<const N: usize> TableLock<N> {
    pub fn new(t: Table<N>) -> TableLock<N> {
        let t = Box::pin(t);
        let cell_addr_range: Range<usize>;
        unsafe {
            let base_ptr = t.cells.as_ptr() as *const Cell<N>;
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
            table_ref: self,
            _read_guard: self.lock.read().unwrap_or_else(|err| err.into_inner()),
        }
    }

    pub fn write_lock(&self) -> TableLockWriteGuard<N> {
        TableLockWriteGuard {
            table_ref: self,
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
}

impl<const N: usize> Deref for TableLock<N> {
    type Target = Table<N>;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl<const N: usize> Display for TableLock<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let read = self.read_lock();
        let mut ret = String::new();
        for x in 0..N {
            for y in 0..N {
                let cell = &self.cells[x][y];
                let final_num = read.read_from_cell(cell).get_final_num();
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

impl<const N: usize> Debug for TableLock<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl<const N: usize> PartialEq for TableLock<N> {
    fn eq(&self, other: &Self) -> bool {
        let read_self = self.read_lock();
        let read_other = other.read_lock();
        for x in 0..N {
            for y in 0..N {
                let r1 = read_self.read_from_cell(&self.cells[x][y]);
                let r2 = read_other.read_from_cell(&other.cells[x][y]);

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

pub struct TableLockReadGuard<'a, 'b, const N: usize> {
    table_ref: &'a TableLock<N>,
    _read_guard: RwLockReadGuard<'b, ()>,
}

impl<'a, 'b, const N: usize> TableLockReadGuard<'a, 'b, N> {
    pub fn read_from_cell(&self, cell: &Cell<N>) -> &NumCheck<N> {
        self.table_ref.assert_cell_in_table(cell);
        unsafe { &*cell.chk_unsafe.get() }
    }
}

pub struct TableLockWriteGuard<'a, 'b, const N: usize> {
    table_ref: &'a TableLock<N>,
    _write_guard: RwLockWriteGuard<'b, ()>,
}

impl<'a, 'b, const N: usize> TableLockWriteGuard<'a, 'b, N> {
    pub fn write_from_cell(&mut self, cell: &Cell<N>) -> &mut NumCheck<N> {
        self.table_ref.assert_cell_in_table(cell);
        unsafe { &mut *cell.chk_unsafe.get() }
    }
}
