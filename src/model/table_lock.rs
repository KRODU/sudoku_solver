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

    #[inline]
    pub fn read_lock(&self) -> TableLockReadGuard<N> {
        TableLockReadGuard {
            table_lock: self,
            _read_guard: self.lock.read().unwrap_or_else(|err| err.into_inner()),
        }
    }

    #[inline]
    pub fn write_lock(&self) -> TableLockWriteGuard<N> {
        TableLockWriteGuard {
            table_lock: self,
            _write_guard: self.lock.write().unwrap_or_else(|err| err.into_inner()),
        }
    }

    /// cell은 table에 속해있어야만 함. 그렇지 않으면 panic 발생.
    ///
    /// 다른 table에 속한 cell을 여기서 read, write하는 걸 방지하기 위함.
    #[inline]
    fn assert_cell_in_table(&self, cell: &Cell<N>) {
        let cell_addr = cell as *const _ as usize;
        assert!(
            self.cell_addr_range.contains(&cell_addr),
            "assert_cell_in_table FAIL",
        );
    }

    /// 테이블 전체 NumCheck가 올바른지 검사합니다. 디버그 빌드에서만 검사합니다.
    #[cfg(debug_assertions)]
    pub fn table_debug_validater(&self) {
        let read = self.read_lock();

        for check in self {
            read.read_from_cell(check).validater();
        }
    }

    /// 테이블 전체 NumCheck가 올바른지 검사합니다. 릴리즈 빌드에서는 검사를 생략합니다.
    #[cfg(not(debug_assertions))]
    pub fn table_debug_validater(&self) {}

    #[must_use]
    #[inline]
    pub fn get_cell_from_coordinate(&self, x: MaxNum<N>, y: MaxNum<N>) -> &Cell<N> {
        unsafe {
            // MaxNum<N>의 값은 N보다 작은 것이 보장됨
            self.table
                .cells
                .get_unchecked(x.get_value() + y.get_value() * N)
        }
    }

    pub fn note_fmt(&self) -> String {
        let rec_size = (N as f64).sqrt().ceil() as usize;
        let mut row_string: Vec<String> = Vec::with_capacity(N * 2);
        let mut row_string_cursor = 0_usize;

        let read = self.read_lock();

        for y in MaxNum::<N>::iter() {
            for _ in 0..rec_size {
                row_string.push(String::with_capacity(N * 2));
            }

            for x in MaxNum::<N>::iter() {
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
        let ret = read.make_string(NumCheck::final_num);
        write!(f, "{ret}")
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
    type IntoIter = TableCellIter<'a, N>;

    fn into_iter(self) -> Self::IntoIter {
        TableCellIter { index: 0, t: self }
    }
}

pub struct TableCellIter<'a, const N: usize> {
    index: usize,
    t: &'a [Cell<N>],
}

impl<'a, const N: usize> Iterator for TableCellIter<'a, N> {
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
    #[must_use]
    #[inline]
    pub fn read_from_cell(&self, cell: &Cell<N>) -> &'a NumCheck<N> {
        self.table_lock.assert_cell_in_table(cell);
        unsafe { &*cell.chk_unsafe.get() }
    }

    /// # Safety
    ///
    /// Table 내에 속한 Cell로만 이 함수를 호출해야 함.
    #[must_use]
    #[inline]
    pub unsafe fn read_from_cell_unchecked(&self, cell: &Cell<N>) -> &'a NumCheck<N> {
        unsafe { &*cell.chk_unsafe.get() }
    }

    #[must_use]
    #[inline]
    pub fn read_from_coordinate(&self, x: MaxNum<N>, y: MaxNum<N>) -> &'a NumCheck<N> {
        unsafe {
            &*self
                .table_lock
                .get_cell_from_coordinate(x, y)
                .chk_unsafe
                .get()
        }
    }

    /// ReadLock을 WriteLock으로 업그레이드합니다. ReadLock을 Drop한 뒤 다시 WriteLock을 얻는 것과 같습니다.
    #[must_use]
    #[inline]
    pub fn upgrade_to_write<'c, 'd>(self) -> TableLockWriteGuard<'c, 'd, N>
    where
        'a: 'c + 'd,
    {
        drop(self._read_guard);
        self.table_lock.write_lock()
    }

    fn make_string(&self, final_fn: impl Fn(&NumCheck<N>) -> Option<MaxNum<N>>) -> String {
        let mut some = 0u32;
        let mut none = 0u32;
        let mut ret = String::with_capacity(N * N * 5);

        let table = &self.table_lock.table;

        // 첫 줄
        ret.push('╔');
        ret.push('═');

        for x in MaxNum::<N>::iter() {
            if x == MaxNum::<N>::MAX {
                ret.push('╗');
            } else {
                let next_cell = table.get_from_coordi(x.offset(1).unwrap(), MaxNum::<N>::MIN);
                let this_cell = table.get_from_coordi(x, MaxNum::<N>::MIN);

                if next_cell.rep_zone() == this_cell.rep_zone() {
                    ret.push('═');
                } else {
                    ret.push('╦');
                }

                ret.push('═');
            }
        }

        for y in MaxNum::<N>::iter() {
            // 첫 줄에선 이 코드를 실행하지 않음.
            if y != MaxNum::<N>::MIN {
                for x in MaxNum::<N>::iter() {
                    let this_cell = table.get_from_coordi(x, y.offset(-1).unwrap());
                    let next_y = table.get_from_coordi(x, y);
                    if x == MaxNum::<N>::MIN {
                        if this_cell.rep_zone() == next_y.rep_zone() {
                            ret.push('║');
                            ret.push('╌');
                            ret.push('╌');
                        } else {
                            ret.push('╠');
                            ret.push('═');
                            ret.push('═');
                        }

                        continue;
                    }
                    if x == MaxNum::<N>::MAX {
                        if this_cell.rep_zone() == next_y.rep_zone() {
                            ret.push('╌');
                            ret.push('║');
                        } else {
                            ret.push('═');
                            ret.push('╣');
                        }
                        continue;
                    }
                    let next_x = table.get_from_coordi(x.offset(1).unwrap(), y.offset(-1).unwrap());
                    let next_xy = table.get_from_coordi(x.offset(1).unwrap(), y);

                    let up_side = this_cell.rep_zone() == next_x.rep_zone();
                    let left_side = this_cell.rep_zone() == next_y.rep_zone();
                    let right_side = next_x.rep_zone() == next_xy.rep_zone();
                    let down_side = next_y.rep_zone() == next_xy.rep_zone();

                    match (up_side, left_side, right_side, down_side) {
                        (true, true, true, true) => {
                            ret.push('╌');
                            ret.push('╌')
                        }
                        (false, false, false, false) => {
                            ret.push('═');
                            ret.push('╬');
                        }
                        (true, false, false, true) => {
                            ret.push('═');
                            ret.push('═');
                        }
                        (false, true, true, false) => {
                            ret.push('╌');
                            ret.push('║');
                        }
                        (true, true, false, false) => {
                            ret.push('╌');
                            ret.push('═');
                        }
                        (false, false, false, true) => {
                            ret.push('═');
                            ret.push('╩');
                        }
                        (true, false, true, false) => {
                            ret.push('═');
                            ret.push('╗');
                        }
                        (false, false, true, true) => {
                            ret.push('═');
                            ret.push('╝');
                        }
                        (false, true, false, true) => {
                            ret.push('╌');
                            ret.push('╚');
                        }
                        (true, false, false, false) => {
                            ret.push('═');
                            ret.push('╦');
                        }
                        _ => {
                            ret.push(' ');
                            ret.push(' ')
                        }
                    }
                }
            }
            ret.push('\n');
            ret.push('║');

            for x in MaxNum::<N>::iter() {
                let cell = self.read_from_coordinate(x, y);
                let final_num = final_fn(cell);
                if let Some(num) = final_num {
                    ret.push(num.get_char());
                    some += 1;
                } else {
                    ret.push(' ');
                    none += 1;
                }

                if x == MaxNum::<N>::MAX {
                    ret.push('║');
                } else {
                    let next_cell = table.get_from_coordi(x.offset(1).unwrap(), y);
                    let this_cell = table.get_from_coordi(x, y);

                    if this_cell.rep_zone() == next_cell.rep_zone() {
                        ret.push('┆');
                    } else {
                        ret.push('║');
                    }
                }
            }
            ret.push('\n');
        }

        // 마지막 줄
        ret.push('╚');
        ret.push('═');
        for x in MaxNum::<N>::iter() {
            if x == MaxNum::<N>::MAX {
                ret.push('╝');
            } else {
                let next_cell = table.get_from_coordi(x.offset(1).unwrap(), MaxNum::<N>::MAX);
                let this_cell = table.get_from_coordi(x, MaxNum::<N>::MAX);

                if next_cell.rep_zone() == this_cell.rep_zone() {
                    ret.push('═');
                } else {
                    ret.push('╩');
                }

                ret.push('═');
            }
        }
        ret.push('\n');
        ret.push_str("some: ");
        ret.push_str(&some.to_string());
        ret.push('\t');
        ret.push_str("none: ");
        ret.push_str(&none.to_string());

        ret
    }

    pub fn to_string_with_punch(&self) -> String {
        self.make_string(NumCheck::fixed_final_num)
    }
}

