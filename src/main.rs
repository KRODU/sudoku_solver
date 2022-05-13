use std::time::Instant;

use solver::Solver;
use table::Table;

pub mod cell;
pub mod coordinate;
pub mod num_check;
pub mod solver;
pub mod table;
pub mod util;
pub mod zone;
pub mod zone_set;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let t = Table::new_default_nine();
    let mut solver = Solver::new(&t);
    let start = Instant::now();
    while !solver.is_complete_puzzle() {
        let result = solver.solve();
        if let Ok(history) = result {
            // println!("{:?}", history);
        } else {
            panic!("오류!")
        }
        println!("{}", t);
        println!(
            "-------------------------------------------------------------------------------------"
        );
        // std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("{}", t);
    let end = Instant::now();

    println!("duration time: {}ms", (end - start).as_millis())
}
