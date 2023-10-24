pub mod naked_single;

use crate::{
    model::{table_lock::TableLock, zone_cache::ZoneCache},
    num_check::NumCheck,
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
            let naked_single = Punch::naked_single_punch(self.table, &self.zone_cache, &read);

            if naked_single.is_empty() {
                break;
            }

            let pick = self.rng.pick_one_from_slice(&naked_single);
            let mut write = read.upgrade_to_write();

            *write.write_from_cell(pick) = NumCheck::new_with_true();
        }
    }
}
