use std::collections::{LinkedList};
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::Message;
use crate::number_options::NumberOptions;
use crate::solve_report::{ReportStep};
use crate::sudoku_board::{SudokuBoard};
use crate::util::Array2D;

type Possibilities<const SIZE: usize> = Array2D<NumberOptions<SIZE>, SIZE>;

/// Main struct to solve boards
/// Constant type parameters are used to increase performance and avoid heap allocations
pub struct SudokuSolver<const SIZE: usize, const BLOCK_SIZE: usize> {
    record_steps: usize,
    pub steps: Vec<ReportStep<SIZE, BLOCK_SIZE>>,
}

impl<const SIZE: usize, const BLOCK_SIZE: usize> SudokuSolver<SIZE, BLOCK_SIZE> {
    pub fn new(record_steps: usize) -> Self {
        SudokuSolver {
            record_steps,
            steps: Vec::with_capacity(record_steps),
        }
    }

    fn should_report_step(&self) -> bool {
        self.steps.len() < self.record_steps
    }

    pub fn solve_random(&mut self, board: &SudokuBoard<SIZE, BLOCK_SIZE>, rand: &mut ThreadRng) -> Option<SudokuBoard<SIZE, BLOCK_SIZE>> {
        let mut stack = LinkedList::<SudokuBoard<SIZE, BLOCK_SIZE>>::new();
        stack.push_front(board.clone());

        while !stack.is_empty() {
            let mut current = stack.pop_front().unwrap();

            while self.develop(&mut current) {}

            if current.is_full() {
                return Some(current);
            }

            let next = Self::find_random_to_try(&current, rand);
            if next.is_none() {
                continue;
            }

            let [row, col] = next.unwrap();
            let possible = current.get_possible(row, col);
            for possible in possible.as_vec() {
                let mut board = current.clone();
                board.set_number(Some(possible), row, col);
                stack.push_front(board);
            }
        }

        None
    }

    pub fn solve(&mut self, board: &SudokuBoard<SIZE, BLOCK_SIZE>) -> Option<SudokuBoard<SIZE, BLOCK_SIZE>> {
        let mut stack = LinkedList::<SudokuBoard<SIZE, BLOCK_SIZE>>::new();
        let mut info_stack = LinkedList::<ReportStep<SIZE, BLOCK_SIZE>>::new();
        self.steps.clear();

        stack.push_front(board.clone());

        while !stack.is_empty() {
            let mut current = stack.pop_front().unwrap();

            if self.should_report_step() && !info_stack.is_empty() {
                self.steps.push(info_stack.pop_front().unwrap());
            }

            // Fill as many known cells as possible to reduce the number of guesses
            while self.develop(&mut current) {}

            // If the board is finished
            if current.is_full() {
                return Some(current);
            }

            let next = Self::find_next_to_try(&current);

            if next.is_none() { continue; }
            let [row, col] = next.unwrap();

            let possible = current.get_possible(row, col);
            // Add to the stack all variations of the board regarding that cell
            for possible in possible.as_vec() {
                let mut board = current.clone();
                board.set_number(Some(possible), row, col);

                if self.should_report_step() {
                    info_stack.push_front(ReportStep {
                        message: Message::Tried(possible, row, col),
                        highlight_row: Some(row as u8),
                        highlight_col: Some(col as u8),
                        highlight_block: None,
                        literal: board.to_literal(),
                    });
                }

                stack.push_front(board);
            }
        }

        if self.should_report_step() {
            self.steps.push(ReportStep {
                message: Message::GaveUp,
                highlight_row: None,
                highlight_col: None,
                highlight_block: None,
                literal: board.to_literal(),
            });
        }

        None
    }

    /// Search for a cell that can only contain one number, because all the other ones are already
    /// taken in the row/column/block.
    /// Return whether a cell meeting the condition was found.
    fn sole_candidates(&mut self, board: &mut SudokuBoard<SIZE, BLOCK_SIZE>,
                       possibilities: &Array2D<NumberOptions<SIZE>, SIZE>) -> bool {
        for row in 0..SIZE {
            for col in 0..SIZE {
                // Skip cells with known numbers
                if board.get_number(row, col).is_some() { continue; }

                let possible = possibilities[row][col];
                if possible.count() == 1 {
                    let value = possible.first().unwrap();

                    board.set_number(Some(value), row, col);
                    if self.should_report_step() {
                        self.steps.push(ReportStep {
                            message: Message::CanContainOnly(value, row + 1, col + 1),
                            highlight_row: Some(row as u8),
                            highlight_col: Some(col as u8),
                            highlight_block: None,
                            literal: board.to_literal(),
                        })
                    }
                    return true;
                }
            }
        }

        false
    }