impl<'a, 'b, const N: usize> Display for TableLockReadGuard<'a, 'b, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ret = self.make_string(NumCheck::final_num);
        write!(f, "{ret}")
    }
}

impl<'a, 'b, 'c, const N: usize> IntoIterator for &'c TableLockReadGuard<'a, 'b, N> {
    type Item = (&'a Cell<N>, &'a NumCheck<N>);

    type IntoIter = ReadCellIter<'a, 'b, 'c, N>;

    fn into_iter(self) -> Self::IntoIter {
        ReadCellIter {
            read: self,
            index: 0,
            cells: &self.table_lock.table.cells,
        }
    }
}

pub struct ReadCellIter<'a, 'b, 'c, const N: usize> {
    read: &'c TableLockReadGuard<'a, 'b, N>,
    index: usize,
    cells: &'a [Cell<N>],
}

impl<'a, 'b, 'c, const N: usize> Iterator for ReadCellIter<'a, 'b, 'c, N> {
    type Item = (&'a Cell<N>, &'a NumCheck<N>);

    fn next(&mut self) -> Option<Self::Item> {
        let cell = self.cells.get(self.index)?;
        self.index += 1;

        unsafe { Some((cell, self.read.read_from_cell_unchecked(cell))) }
    }
}

pub struct TableLockWriteGuard<'a, 'b, const N: usize> {
    table_lock: &'a TableLock<N>,
    _write_guard: RwLockWriteGuard<'b, ()>,
}

