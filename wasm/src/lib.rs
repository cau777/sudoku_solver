pub mod sudoku_board;
pub mod sudoku_examples;
pub mod sudoku_solver;
mod number_options;
mod util;
mod solve_report;

use rand::Rng;
use wasm_bindgen::prelude::*;
use crate::sudoku_board::SudokuBoard;
use crate::sudoku_solver::SudokuSolver;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn add(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

#[wasm_bindgen]
pub fn solve(board_literal: &str, block_size: usize, record_steps: usize) -> String {
    match block_size {
        2 => solve_with_size::<4, 2>(board_literal, record_steps),
        3 => solve_with_size::<9, 3>(board_literal, record_steps),
        4 => solve_with_size::<16, 4>(board_literal, record_steps),
        _ => panic!("Invalid size")
    }
}

fn solve_with_size<const SIZE: usize, const BLOCK_SIZE: usize>(board_literal: &str, record_steps: usize) -> String {
    let board = SudokuBoard::<SIZE, BLOCK_SIZE>::from_literal(board_literal);
    let mut solver = SudokuSolver::new(record_steps);
    let result = solver.solve(&board);
    match result {
        None => "".to_owned(),
        Some(board) => board.to_literal(),
    }
}

#[wasm_bindgen]
pub fn find_errors(board_literal: &str, block_size: usize) -> String {
    match block_size {
        2 => find_errors_with_size::<4, 2>(board_literal),
        3 => find_errors_with_size::<9, 3>(board_literal),
        4 => find_errors_with_size::<16, 4>(board_literal),
        _ => panic!("Invalid size")
    }
}

fn find_errors_with_size<const SIZE: usize, const BLOCK_SIZE: usize>(board_literal: &str) -> String {
    let result = SudokuBoard::<SIZE, BLOCK_SIZE>::from_literal_checked(board_literal);
    match result {
        Ok(_) => "null".to_owned(),
        Err(e) => format!("[{},{}]", e[0], e[1])
    }
}

#[wasm_bindgen]
pub fn random_board(coverage: f64, block_size: usize) -> String {
    match block_size {
        2 => random_board_with_size::<4, 2>(coverage),
        3 => random_board_with_size::<9, 3>(coverage),
        4 => random_board_with_size::<16, 4>(coverage),
        _ => panic!("Invalid size")
    }
}

fn random_board_with_size<const SIZE: usize, const BLOCK_SIZE: usize>(coverage: f64) -> String {
    let mut rand = rand::thread_rng();
    let mut board = SudokuBoard::<SIZE, BLOCK_SIZE>::new();
    board.set_number(Some(rand.gen_range(1..=SIZE) as u8),
                     rand.gen_range(0..SIZE), rand.gen_range(0..SIZE));

    let mut solver = SudokuSolver::new(0);
    let mut result = solver.solve_random(&board, &mut rand).unwrap();

    for row in 0..SIZE {
        for col in 0..SIZE {
            if !rand.gen_bool(coverage) {
                result.set_number(None, row, col);
            }
        }
    }

    result.to_literal()
}

#[test]
fn test_random_board_with_size() {
    for _ in 0..100 {
        let mut rand = rand::thread_rng();
        let mut board = SudokuBoard::<9, 3>::new();
        board.set_number(Some(1), rand.gen_range(0..9), rand.gen_range(0..9));
        let mut solver = SudokuSolver::new(0);
        let result = solver.solve_random(&board, &mut rand);

        assert!(result.is_some());
    }
}