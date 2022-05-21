use enum_iterator::IntoEnumIterator;
use solver::Solver;
use std::time::Instant;
use table::Table;

use crate::solver::solver_skip_result::SolverResultSimple;

pub mod cell;
pub mod coordinate;
pub mod num_check;
pub mod solver;
pub mod table;
pub mod zone;
pub mod zone_set;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    let mut t = Table::new_default_16();
    let mut solver = Solver::new(&mut t);
    // solver.set_random_seed(297982631672622005);

    let start = Instant::now();
    solver.fill_puzzle_with_timeout(std::time::Duration::MAX);
    println!("{}", solver.get_table());
    let end = Instant::now();

    println!("puzzle seed: {}", solver.get_random_seed());
    for n in SolverResultSimple::into_enum_iter() {
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
