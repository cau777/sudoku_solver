use std::collections::{LinkedList};
use crate::number_options::NumberOptions;
use crate::sudoku_board::{SudokuBoard};
use crate::util::Array2D;

type Possibilities<const SIZE: usize> = Array2D<NumberOptions, SIZE>;

pub fn solve<const SIZE: usize, const BLOCK_SIZE: usize>(board: &SudokuBoard<SIZE, BLOCK_SIZE>)
                                                         -> Option<SudokuBoard<SIZE, BLOCK_SIZE>> {
    let mut stack = LinkedList::<SudokuBoard<SIZE, BLOCK_SIZE>>::new();

    stack.push_front(board.clone());

    while !stack.is_empty() {
        let mut current = stack.pop_front().unwrap();

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

fn sole_candidates<const SIZE: usize, const BLOCK_SIZE: usize>(
    board: &mut SudokuBoard<SIZE, BLOCK_SIZE>, possibilities: &Array2D<NumberOptions, SIZE>) -> bool {
    for row in 0..SIZE {
        for col in 0..SIZE {
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

fn unique_candidates_lines<const INVERT: bool, const SIZE: usize, const BLOCK_SIZE: usize>(
    board: &mut SudokuBoard<SIZE, BLOCK_SIZE>, possibilities: &Possibilities<SIZE>) -> bool {
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
            // println!("{:?}", board);
            // println!("{:?}", unique);
            // println!("{:?}", at_least_one);
            // println!("{:?}", more_than_one);
            let first = unique.as_vec()[0];

            for j in 0..SIZE {
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

fn unique_candidates_blocks<const SIZE: usize, const BLOCK_SIZE: usize>(
    board: &mut SudokuBoard<SIZE, BLOCK_SIZE>, possibilities: &Possibilities<SIZE>) -> bool {
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
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn generate_possibilities<const SIZE: usize, const BLOCK_SIZE: usize>(board: &SudokuBoard<SIZE, BLOCK_SIZE>) -> Possibilities<SIZE> {
    let mut result = [[NumberOptions::default(); SIZE]; SIZE];

    for row in 0..SIZE {
        for col in 0..SIZE {
            result[row][col] = board.get_possibilities(row, col);
        }
    }

    result
}

fn develop<const SIZE: usize, const BLOCK_SIZE: usize>(board: &mut SudokuBoard<SIZE, BLOCK_SIZE>) -> bool {
    let possibilities = generate_possibilities(board);

    if sole_candidates(board, &possibilities)
        || unique_candidates_lines::<false, SIZE, BLOCK_SIZE>(board, &possibilities)
        || unique_candidates_lines::<true, SIZE, BLOCK_SIZE>(board, &possibilities)
        || unique_candidates_blocks(board, &possibilities)
    {
        return true;
    }

    false
}

fn find_next_to_try<const SIZE: usize, const BLOCK_SIZE: usize>(board: &SudokuBoard<SIZE, BLOCK_SIZE>) -> Option<[usize; 2]> {
    let mut results: [Option<[usize; 2]>; SIZE] = [None; SIZE];

    for row in 0..SIZE {
        for col in 0..SIZE {
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
    use std::fs::OpenOptions;
    use std::io::{BufRead, BufReader};
    use crate::sudoku_board::{DefaultBoard};
    use crate::sudoku_examples::{EASY_LITERALS, HARD_LITERALS, MEDIUM_LITERALS};
    use crate::sudoku_solver::{develop, generate_possibilities, solve, unique_candidates_lines};

    #[test]
    fn util() {}

    #[test]
    fn develop_easy() {
        for mut board in EASY_LITERALS.map(DefaultBoard::from_literal) {
            while develop(&mut board) {}
            assert!(board.is_full());
        }
    }

    #[test]
    fn solve_easy() {
        for board in EASY_LITERALS.map(DefaultBoard::from_literal) {
            let solved = solve(&board);
            assert!(solved.is_some());
            assert!(solved.unwrap().is_full());
        }
    }

    #[test]
    fn solve_medium() {
        for board in MEDIUM_LITERALS.map(DefaultBoard::from_literal) {
            let solved = solve(&board);
            assert!(solved.is_some());
            assert!(solved.unwrap().is_full());
        }
    }

    #[test]
    fn solve_hard() {
        for board in HARD_LITERALS.map(DefaultBoard::from_literal) {
            let solved = solve(&board);
            println!("{:?}", solved);
            assert!(solved.is_some());
            assert!(solved.unwrap().is_full());
        }
    }

    #[test]
    fn unique_candidates_lines_test() {
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
            let possibilities = generate_possibilities(&example);
            let result = unique_candidates_lines::<true, 9, 3>(&mut example, &possibilities);
            assert!(result);
        }
    }

    #[test]
    fn file_4000() {
        let file = OpenOptions::new()
            .read(true)
            .open("./test_data/tests_4000.csv").unwrap(); // Subset from https://www.kaggle.com/datasets/bryanpark/sudoku?resource=download

        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let v: Vec<String> = line.split(',').map(String::from).collect();

            let input = DefaultBoard::from_literal(&v[0]);
            let expected = DefaultBoard::from_literal(&v[1]);

            let result = solve(&input);

            assert!(result.is_some());
            assert_eq!(result.unwrap(), expected);
        }
    }
}