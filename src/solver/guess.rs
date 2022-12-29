use super::solver_history::{SolverHistory, SolverHistoryType};
use super::Solver;
use crate::model::cell::Cell;
use crate::model::max_num::MaxNum;
use crate::model::table_lock::{TableLockReadGuard, TableLockWriteGuard};
use rand::Rng;

impl<'a, const N: usize> Solver<'a, N> {
    /// 값이 확정되지 않은 cell중에 하나를 무작위로 guess하여 넣습니다.
    pub fn guess_random(&mut self, read: TableLockReadGuard<N>) {
        let mut minimum_note_cnt = usize::MAX;
        let mut minimum_note_list: Vec<&Cell<N>> = Vec::new();

        // 모든 cell중에서 무작위로 선택하는 대신 후보숫자가 가장 적은 cell중에 무작위 선택
        // 이렇게하면 나중에 rollback이 필요할 가능성을 조금이라도 줄일 수 있음
        for (_, cells) in &self.ordered_zone {
            for c in cells {
                let b = read.read_from_cell(c);
                let true_cnt = b.get_true_cnt();
                if true_cnt <= 1 || true_cnt > minimum_note_cnt {
                    continue;
                }

                if true_cnt < minimum_note_cnt {
                    minimum_note_list.clear();
                    minimum_note_cnt = true_cnt;
                }

                minimum_note_list.push(c);
            }
        }

        // 모든 스도쿠 퍼즐이 채워진 경우 return
        if minimum_note_list.is_empty() {
            return;
        }

        let cell_pick = minimum_note_list[self.rng.gen_range(0..minimum_note_list.len())];
        // println!("{:?}", cell_pick.coordi);
        let cell_notes = self
            .t
            .read_lock()
            .read_from_cell(cell_pick)
            .clone_chk_list();
        // println!("{:?}", cell_notes);
        let note_pick = cell_notes[self.rng.gen_range(0..cell_notes.len())];
        // println!("{:?}", note_pick);

        self.guess_mut_something(read.upgrade_to_write(), cell_pick, note_pick);
    }

    /// 특정 Cell의 값을 가정합니다. 불가능한 값일 경우 panic이 발생합니다.
    ///
    /// 이 함수는 노트의 값을 변경시키기에 다른 스레드에서 값을 읽는중이면 안됩니다.
    ///
    /// 히스토리에 Guess를 추가합니다.
    pub fn guess_mut_something(
        &mut self,
        mut write: TableLockWriteGuard<N>,
        cell: &'a Cell<N>,
        final_num: MaxNum<N>,
    ) {
        let b = write.write_from_cell(cell);

        // 불가능한 값으로 guess할 경우 panic 발생
        if !b.get_chk(final_num) {
            panic!("불가능한 값으로의 GUESS!")
        }

        // 만약 해당 cell이 이미 해당 값으로 확정된 경우 무시됨
        if b.is_final_num() {
            return;
        }

        let backup = b.clone_chk_list();
        let backup_chk = vec![(cell, backup)];
        b.set_to_value(final_num);
        self.changed_cell.insert(cell);
        self.checked_zone_clear(cell);

        let history: SolverHistory<'a, N> = SolverHistory {
            history_type: SolverHistoryType::Guess { cell, final_num },
            backup_chk,
        };

        self.solver_history_stack.push(history);
    }
}
