use core::panic;
use enum_iterator::all;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use sudoku_solver_lib::model::max_num::MaxNum;
use sudoku_solver_lib::model::table::Table;
use sudoku_solver_lib::solver::solver_simple::SolverSimple;
use sudoku_solver_lib::solver::Solver;

const TEST_SAME_PUZZLE: bool = false;
const TEST_SAME_PUZZLE_HISTORY: bool = false;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    let seed_arg = std::env::args().find_map(|arg| arg.parse::<u64>().ok());

    let mut t = Table::new_default_16();
    let mut solver = if let Some(seed) = seed_arg {
        Solver::new_with_seed(&mut t, seed)
    } else {
        Solver::new(&mut t)
    };

    let start = Instant::now();
    // solver.set_random_seed(11151576802745503641); // 실행시간 측정을 위한 시드 고정. 이걸 빼면 무작위 스도쿠 퍼즐이 만들어짐.
    solver.fill_puzzle_with_timeout(std::time::Duration::MAX);
    let end = Instant::now();
    println!("{}", solver.get_table());

    println!("puzzle seed: {}", solver.get_random_seed());
    for n in all::<SolverSimple>() {
        println!("{:?}: {}", n, solver.solve_cnt(n));
    }

    println!(
        "guess: {}, guess_rollback_cnt: {}, guess_backtrace_rollback_cnt: {}",
        solver.guess_cnt(),
        solver.guess_rollback_cnt(),
        solver.guess_backtrace_rollback_cnt()
    );
    println!("solver time: {}ms", (end - start).as_millis());

    let start: Instant = Instant::now();
    let mut punch = solver.into_punch();
    punch.punch_all();
    let end = Instant::now();

    println!("{}", punch.get_table().read_lock().to_string_with_punch());
    println!("==============================================================================");
    println!("{}", punch.get_table());
    println!("==============================================================================");
    println!("punch time: {}ms", (end - start).as_millis());
    let mut solver = punch.into_solver();
    println!("{}", solver.get_table());
    solver.fill_puzzle_with_timeout(std::time::Duration::MAX);
    assert_eq!(solver.guess_cnt(), 0);

    if TEST_SAME_PUZZLE {
        test_same_puzzle();
    }

    if TEST_SAME_PUZZLE_HISTORY {
        test_same_puzzle_history();
    }
}

fn test_same_puzzle() {
    for _ in 0..1000 {
        let mut t2 = Table::new_default_16();
        let mut t3 = Table::new_default_16();

        let mut solver2 = Solver::new(&mut t2);
        solver2.fill_puzzle_with_timeout(std::time::Duration::MAX);

        let mut solver3 = Solver::new(&mut t3);
        solver3.set_random_seed(solver2.get_random_seed());
        solver3.fill_puzzle_with_timeout(std::time::Duration::MAX);

        drop(solver2);
        drop(solver3);

        assert_eq!(t2, t3);
    }
}

fn test_same_puzzle_history() {
    let mut writer = BufWriter::new(File::create("log.txt").unwrap());
    for _ in 0..100 {
        let mut t2 = Table::new_default_16();
        let mut t3 = Table::new_default_16();

        let mut solver2_history: String = String::with_capacity(10_0000);
        let mut solver3_history: String = String::with_capacity(10_0000);

        loop {
            solver2_history.clear();
            solver3_history.clear();

            let last_left = format!("{:?}", t2);
            let last_right = format!("{:?}", t3);

            let mut solver2 = Solver::new(&mut t2);
            if solver2.validater().is_some() {
                println!("INVALID_TABLE");
                continue;
            }
            if !solver2.guess_random() {
                break;
            }

            solver2_history.push_str(&format!(
                "{:?}",
                solver2.get_solver_history().last().unwrap()
            ));
            solver2_history.push('\n');

            while solver2.solve() {
                solver2_history.push_str(&format!(
                    "{:?}",
                    solver2.get_solver_history().last().unwrap()
                ));
                solver2_history.push('\n');
            }

            let mut solver3 = Solver::new(&mut t3);
            solver3.set_random_seed(solver2.get_random_seed());
            if !solver3.guess_random() {
                break;
            }

            solver3_history.push_str(&format!(
                "{:?}",
                solver3.get_solver_history().last().unwrap()
            ));
            solver3_history.push('\n');

            while solver3.solve() {
                solver3_history.push_str(&format!(
                    "{:?}",
                    solver3.get_solver_history().last().unwrap()
                ));
                solver3_history.push('\n');
            }

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
                        wrtie_log(format!(" left Cell: {:?}", r1), &mut writer);
                        wrtie_log(String::new(), &mut writer);
                        wrtie_log(format!("right: Cell {:?}", r2), &mut writer);
                        diff_found = true;
                    }
                }
            }

            if diff_found {
                wrtie_log(format!("left: {:?}", t2), &mut writer);
                wrtie_log(format!("right: {:?}", t3), &mut writer);
                wrtie_log(format!("left: {}", solver2_history), &mut writer);
                wrtie_log(String::new(), &mut writer);
                wrtie_log(format!("right: {}", solver3_history), &mut writer);
                wrtie_log(format!("left_last: {}", last_left), &mut writer);
                wrtie_log(String::new(), &mut writer);
                wrtie_log(format!("right_last: {}", last_right), &mut writer);
                panic!("TABLE_NOT_SAME");
            }
        }
    }
}

fn wrtie_log(str: String, writer: &mut BufWriter<File>) {
    writer.write_all(str.as_bytes()).unwrap();
    writer.write_all(b"\n").unwrap();
}
