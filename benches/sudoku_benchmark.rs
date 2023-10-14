use criterion::{criterion_group, criterion_main, Criterion};
use rand::{RngCore, SeedableRng};
use std::hint::black_box;
use sudoku_solver_lib::{self, model::table::Table, solver::Solver};

fn bench_sudoku(c: &mut Criterion) {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    c.bench_function("sudoku_9x9", |b| {
        b.iter(|| {
            let mut t = Table::new_default_9();
            let mut solver = Solver::new_with_seed(&mut t, rng.next_u64());

            // solver.set_random_seed(black_box(0)); // 실행시간 측정을 위한 시드 고정. 이걸 빼면 무작위 스도쿠 퍼즐이 만들어짐.
            black_box(solver.fill_puzzle_with_timeout(std::time::Duration::MAX))
        })
    });

    c.bench_function("sudoku_16x16", |b| {
        b.iter(|| {
            let mut t = Table::new_default_16();
            let mut solver = Solver::new_with_seed(&mut t, rng.next_u64());

            // solver.set_random_seed(black_box(0)); // 실행시간 측정을 위한 시드 고정. 이걸 빼면 무작위 스도쿠 퍼즐이 만들어짐.
            black_box(solver.fill_puzzle_with_timeout(std::time::Duration::MAX))
        })
    });
}

criterion_group!(benches, bench_sudoku);
criterion_main!(benches);
