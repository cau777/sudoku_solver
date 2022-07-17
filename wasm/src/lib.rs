pub mod sudoku_board;
pub mod sudoku_examples;
pub mod sudoku_solver;
mod number_options;
mod util;
mod solve_report;

use wasm_bindgen::prelude::*;
use crate::sudoku_board::SudokuBoard;
use crate::sudoku_solver::SudokuSolver;

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
    result.unwrap().board_to_string()
}