    /// Search for a situation where, in a row/column, a number can only be put in one cell.
    /// The type parameter makes the code search on columns instead of rows.
    /// Return whether a cell meeting the condition was found.
    fn unique_candidates_lines<const INVERT: bool>(&mut self,
                                                   board: &mut SudokuBoard<SIZE, BLOCK_SIZE>,
                                                   possibilities: &Possibilities<SIZE>) -> bool {
        for i in 0..SIZE {
            let mut at_least_one = NumberOptions::default();
            let mut more_than_one = NumberOptions::default();

            for j in 0..SIZE {
                let row = if INVERT { j } else { i };
                let col = if INVERT { i } else { j };

                if board.get_number(row, col).is_some() { continue; }

                let possible = possibilities[row][col];
                more_than_one |= at_least_one & possible;
                at_least_one |= possible;
            }

            let unique = at_least_one & !more_than_one;
            if unique.count() != 0 {
                let first = unique.as_vec()[0];

                for j in 0..SIZE {
                    let row = if INVERT { j } else { i };
                    let col = if INVERT { i } else { j };

                    if board.get_number(row, col).is_some() { continue; }

                    if possibilities[row][col].has_number(first) {
                        board.set_number(Some(first), row, col);
                        if self.should_report_step() {
                            self.steps.push(ReportStep {
                                message: if INVERT { Message::NumberOnlyFitsInCol(first, i + 1) } else { Message::NumberOnlyFitsInRow(first, i + 1) },
                                highlight_row: if INVERT { None } else { Some(row as u8) },
                                highlight_col: if INVERT { Some(col as u8) } else { None },
                                highlight_block: None,
                                literal: board.to_literal(),
                            });
                        }
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Search for a situation where, in a block, a number can only be put in one cell.
    /// Return whether a cell meeting the condition was found.
    fn unique_candidates_blocks(&mut self, board: &mut SudokuBoard<SIZE, BLOCK_SIZE>, possibilities: &Possibilities<SIZE>) -> bool {
        for block_row in 0..BLOCK_SIZE {
            for block_col in 0..BLOCK_SIZE {
                let mut at_least_one = NumberOptions::default();
                let mut more_than_one = NumberOptions::default();

                let block_offset_row = block_row * BLOCK_SIZE;
                let block_offset_col = block_col * BLOCK_SIZE;

                for i in 0..BLOCK_SIZE {
                    for j in 0..BLOCK_SIZE {
                        let row = block_offset_row + i;
                        let col = block_offset_col + j;

                        if board.get_number(row, col).is_some() { continue; }

                        let possible = possibilities[row][col];
                        more_than_one |= at_least_one & possible;
                        at_least_one |= possible;
                    }
                }

                let unique = at_least_one & !more_than_one;
                if unique.count() != 0 {
                    let first = unique.as_vec()[0];

                    for i in 0..BLOCK_SIZE {
                        for j in 0..BLOCK_SIZE {
                            let row = block_offset_row + i;
                            let col = block_offset_col + j;

                            if board.get_number(row, col).is_some() { continue; }

                            if possibilities[row][col].has_number(first) {
                                board.set_number(Some(first), row, col);
                                if self.should_report_step() {
                                    self.steps.push(ReportStep {
                                        message: Message::NumberOnlyFitsInBlock(first, block_row + 1, block_col + 1),
                                        highlight_row: None,
                                        highlight_col: None,
                                        highlight_block: Some([block_row as u8, block_col as u8]),
                                        literal: board.to_literal(),
                                    });
                                }
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    /// Return a nested array of all the values that can be put in each cell
    fn generate_possibilities(board: &SudokuBoard<SIZE, BLOCK_SIZE>) -> Possibilities<SIZE> {
        let mut result = [[NumberOptions::default(); SIZE]; SIZE];

        for row in 0..SIZE {
            for col in 0..SIZE {
                result[row][col] = board.get_possible(row, col);
            }
        }

        result
    }

    /// Fill a cell whose value can be known for certain. Return whether it was able to find
    /// a cell that met this condition
    fn develop(&mut self, board: &mut SudokuBoard<SIZE, BLOCK_SIZE>) -> bool {
        let possibilities = Self::generate_possibilities(board);
        if self.sole_candidates(board, &possibilities)
            || self.unique_candidates_lines::<false>(board, &possibilities)
            || self.unique_candidates_lines::<true>(board, &possibilities)
            || self.unique_candidates_blocks(board, &possibilities)
        {
            return true;
        }

        false
    }

    /// Get a the cell with the least number of possibilities to try next. If multiple cells
    /// have the same number of possibilities, it returns the last one.
    fn find_next_to_try(board: &SudokuBoard<SIZE, BLOCK_SIZE>) -> Option<[usize; 2]> {
        let mut results: [Option<[usize; 2]>; SIZE] = [None; SIZE];

        for row in 0..SIZE {
            for col in 0..SIZE {
                if board.get_number(row, col).is_some() {
                    continue;
                }

                let count = board.get_possible(row, col).count();
                if count == 0 {
                    return None;
                }
                results[(count - 1) as usize] = Some([row, col]);
            }
        }

        for x in results {
            if x.is_some() {
                return x;
            }
        }

        None
    }

    /// Get a the cell with the least number of possibilities to try next. If multiple cells
    /// have the same number of possibilities, it returns a random one.
    /// Only used for random board generation.
    fn find_random_to_try(board: &SudokuBoard<SIZE, BLOCK_SIZE>, rand: &mut ThreadRng) -> Option<[usize; 2]> {
        let mut results: [Option<[usize; 2]>; SIZE] = [None; SIZE];

        for row in 0..SIZE {
            for col in 0..SIZE {
                if board.get_number(row, col).is_some() {
                    continue;
                }

                let count = board.get_possible(row, col).count();
                if count == 0 {
                    return None;
                }

                let index = (count - 1) as usize;
                if results[index].is_some() && rand.gen_bool(0.4) { continue; }
                results[index] = Some([row, col]);
            }
        }

        for x in results {
            if x.is_some() {
                return x;
            }
        }

        None
    }
}


#[cfg(test)]
mod tests {
    use std::fs::OpenOptions;
    use std::io::{BufRead, BufReader};
    use crate::sudoku_board::{DefaultBoard};
    use crate::sudoku_examples::{EASY_LITERALS, HARD_LITERALS, MEDIUM_LITERALS};
    use crate::sudoku_solver::SudokuSolver;

    #[test]
    fn util() {}

    #[test]
    fn develop_easy() {
        let mut instance = SudokuSolver::new(0);
        for mut board in EASY_LITERALS.map(DefaultBoard::from_literal) {
            while instance.develop(&mut board) {}
            assert!(board.is_full());
        }
    }

    #[test]
    fn solve_easy() {
        let mut instance = SudokuSolver::new(0);
        for board in EASY_LITERALS.map(DefaultBoard::from_literal) {
            let solved = instance.solve(&board);
            assert!(solved.is_some());
            assert!(solved.unwrap().is_full());
        }
    }

    #[test]
    fn solve_medium() {
        let mut instance = SudokuSolver::new(0);
        for board in MEDIUM_LITERALS.map(DefaultBoard::from_literal) {
            let solved = instance.solve(&board);
            assert!(solved.is_some());
            assert!(solved.unwrap().is_full());
        }
    }

    #[test]
    fn solve_hard() {
        let mut instance = SudokuSolver::new(0);
        for board in HARD_LITERALS.map(DefaultBoard::from_literal) {
            let solved = instance.solve(&board);
            assert!(solved.is_some());
            assert!(solved.unwrap().is_full());
        }
    }

    #[test]
    fn unique_candidates_lines_test() {
        let mut instance = SudokuSolver::new(0);
        let examples: [&str; 1] = [
            "
            _ _ _ _ _ _ _ 9 _
            3 _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _ _
            _ _ _ 2 _ _ 9 _ _
            4 _ _ _ 7 _ _ _ _
            _ _ _ _ _ 9 _ _ _
            6 _ _ _ _ _ _ _ _
            7 _ _ _ _ _ _ _ _
            8 1 2 3 4 5 6 7 9
            "
        ];

        for mut example in examples.map(DefaultBoard::from_literal) {
            let possibilities = SudokuSolver::generate_possibilities(&example);
            let result = instance.unique_candidates_lines::<true>(&mut example, &possibilities);
            assert!(result);
        }
    }

    #[test]
    fn file_4000() {
        let file = OpenOptions::new()
            .read(true)
            .open("./test_data/tests_4000.csv").unwrap(); // Subset from https://www.kaggle.com/datasets/bryanpark/sudoku?resource=download

        let reader = BufReader::new(file);
        let mut instance = SudokuSolver::new(0);

        for line in reader.lines() {
            let line = line.unwrap();
            let v: Vec<String> = line.split(',')
                .map(|o| o.split("")
                    .collect::<Vec<&str>>()
                    .join(" ")
                )
                .map(String::from).collect();

            let input = DefaultBoard::from_literal(&v[0]);
            let expected = DefaultBoard::from_literal(&v[1]);

            let result = instance.solve(&input);

            assert!(result.is_some());
            assert_eq!(result.unwrap(), expected);
        }
    }
}