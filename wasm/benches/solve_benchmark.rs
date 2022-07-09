use std::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wasm::sudoku_board::{DefaultBoard};
use wasm::sudoku_examples::{EASY_LITERALS, HARD_LITERALS, MEDIUM_LITERALS};
use wasm::sudoku_solver::solve;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve easy",
                     |b| b.iter(|| solve(black_box(&DefaultBoard::from_literal(EASY_LITERALS[0])))));

    c.bench_function("solve medium",
                     |b| b.iter(|| solve(black_box(&DefaultBoard::from_literal(MEDIUM_LITERALS[0])))));

    c.bench_function("solve hard",
                     |b| b.iter(|| solve(black_box(&DefaultBoard::from_literal(HARD_LITERALS[0])))));
}


criterion_group!{
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = criterion_benchmark
}
criterion_main!(benches);