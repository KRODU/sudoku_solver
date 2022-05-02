use hashbrown::HashMap;
use rusty_pool::ThreadPool;

use crate::{cell::Cell, coordinate::Coordinate, table::Table, zone::Zone};

pub mod naked;
pub mod solver_result;

pub struct Solver<'a> {
    t: &'a Table,
    zone_list: Vec<&'a Zone>,
    ref_cache: HashMap<&'a Zone, Vec<&'a Cell>>,
    pool: ThreadPool,
}

impl<'a> Solver<'a> {
    pub fn solve(&mut self) {}

    fn shutdown_pool(&mut self) {
        let old_pool: ThreadPool = std::mem::replace(&mut self.pool, ThreadPool::default());
        old_pool.shutdown();
    }

    #[must_use]
    pub fn new(t: &'a Table) -> Self {
        let zone_list = Solver::get_zone_list_init(t.get_cell(), t.get_size());
        Solver {
            t,
            zone_list,
            ref_cache: Solver::get_zone_ref(t),
            pool: ThreadPool::default(),
        }
    }

    #[must_use]
    fn get_zone_ref(t: &'a Table) -> HashMap<&'a Zone, Vec<&'a Cell>> {
        let size: usize = t.get_size();
        let mut zone_ref: HashMap<&'a Zone, Vec<&'a Cell>> = HashMap::with_capacity(size * size);
        for (_, this_cell) in t.get_cell() {
            for z in this_cell.get_zone() {
                let row: &mut Vec<&Cell> = zone_ref
                    .entry(z)
                    .or_insert_with(|| Vec::with_capacity(size));
                row.push(this_cell);
            }
        }

        zone_ref
    }

    #[must_use]
    fn get_zone_list_init(cells: &'a HashMap<Coordinate, Cell>, size: usize) -> Vec<&'a Zone> {
        let mut ret: Vec<&'a Zone> = Vec::with_capacity(size);

        for (_, this_cell) in cells {
            for z in this_cell.get_zone() {
                if !ret.contains(&z) {
                    ret.push(z);
                }
            }
        }
        ret
    }

    #[must_use]
    #[inline]
    pub fn get_zone_list(&self) -> &Vec<&Zone> {
        &self.zone_list
    }

    #[must_use]
    #[inline]
    pub fn zone_iter(&self, zone: &Zone) -> std::slice::Iter<'_, &Cell> {
        self.ref_cache[zone].iter()
    }

    #[must_use]
    #[inline]
    pub fn get_cache(&self) -> &HashMap<&Zone, Vec<&Cell>> {
        &self.ref_cache
    }
}