impl<'a, 'b, const N: usize> TableLockWriteGuard<'a, 'b, N> {
    #[must_use]
    #[inline]
    pub fn read_from_cell(&self, cell: &Cell<N>) -> &NumCheck<N> {
        self.table_lock.assert_cell_in_table(cell);
        unsafe { &*cell.chk_unsafe.get() }
    }

    #[must_use]
    #[inline]
    pub fn read_from_coordinate(&self, x: MaxNum<N>, y: MaxNum<N>) -> &NumCheck<N> {
        unsafe {
            &*self
                .table_lock
                .get_cell_from_coordinate(x, y)
                .chk_unsafe
                .get()
        }
    }

    #[must_use]
    #[inline]
    pub fn write_from_cell(&mut self, cell: &Cell<N>) -> &'a mut NumCheck<N> {
        self.table_lock.assert_cell_in_table(cell);
        unsafe { &mut *cell.chk_unsafe.get() }
    }

    /// # Safety
    ///
    /// Table 내에 속한 Cell로만 이 함수를 호출해야 함.
    #[must_use]
    #[inline]
    pub unsafe fn write_from_cell_unchecked(&mut self, cell: &Cell<N>) -> &'a mut NumCheck<N> {
        unsafe { &mut *cell.chk_unsafe.get() }
    }

    #[must_use]
    #[inline]
    pub fn write_from_coordinate(&mut self, x: MaxNum<N>, y: MaxNum<N>) -> &'a mut NumCheck<N> {
        unsafe {
            &mut *self
                .table_lock
                .get_cell_from_coordinate(x, y)
                .chk_unsafe
                .get()
        }
    }
}

impl<'a, 'b, 'c, const N: usize> IntoIterator for &'c mut TableLockWriteGuard<'a, 'b, N> {
    type Item = (&'a Cell<N>, &'a mut NumCheck<N>);

    type IntoIter = WriteCellIter<'a, 'b, 'c, N>;

    fn into_iter(self) -> Self::IntoIter {
        WriteCellIter {
            write: self,
            index: 0,
        }
    }
}

pub struct WriteCellIter<'a, 'b, 'c, const N: usize> {
    write: &'c mut TableLockWriteGuard<'a, 'b, N>,
    index: usize,
}

impl<'a, 'b, 'c, const N: usize> Iterator for WriteCellIter<'a, 'b, 'c, N> {
    type Item = (&'a Cell<N>, &'a mut NumCheck<N>);

    fn next(&mut self) -> Option<Self::Item> {
        let cell = self.write.table_lock.table.cells.get(self.index)?;
        self.index += 1;

        unsafe { Some((cell, self.write.write_from_cell_unchecked(cell))) }
    }
}
