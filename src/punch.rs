pub mod naked_single;

use crate::{
    model::{
        cell::Cell,
        table_lock::{TableLock, TableLockReadGuard},
        zone_cache::ZoneCache,
    },
    num_check::NumCheck,
    solver::Solver,
};
use rand::{rngs::SmallRng, seq::SliceRandom};

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

    pub fn into_solver(self) -> Solver<'a, N> {
        self.zone_cache.checked_zone_all_clear();
        Solver::new_with_cache(self.table, self.zone_cache)
    }

    #[inline]
    fn punch_cell_commit(&mut self, read: TableLockReadGuard<N>, cells: &[&Cell<N>]) {
        let pick = cells.choose(&mut self.rng).expect("cells is empty");

        let mut effect_cell: Vec<&Cell<N>> = Vec::with_capacity(N * N);
        effect_cell.push(pick);

        for zone in &pick.zone_vec {
            for zone_cell in &self.zone_cache.zone()[zone] {
                if zone_cell == pick {
                    continue;
                }

                effect_cell.push(zone_cell);
            }
        }

        let mut change_cell: Vec<(&Cell<N>, NumCheck<N>)> = Vec::with_capacity(N * N);

        for cell in effect_cell {
            let mut cell_chk = NumCheck::<N>::new_with_true();
            for zone in &cell.zone_vec {
                for &zone_cell in &self.zone_cache.zone()[zone] {
                    if zone_cell == cell {
                        continue;
                    }

                    if let Some(final_num) = read.read_from_cell(zone_cell).get_final_num() {
                        cell_chk.set_false(final_num);
                    }
                }
            }
            change_cell.push((cell, cell_chk));
        }

        let mut write = read.upgrade_to_write();

        let pick_write = write.write_from_cell(pick);
        pick_write.fixed_final_num_set_none();

        for (cell, chk) in change_cell {
            *write.write_from_cell(cell) = chk;
        }
    }
}
