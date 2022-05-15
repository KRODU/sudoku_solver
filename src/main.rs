use solver::Solver;
use std::time::Instant;
use table::Table;

pub mod cell;
pub mod combinations;
pub mod coordinate;
pub mod num_check;
pub mod solver;
pub mod table;
pub mod zone;
pub mod zone_set;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    let t = Table::new_default_16();
    let mut solver = Solver::new(&t);
    // solver.set_random_seed(297982631672622005);

    let start = Instant::now();
    solver.fill_puzzle_with_timeout(std::time::Duration::MAX);
    println!("{}", t);
    let end = Instant::now();

    println!("puzzle seed: {}", solver.get_random_seed());
    println!(
        "solve: {}, guess: {}, guess_rollback_cnt: {}, guess_backtrace_rollback_cnt: {}",
        solver.solve_cnt(),
        solver.guess_cnt(),
        solver.guess_rollback_cnt(),
        solver.guess_backtrace_rollback_cnt()
    );
    println!("duration time: {}ms", (end - start).as_millis());
}
