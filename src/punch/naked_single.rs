use super::Punch;
use crate::model::{
    cell::Cell,
    table_lock::{TableLock, TableLockReadGuard},
    zone_cache::ZoneCache,
};

impl<'a, const N: usize> Punch<'a, N> {
    pub fn naked_single_punch(
        table: &'a TableLock<N>,
        zone_cache: &ZoneCache<'a, N>,
        read: &TableLockReadGuard<N>,
    ) -> Vec<&'a Cell<N>> {
        let mut target_cell: Vec<&Cell<N>> = Vec::new();

        for cell in table {
            if !read.read_from_cell(cell).is_final_num() {
                continue;
            }

            'zone_iter: for zone in cell.get_zone() {
                for &zone_in_cell in &zone_cache.zone()[zone] {
                    if cell == zone_in_cell {
                        continue;
                    }

                    if !read.read_from_cell(zone_in_cell).is_final_num() {
                        continue 'zone_iter;
                    }
                }

                if target_cell.capacity() == 0 {
                    target_cell.reserve_exact(N * N);
                }
                target_cell.push(cell);
                break 'zone_iter;
            }
        }

        target_cell
    }
}
