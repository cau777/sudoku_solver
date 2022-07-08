use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wasm::sudoku_board::SudokuBoard;
use wasm::sudoku_examples::{EASY_LITERALS, MEDIUM_LITERALS};
use wasm::sudoku_solver::solve;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve easy",
                     |b| b.iter(|| solve(black_box(&SudokuBoard::from_literal(EASY_LITERALS[0])))));

    c.bench_function("solve medium",
                     |b| b.iter(|| solve(black_box(&SudokuBoard::from_literal(MEDIUM_LITERALS[0])))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);