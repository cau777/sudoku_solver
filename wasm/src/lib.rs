pub mod sudoku_board;
pub mod sudoku_examples;
pub mod sudoku_solver;
mod number_options;
mod util;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
