use core::panic;
use enum_iterator::all;
use std::time::Instant;
use sudoku_solver_lib::model::max_num::MaxNum;
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

    drop(solver);
    for _ in 1..=1000 {
        let mut t2 = Table::new_default_16();
        let mut t3 = Table::new_default_16();

        loop {
            let mut solver2 = Solver::new(&mut t2);
            solver2.set_random_seed(0);
            if !solver2.guess_random() {
                break;
            }

            while solver2.solve() {}

            let mut solver3 = Solver::new(&mut t3);
            solver3.set_random_seed(0);
            if !solver3.guess_random() {
                break;
            }
            while solver3.solve() {}
            drop(solver2);
            drop(solver3);

            let t2_read = t2.read_lock();
            let t3_read = t3.read_lock();
            let mut diff_found = false;

            for x in MaxNum::<16>::iter() {
                for y in MaxNum::<16>::iter() {
                    let r1 = t2_read.read_from_coordinate(x, y);
                    let r2 = t3_read.read_from_coordinate(x, y);

                    if !r1.is_same_note(r2) {
                        println!(" left Cell: {:?}", r1);
                        println!("right: Cell {:?}", r2);
                        diff_found = true;
                    }
                }
            }

            if diff_found {
                println!("left: {:?}", t2);
                println!("right: {:?}", t3);
                panic!("ERROR");
            }
        }

        assert_eq!(t2, t3);
    }
}
