use sudoku_solver_lib::model::table::Table;
use sudoku_solver_lib::solver::{GeneralSolve, Solver};

#[test]
fn main() {
    let mut t = Table::new_default_9(); // 9x9 스도쿠 구조. 16x16를 생성하기 위해선 Table::new_default_16()을 사용.
    let mut solver = Solver::new(&mut t);

    solver.set_random_seed(0); // 스도쿠의 랜덤 시드 고정. 이걸 빼면 무작위 스도쿠가 만들어짐.
    solver.fill_puzzle_with_timeout(std::time::Duration::MAX);
    println!("{}", solver.get_table());
}
