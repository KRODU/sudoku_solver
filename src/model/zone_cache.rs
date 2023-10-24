use super::{
    cell::Cell,
    index_key_map::{IndexKeyMap, IndexKeySet},
    non_atomic_bool::NonAtomicBool,
    table_lock::TableLock,
    zone::{Zone, ZoneType},
};
use crate::solver::solver_simple::SolverSimple;
use enum_iterator::cardinality;
use rayon::slice::ParallelSliceMut;

pub struct ZoneCache<'a, const N: usize> {
    // Zone과 Zone에 속한 Cell 목록을 Vec로 정렬
    zone: IndexKeyMap<Zone, Vec<&'a Cell<N>>>,
    connect_zone: IndexKeyMap<Zone, IndexKeySet<Zone>>,
    checked_zone: IndexKeyMap<Zone, IndexKeyMap<SolverSimple, NonAtomicBool>>,
}

impl<'a, const N: usize> ZoneCache<'a, N> {
    pub fn new(t: &'a TableLock<N>) -> Self {
        let zone = ZoneCache::get_zone_map(t);
        let zone_cnt = zone.len();

        for (z, c) in &zone {
            let ZoneType::Unique = z.get_zone_type() else {
                continue;
            };

            if c.len() != N {
                panic!(
                    "Unique 타입의 개수는 퍼즐 사이즈와 동일해야 함! 사이즈:{}, 실제 갯수: {}",
                    N,
                    c.len()
                )
            }
        }

        let connect_zone = ZoneCache::<N>::get_connected_zone(&zone);

        let mut checked_zone: IndexKeyMap<Zone, IndexKeyMap<SolverSimple, NonAtomicBool>> =
            IndexKeyMap::with_capacity(zone_cnt);
        for (z, _) in &zone {
            checked_zone.insert(
                *z,
                IndexKeyMap::with_capacity(cardinality::<SolverSimple>()),
            );
        }

        for (_, check_map) in checked_zone.iter_mut() {
            for n in enum_iterator::all::<SolverSimple>() {
                check_map.insert(n, NonAtomicBool::new(false));
            }
        }

        ZoneCache {
            zone,
            connect_zone,
            checked_zone,
        }
    }

    #[must_use]
    fn get_zone_map(t: &'a TableLock<N>) -> IndexKeyMap<Zone, Vec<&Cell<N>>> {
        let mut index_vec: IndexKeyMap<Zone, Vec<&Cell<N>>> = IndexKeyMap::with_capacity(N * N);
        for cell in t {
            for z in &cell.zone_vec {
                let row = index_vec.entry_or_insert_with(*z, || Vec::with_capacity(N));
                row.push(cell);
            }
        }

        index_vec
            .iter_mut()
            .for_each(|(_, v)| v.par_sort_unstable());
        index_vec
    }

    #[must_use]
    fn get_connected_zone(
        ordered_zone: &IndexKeyMap<Zone, Vec<&Cell<N>>>,
    ) -> IndexKeyMap<Zone, IndexKeySet<Zone>> {
        let mut ret: IndexKeyMap<Zone, IndexKeySet<Zone>> =
            IndexKeyMap::with_capacity(ordered_zone.len());

        for (z1, _) in ordered_zone {
            for (z2, c2_list) in ordered_zone {
                let connected = c2_list.iter().any(|c2| c2.zone_set.contains(z1));

                if connected {
                    ret.entry_or_insert_with(*z1, IndexKeySet::new).insert(*z2);
                }
            }
        }

        ret
    }

    /// 특정 zone에 대한 checked 여부를 반환
    #[must_use]
    #[inline]
    pub fn checked_zone_get_bool(&self, z: &Zone, solver: SolverSimple) -> bool {
        self.checked_zone[z][&solver].get()
    }

    /// 특정 zone에 대한 checked 여부를 true로 설정
    #[inline]
    pub fn checked_zone_set_bool_true(&self, z: Zone, solver: SolverSimple) {
        self.checked_zone[&z][&solver].set(true);
    }

    /// 특정 zone에 대한 checked를 모두 초기화
    pub fn checked_zone_clear<'b>(&self, cells: impl Iterator<Item = &'b Cell<N>>) {
        let mut changed_zone_set: IndexKeySet<Zone> = IndexKeySet::new();

        for c in cells {
            for z in &c.zone_vec {
                if changed_zone_set.contains(z) {
                    continue;
                }
                self.checked_zone[z]
                    .iter()
                    .for_each(|(_, value)| value.set(false));
                changed_zone_set.insert(*z);
            }
        }
    }

    pub fn checked_zone_all_clear(&self) {
        for (_, map) in &self.checked_zone {
            map.iter().for_each(|(_, value)| value.set(false));
        }
    }

    #[must_use]
    #[inline]
    pub fn zone(&self) -> &IndexKeyMap<Zone, Vec<&'a Cell<N>>> {
        &self.zone
    }

    #[must_use]
    #[inline]
    pub fn connect_zone(&self) -> &IndexKeyMap<Zone, IndexKeySet<Zone>> {
        &self.connect_zone
    }

    #[must_use]
    #[inline]
    pub fn checked_zone(&self) -> &IndexKeyMap<Zone, IndexKeyMap<SolverSimple, NonAtomicBool>> {
        &self.checked_zone
    }
}
