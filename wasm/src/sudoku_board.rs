use std::fmt::{Debug, Formatter};
use std::hash::{Hash};
use crate::number_options::{NumberOptions};
use crate::util::Array2D;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct SudokuNumbers {
    numbers: [i8; 9 * 9],
}

impl SudokuNumbers {
    pub fn from_board(board: &SudokuBoard) -> Self {
        let mut result = SudokuNumbers { numbers: [-1; 9 * 9] };
        let mut i = 0;

        for row in 0..9 {
            for col in 0..9 {
                let num = board.get_number(row, col);
                if num.is_some() {
                    result.numbers[i] = num.unwrap() as i8;
                }
                i += 1;
            }
        }

        result
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct SudokuBoard {
    numbers: Array2D<Option<u8>, 9>,
    rows: [NumberOptions; 9],
    cols: [NumberOptions; 9],
    blocks: Array2D<NumberOptions, 3>,
}

impl SudokuBoard {
    pub fn new() -> Self {
        SudokuBoard {
            numbers: [[None; 9]; 9],
            cols: [NumberOptions::default(); 9],
            rows: [NumberOptions::default(); 9],
            blocks: [[NumberOptions::default(); 3]; 3],
        }
    }

    pub fn set_number(&mut self, value: Option<u8>, row: usize, col: usize) {
        let prev = self.numbers[row][col];
        if prev.is_some() {
            let val = prev.unwrap();
            self.rows[row].remove_number(val);
            self.cols[col].remove_number(val);
            self.blocks[row / 3][col / 3].remove_number(val);
            self.numbers[row][col] = None;
        }

        if value.is_some() {
            let val = value.unwrap();
            self.rows[row].add_number(val);
            self.cols[col].add_number(val);
            self.blocks[row / 3][col / 3].add_number(val);
            self.numbers[row][col] = value;
        }
    }

    pub fn get_number(&self, row: usize, col: usize) -> Option<u8> {
        self.numbers[row][col]
    }

    pub fn get_possibilities(&self, row: usize, col: usize) -> NumberOptions {
        !(self.rows[row] | self.cols[col] | self.blocks[row / 3][col / 3])
    }

    pub fn from_literal(literal: &str) -> SudokuBoard {
        let mut board = SudokuBoard::new();
        let mut board_index = 0;

        for c in literal.chars() {
            let digit = c.to_digit(10);
            if digit.is_some() {
                board.set_number(Some(digit.unwrap() as u8), board_index / 9, board_index % 9);
                board_index += 1;
            } else if c == '_' {
                board_index += 1;
            }
        }

        board
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        for row in self.numbers.iter() {
            for &e in row {
                if e.is_none() {
                    return false;
                }
            }
        }
        true
    }

    pub fn board_to_string(&self) -> String {
        let mut result = String::new();
        result += "  ---------------------\n";

        for row in 0..9_usize {
            result += &format!("{} | ", row);
            for col in 0..9_usize {
                result += &(self.numbers[row][col].map(|x| x.to_string()).unwrap_or("_".to_owned()).to_string() + " ");
            }
            result += "|\n";
        }
        result += "  ---------------------";

        result
    }
}

impl Debug for SudokuBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board_to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::sudoku_board::SudokuBoard;

    #[test]
    fn util() {
        assert!(true);
    }

    #[test]
    fn empty_board() {
        let board = SudokuBoard::new();

        for row in 0..9_usize {
            for col in 0..9_usize {
                assert!(board.get_possibilities(row, col).all())
            }
        }
    }

    #[test]
    fn one_element_board() {
        let mut board = SudokuBoard::new();
        board.set_number(Some(1), 0, 0);
        // println!("{:?}", board);

        for row in 0..9_usize {
            for col in 0..9_usize {
                println!("{} {} {:?}", row, col, board.get_possibilities(row, col));
                assert_eq!(row != 0 && col != 0 && !(row < 3 && col < 3), board.get_possibilities(row, col).has_number(1));
            }
        }
    }

    #[test]
    fn insert_and_remove_from_board() {
        let mut board = SudokuBoard::new();
        board.set_number(Some(1), 0, 0);
        board.set_number(None, 0, 0);

        for row in 0..9_usize {
            for col in 0..9_usize {
                assert!(board.get_possibilities(row, col).all())
            }
        }
    }

    #[test]
    fn from_literal_board() {
        let literal = "
        1 2 3 4 5 6 7 8 9
        4 5 6 7 8 9 1 2 3
        7 8 9 1 2 3 4 5 6

        _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _

        _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _";

        let board = SudokuBoard::from_literal(literal);
        assert_eq!(board.numbers, [
            [Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9)],
            [Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3)],
            [Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)],
            [None; 9], [None; 9], [None; 9],
            [None; 9], [None; 9], [None; 9],
        ]);
    }
}
