use enum_iterator::all;
use std::time::Instant;
use sudoku_solver_lib::model::table::Table;
use sudoku_solver_lib::solver::solver_simple::SolverSimple;
use sudoku_solver_lib::solver::Solver;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    let mut t = Table::new_default_16();
    let mut solver = Solver::new(&mut t);

    let start = Instant::now();
    solver.set_random_seed(0); // 실행시간 측정을 위한 시드 고정. 이걸 빼면 무작위 스도쿠 퍼즐이 만들어짐.
    solver.fill_puzzle_with_timeout(std::time::Duration::MAX);
    let end = Instant::now();
    println!("{}", solver.get_table());

    println!("puzzle seed: {}", solver.get_random_seed());
    for n in all::<SolverSimple>() {
        println!("{:?}: {}", n, solver.solve_cnt(&n));
    }

    println!(
        "guess: {}, guess_rollback_cnt: {}, guess_backtrace_rollback_cnt: {}",
        solver.guess_cnt(),
        solver.guess_rollback_cnt(),
        solver.guess_backtrace_rollback_cnt()
    );
    println!("time: {}ms", (end - start).as_millis());
}
