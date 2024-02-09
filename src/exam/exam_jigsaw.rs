use sudoku_solver_lib::model::cell::Cell;
use sudoku_solver_lib::model::table::Table;
use sudoku_solver_lib::model::table_lock::TableLock;
use sudoku_solver_lib::model::zone::Zone;
use sudoku_solver_lib::solver::Solver;

pub fn new_jigsaw() -> TableLock<9> {
    let zone: [usize; 81] = [
        1, 1, 1, 1, 1, 2, 2, 2, 2, 4, 1, 1, 1, 3, 3, 2, 2, 2, 4, 4, 1, 3, 3, 3, 3, 2, 2, 4, 4, 4,
        5, 5, 3, 3, 3, 6, 4, 4, 5, 5, 5, 5, 5, 6, 6, 4, 7, 7, 7, 5, 5, 6, 6, 6, 8, 8, 7, 7, 7, 7,
        9, 6, 6, 8, 8, 8, 7, 7, 9, 9, 9, 6, 8, 8, 8, 8, 9, 9, 9, 9, 9,
    ]; // 직소 스도쿠의 모양

    let mut cells: Vec<Vec<Cell<9>>> = Vec::with_capacity(9);
    for y in 0..9 {
        let mut row: Vec<Cell<9>> = Vec::with_capacity(9);
        for x in 0..9 {
            let index = zone[x + y * 9];

            let this_zone = vec![
                Zone::new_unique_from_usize(index),  // cell이 속해있는 직소 모양
                Zone::new_unique_from_usize(x + 10), // cell이 속한 세로
                Zone::new_unique_from_usize(y + 19), // cell이 속한 가로
            ];
            let cell = Cell::new(x, y, this_zone);

            row.push(cell);
        }
        cells.push(row);
    }

    Table::new_with_vec_cells(cells)
}

#[test]
fn main() {
    let mut t = new_jigsaw();
    let mut solver = Solver::new(&mut t);

    solver.set_random_seed(0); // 스도쿠의 랜덤 시드 고정. 이걸 빼면 무작위 스도쿠가 만들어짐.
    solver.fill_puzzle_with_timeout(std::time::Duration::MAX);
    println!("{}", solver.get_table());
}
