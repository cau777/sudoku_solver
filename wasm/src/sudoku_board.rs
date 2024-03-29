use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use crate::number_options::{NumberOptions};
use crate::util::Array2D;

pub type DefaultBoard = SudokuBoard<9, 3>;

/// Struct that keeps track of the numbers in the board and also what values are already used
/// in each row/column/block
#[derive(Clone, Eq, PartialEq)]
pub struct SudokuBoard<const SIZE: usize, const BLOCK_SIZE: usize> {
    pub numbers: Array2D<Option<u8>, SIZE>,
    rows: [NumberOptions<SIZE>; SIZE],
    cols: [NumberOptions<SIZE>; SIZE],
    blocks: Array2D<NumberOptions<SIZE>, BLOCK_SIZE>,
}

pub enum BoardError {
    RowError(usize),
    ColError(usize),
    BlockError(usize, usize),
}

impl<const SIZE: usize, const BLOCK_SIZE: usize> SudokuBoard<SIZE, BLOCK_SIZE> {
    pub fn new() -> Self {
        SudokuBoard {
            numbers: [[None; SIZE]; SIZE],
            cols: [NumberOptions::default(); SIZE],
            rows: [NumberOptions::default(); SIZE],
            blocks: [[NumberOptions::default(); BLOCK_SIZE]; BLOCK_SIZE],
        }
    }

    pub fn set_number(&mut self, value: Option<u8>, row: usize, col: usize) {
        let prev = self.numbers[row][col];
        if let Some(val) = prev {
            self.rows[row].remove_number(val);
            self.cols[col].remove_number(val);
            self.blocks[row / BLOCK_SIZE][col / BLOCK_SIZE].remove_number(val);
            self.numbers[row][col] = None;
        }

        if let Some(val) = value {
            self.rows[row].add_number(val);
            self.cols[col].add_number(val);
            self.blocks[row / BLOCK_SIZE][col / BLOCK_SIZE].add_number(val);
            self.numbers[row][col] = value;
        }
    }

    pub fn get_number(&self, row: usize, col: usize) -> Option<u8> {
        self.numbers[row][col]
    }

    pub fn get_possible(&self, row: usize, col: usize) -> NumberOptions<SIZE> {
        !(self.rows[row] | self.cols[col] | self.blocks[row / BLOCK_SIZE][col / BLOCK_SIZE])
    }

    pub fn from_literal(literal: &str) -> Self {
        let mut board = SudokuBoard::new();

        literal
            .replace('\n', " ")
            .split(' ')
            .filter(|o| !o.is_empty())
            .enumerate()
            .for_each(|(index, o)| {
                board.set_number(u8::from_str(o).ok(), index / SIZE, index % SIZE)
            });

        board
    }

    /// Return a more compact representation of the board, in the format "1 2 3 _ _ 6 7 8 _"
    /// without newlines
    pub fn to_literal(&self) -> String {
        let mut result = String::new();
        for row in &self.numbers {
            for num in row {
                if num.is_some() {
                    result += &num.unwrap().to_string();
                } else {
                    result += "_";
                }

                result += " ";
            }
        }
        result
    }

    pub fn from_literal_checked(literal: &str) -> Result<Self, BoardError> {
        let mut board = SudokuBoard::new();

        for (index, number) in literal
            .replace('\n', " ")
            .split(' ')
            .filter(|o| !o.is_empty())
            .map(u8::from_str)
            .enumerate()
            .filter(|(_, o)| o.is_ok())
            .map(|(index, o)| (index, o.unwrap())) {
            let row = index / SIZE;
            let col = index % SIZE;

            if board.rows[row].has_number(number) {
                return Err(BoardError::RowError(row));
            }
            if board.cols[col].has_number(number) {
                return Err(BoardError::ColError(col));
            }

            let block_row = row / BLOCK_SIZE;
            let block_col = col / BLOCK_SIZE;
            if board.blocks[block_row][block_col].has_number(number) {
                return Err(BoardError::BlockError(block_row, block_col));
            }

            board.set_number(Some(number), row, col);
        }

        Ok(board)
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

    /// Returns a readable representation of the board
    pub fn board_to_string(&self) -> String {
        let mut result = String::new();
        result += "  ---------------------\n";

        for row in 0..SIZE {
            result += &format!("{} | ", row);
            for col in 0..SIZE {
                match self.numbers[row][col] {
                    Some(x) => result += &x.to_string(),
                    None => result += "_",
                }
                result += " ";
            }
            result += "|\n";
        }
        result += "  ---------------------";

        result
    }
}

impl<const SIZE: usize, const BLOCK_SIZE: usize> Debug for SudokuBoard<SIZE, BLOCK_SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board_to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::sudoku_board::{DefaultBoard};

    #[test]
    fn empty_board() {
        let board = DefaultBoard::new();

        for row in 0..9_usize {
            for col in 0..9_usize {
                assert!(board.get_possible(row, col).all())
            }
        }
    }

    #[test]
    fn one_element_board() {
        let mut board = DefaultBoard::new();
        board.set_number(Some(1), 0, 0);
        // println!("{:?}", board);

        for row in 0..9_usize {
            for col in 0..9_usize {
                println!("{} {} {:?}", row, col, board.get_possible(row, col));
                assert_eq!(row != 0 && col != 0 && !(row < 3 && col < 3), board.get_possible(row, col).has_number(1));
            }
        }
    }

    #[test]
    fn insert_and_remove_from_board() {
        let mut board = DefaultBoard::new();
        board.set_number(Some(1), 0, 0);
        board.set_number(None, 0, 0);

        for row in 0..9_usize {
            for col in 0..9_usize {
                assert!(board.get_possible(row, col).all())
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

        let board = DefaultBoard::from_literal(literal);
        assert_eq!(board.numbers, [
            [Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9)],
            [Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3)],
            [Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)],
            [None; 9], [None; 9], [None; 9],
            [None; 9], [None; 9], [None; 9],
        ]);
    }
}
