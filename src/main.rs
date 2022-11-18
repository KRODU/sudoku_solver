use crate::model::table::{self, Table};
use crate::solver::solver_simple::SolverSimple;
use enum_iterator::all;
use solver::Solver;
use std::time::Instant;

mod combinations;
mod model;
pub mod num_check;
pub mod solver;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    let mut t = Table::new_default_16();
    let mut solver = Solver::new(&mut t);

    let start = Instant::now();
    solver.fill_puzzle_with_timeout(std::time::Duration::MAX);
    println!("{}", solver.get_table());
    let end = Instant::now();

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
    println!("duration time: {}ms", (end - start).as_millis());
}
