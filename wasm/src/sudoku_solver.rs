use std::collections::LinkedList;
use crate::sudoku_board::SudokuBoard;

pub fn solve(board: &SudokuBoard) -> Option<SudokuBoard> {
    let mut stack = LinkedList::<SudokuBoard>::new();
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
        // println!("{} {} {:?}", row, col, possibilities);

        for possible in possibilities.as_vec() {
            // println!("{:?}", current);
            let mut board = current.clone();
            board.set_number(Some(possible), row, col);
            stack.push_front(board);
        }
    }

    None
}

fn develop(board: &mut SudokuBoard) -> bool {
    for row in 0..9_usize {
        for col in 0..9_usize {
            if board.get_number(row, col).is_some() { continue; }

            let possibilities = board.get_possibilities(row, col);
            if possibilities.count() == 1 {
                board.set_number(Some(possibilities.first().unwrap()), row, col);
                return true;
            }
        }
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
    use crate::sudoku_examples::{EASY_LITERALS, MEDIUM_LITERALS};
    use crate::sudoku_solver::{develop, solve};
    
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
}