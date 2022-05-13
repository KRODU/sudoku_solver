use hashbrown::HashMap;
use rand::Rng;

use crate::cell::Cell;

use super::{
    solver_history::{SolverHistory, SolverHistoryType},
    Solver,
};

impl<'a> Solver<'a> {
    /// 값이 확정되지 않은 cell중에 하나를 무작위로 guess하여 넣습니다.
    pub fn guess_random(&mut self) {
        let (cell_pick, note_pick) = {
            let mut minimum_note_cnt = usize::MAX;
            let mut minimum_note_list: Vec<&Cell> = Vec::new();

            for n in self.t.get_cell().values() {
                let b = n.chk.borrow();
                let true_cnt = b.get_true_cnt();
                if true_cnt <= 1 || true_cnt > minimum_note_cnt {
                    continue;
                }

                if true_cnt < minimum_note_cnt {
                    minimum_note_list.clear();
                    minimum_note_cnt = true_cnt;
                }

                minimum_note_list.push(n);
            }

            // 모든 스도쿠 퍼즐이 채워진 경우 return
            if minimum_note_list.is_empty() {
                return;
            }

            let mut rng = self.rng.borrow_mut();
            let cell_pick = minimum_note_list[rng.gen_range(0..minimum_note_list.len())];
            let cell_notes = cell_pick.chk.borrow().clone_chk_list();
            let note_pick = cell_notes[rng.gen_range(0..cell_notes.len())];

            (cell_pick, note_pick)
        };

        self.guess_mut_something(cell_pick, note_pick);
    }

    /// 특정 Cell의 값을 가정합니다. 불가능한 값일 경우 panic이 발생합니다.
    ///
    /// 이 함수는 노트의 값을 변경시키기에 다른 스레드에서 값을 읽는중이면 안됩니다.
    ///
    /// 히스토리에 Guess를 추가합니다.
    pub fn guess_mut_something(&mut self, cell: &'a Cell, final_num: usize) {
        let mut b = cell.chk.borrow_mut();

        // 불가능한 값으로 guess할 경우 panic 발생
        if !b.get_chk(final_num) {
            panic!("불가능한 값으로의 GUESS!")
        }

        // 만약 해당 cell이 이미 해당 값으로 확정된 경우 무시됨
        if b.is_final_num() {
            return;
        }

        let backup = b.clone_chk_list();
        let mut backup_chk: HashMap<&'a Cell, Vec<usize>> = HashMap::with_capacity(1);
        backup_chk.insert(cell, backup);
        b.set_to_value(final_num);

        let history: SolverHistory<'a> = SolverHistory {
            history_type: SolverHistoryType::Guess { cell, final_num },
            backup_chk,
        };

        self.solver_history_stack.push(history);
    }
}
