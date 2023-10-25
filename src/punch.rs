pub mod naked_single;

use crate::{
    model::{
        cell::Cell,
        table_lock::{TableLock, TableLockReadGuard},
        zone_cache::ZoneCache,
    },
    rng_util::RngUtil,
};
use rand::rngs::SmallRng;

pub struct Punch<'a, const N: usize> {
    table: &'a TableLock<N>,
    rng: SmallRng,
    zone_cache: ZoneCache<'a, N>,
}

impl<'a, const N: usize> Punch<'a, N> {
    pub fn new(table: &'a TableLock<N>, rng: SmallRng, zone_cache: ZoneCache<'a, N>) -> Self {
        zone_cache.checked_zone_all_clear();

        let mut write = table.write_lock();
        for (_, chk) in &mut write {
            chk.fixed_final_num_set_dup();
        }

        Punch {
            table,
            rng,
            zone_cache,
        }
    }

    pub fn get_table(&self) -> &'a TableLock<N> {
        self.table
    }

    pub fn punch_all(&mut self) {
        loop {
            let read = self.table.read_lock();
            let naked_single = Punch::naked_single_punch(&self.zone_cache, &read);

            if naked_single.is_empty() {
                break;
            }

            self.punch_cell_commit(read, &naked_single);
        }
    }

    #[inline]
    fn punch_cell_commit(&mut self, read: TableLockReadGuard<N>, cells: &[&Cell<N>]) {
        let pick = self.rng.pick_one_from_slice(cells);
        let mut write = read.upgrade_to_write();

        let pick_write = write.write_from_cell(pick);

        pick_write.fixed_final_num_set_none();
        let pick_final = pick_write
            .get_final_num()
            .expect("punch pick cell unwrap fail");

        for zone in &pick.zone_vec {
            for zone_cell in &self.zone_cache.zone()[zone] {
                if zone_cell == pick {
                    continue;
                }

                write.write_from_cell(zone_cell).set_true(pick_final);
            }
        }
    }
}
