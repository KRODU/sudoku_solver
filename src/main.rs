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
    std::env::set_var("RUST_BACKTRACE", "1");
    let t = Table::new_default_16();
    let mut solver = Solver::new(&t);
    // solver.set_random_seed(7331333875655129923);
    let start = Instant::now();

    while !solver.is_complete_puzzle() {
        let result = solver.solve();
        if let Some(_) = result {

            // println!("{:?}", history);
        } else {
            panic!("오류!")
        }
        // println!("{}\n\n", t);
        // std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("{}", t);
    let end = Instant::now();

    println!("puzzle seed: {}", solver.get_random_seed());
    println!(
        "solve: {}, guess: {}, guess_rollback: {}",
        solver.solve_cnt(), solver.guess_cnt(), solver.guess_rollback_cnt()
    );
    println!("duration time: {}ms", (end - start).as_millis());
}
