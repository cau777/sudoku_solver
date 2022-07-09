use std::collections::{HashSet, LinkedList};
use crate::number_options::NumberOptions;
use crate::sudoku_board::{SudokuBoard, SudokuNumbers};
use crate::util::Array2D;

pub fn solve(board: &SudokuBoard) -> Option<SudokuBoard> {
    let mut stack = LinkedList::<SudokuBoard>::new();
    let mut set = HashSet::<SudokuNumbers>::new();

    stack.push_front(board.clone());

    while !stack.is_empty() {
        let mut current = stack.pop_front().unwrap();

        let numbers = SudokuNumbers::from_board(&current);
        // if set.contains(&numbers) {
        //     continue;
        // }
        // set.insert(numbers);

        while develop(&mut current) {}

        if current.is_full() {
            return Some(current);
        }

        let next = find_next_to_try(&current);
        if next.is_none() {
            continue;
        }
        let [row, col] = next.unwrap();

        let possibilities = current.get_possibilities(row, col);
        for possible in possibilities.as_vec() {
            let mut board = current.clone();
            board.set_number(Some(possible), row, col);
            stack.push_front(board);
        }
    }

    None
}

fn sole_candidates(board: &mut SudokuBoard, possibilities: &Array2D<NumberOptions, 9>) -> bool {
    for row in 0..9_usize {
        for col in 0..9_usize {
            if board.get_number(row, col).is_some() { continue; }

            let possible = possibilities[row][col];
            if possible.count() == 1 {
                board.set_number(Some(possible.first().unwrap()), row, col);
                return true;
            }
        }
    }

    false
}

fn unique_candidates_lines<const INVERT: bool>(board: &mut SudokuBoard, possibilities: &Array2D<NumberOptions, 9>) -> bool {
    for i in 0..9_usize {
        let mut at_least_one = NumberOptions::default();
        let mut more_than_one = NumberOptions::default();

        for j in 0..9_usize {
            let row = if INVERT { j } else { i };
            let col = if INVERT { i } else { j };

            if board.get_number(row, col).is_some() { continue; }

            let possible = possibilities[row][col];
            more_than_one |= at_least_one & possible;
            at_least_one |= possible;
        }

        let unique = at_least_one & !more_than_one;
        if unique.count() != 0 {
            // println!("{:?}", board);
            // println!("{:?}", unique);
            // println!("{:?}", at_least_one);
            // println!("{:?}", more_than_one);
            let first = unique.as_vec()[0];

            for j in 0..9_usize {
                let row = if INVERT { j } else { i };
                let col = if INVERT { i } else { j };

                if board.get_number(row, col).is_some() { continue; }

                if possibilities[row][col].has_number(first) {
                    board.set_number(Some(first), row, col);
                    return true;
                }
            }
        }
    }
    false
}

fn unique_candidates_blocks(board: &mut SudokuBoard, possibilities: &Array2D<NumberOptions, 9>) -> bool {
    for block_row in 0..3_usize {
        for block_col in 0..3_usize {
            let mut at_least_one = NumberOptions::default();
            let mut more_than_one = NumberOptions::default();

            let block_offset_row = block_row * 3;
            let block_offset_col = block_col * 3;

            for i in 0..3_usize {
                for j in 0..3_usize {
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

                for i in 0..3_usize {
                    for j in 0..3_usize {
                        let row = block_offset_row + i;
                        let col = block_offset_col + j;

                        if board.get_number(row, col).is_some() { continue; }

                        if possibilities[row][col].has_number(first) {
                            board.set_number(Some(first), row, col);
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn generate_possibilities(board: &SudokuBoard) -> Array2D<NumberOptions, 9> {
    let mut result = [[NumberOptions::default(); 9]; 9];

    for row in 0..9_usize {
        for col in 0..9_usize {
            result[row][col] = board.get_possibilities(row, col);
        }
    }

    result
}

fn develop(board: &mut SudokuBoard) -> bool {
    let possibilities = generate_possibilities(board);

    if sole_candidates(board, &possibilities)
    || unique_candidates_lines::<false>(board, &possibilities)
    || unique_candidates_lines::<true>(board, &possibilities)
    // || unique_candidates_blocks(board, &possibilities)
    {
        return true;
    }

    false
}

fn find_next_to_try(board: &SudokuBoard) -> Option<[usize; 2]> {
    let mut results: [Option<[usize; 2]>; 9] = [None; 9];

    for row in 0..9_usize {
        for col in 0..9_usize {
            if board.get_number(row, col).is_some() {
                continue;
            }

            let count = board.get_possibilities(row, col).count();
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

#[cfg(test)]
mod tests {
    use crate::sudoku_board::SudokuBoard;
    use crate::sudoku_examples::{EASY_LITERALS, HARD_LITERALS, MEDIUM_LITERALS};
    use crate::sudoku_solver::{develop, generate_possibilities, solve, unique_candidates_lines};

    #[test]
    fn util() {
    }

    #[test]
    fn develop_easy() {
        for mut board in EASY_LITERALS.map(SudokuBoard::from_literal) {
            while develop(&mut board) {}
            assert!(board.is_full());
        }
    }

    #[test]
    fn solve_easy() {
        for board in EASY_LITERALS.map(SudokuBoard::from_literal) {
            let solved = solve(&board);
            assert!(solved.is_some());
            assert!(solved.unwrap().is_full());
        }
    }

    #[test]
    fn solve_medium() {
        for board in MEDIUM_LITERALS.map(SudokuBoard::from_literal) {
            let solved = solve(&board);
            assert!(solved.is_some());
            assert!(solved.unwrap().is_full());
        }
    }

    #[test]
    fn solve_hard() {
        for board in HARD_LITERALS.map(SudokuBoard::from_literal) {
            let solved = solve(&board);
            println!("{:?}", solved);
            assert!(solved.is_some());
            assert!(solved.unwrap().is_full());
        }
    }

    #[test]
    fn unique_candidates_lines_col() {
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

        for example in examples {
            let mut board = SudokuBoard::from_literal(example);

            let possibilities = generate_possibilities(&board);
            let result = unique_candidates_lines::<true>(&mut board, &possibilities);
            assert!(result);
        }
    }
}