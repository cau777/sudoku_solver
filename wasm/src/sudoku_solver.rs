use std::collections::LinkedList;
use crate::sudoku_board::SudokuBoard;

fn solve(board: &SudokuBoard) -> Option<SudokuBoard> {
    let mut stack = LinkedList::<SudokuBoard>::new();
    stack.push_front(board.clone());

    let mut i = 0;
    while !stack.is_empty() {
        // if i == 15 {break;}
        // println!("{:?}", board);
        // i+=1;

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
    use crate::sudoku_solver::{develop, solve};

    fn get_easy() -> Vec<SudokuBoard> {
        vec![
            SudokuBoard::from_literal("
            _ _ _ _ 1 4 _ _ 3
            _ _ 3 2 _ _ _ 1 _
            _ 2 1 9 8 _ 4 _ _
            2 8 _ _ 9 5 _ _ 4
            _ _ _ 4 2 8 _ _ _
            9 _ _ 6 7 _ _ 2 5
            _ _ 5 _ 4 6 7 8 _
            _ 4 _ _ _ 2 6 _ _
            6 _ _ 8 3 _ _ _ _"),
            SudokuBoard::from_literal("
            _ 3 _ _ 1 _ _ 6 _
            7 5 _ _ 3 _ _ 4 8
            _ _ 6 9 8 4 3 _ _
            _ _ 3 _ _ _ 8 _ _
            9 1 2 _ _ _ 6 4 7
            _ _ 4 _ _ _ 5 _ _
            _ _ 1 6 7 5 2 _ _
            6 8 _ _ 9 _ _ 1 5
            _ 9 _ _ 4 _ _ 3 _"),
            SudokuBoard::from_literal("
            _ _ _ 9 _ 5 _ 6 _
            1 6 _ _ _ 8 _ _ _
            _ _ _ _ 4 _ _ 1 3
            _ 2 _ 5 _ _ 8 _ _
            7 3 9 _ 8 _ _ 4 5
            _ _ 8 _ _ _ 2 _ 9
            3 _ _ _ 7 _ _ 2 _
            _ 8 2 4 5 _ 3 _ 7
            9 5 _ _ _ _ _ 8 _
            "),
        ]
    }

    fn get_hard() -> Vec<SudokuBoard> {
        vec![
            SudokuBoard::from_literal("
            6 3 _ _ _ _ _ 8 1
            _ 2 _ _ _ 3 _ _ _
            _ _ _ _ 1 7 4 3 _
            _ 9 _ 4 _ _ 5 7 _
            _ _ _ 7 6 2 _ _ _
            _ 8 _ _ _ _ 6 _ _
            _ 6 _ _ 2 _ _ _ _
            3 _ 9 _ _ _ _ 6 _
            _ _ _ _ _ _ _ _ 9
            ")
        ]
    }

    fn get_super_hard() -> Vec<SudokuBoard> {
        vec![
            SudokuBoard::from_literal("
            _ _ _ _ _ _ _ _ _
            _ _ _ _ _ 3 _ 8 5
            _ _ 1 _ 2 _ _ _ _
            _ _ _ 5 _ 7 _ _ _
            _ _ 4 _ _ _ 1 _ _
            _ 9 _ _ _ _ _ _ _
            5 _ _ _ _ _ _ 7 3
            _ _ 2 _ 1 _ _ _ _
            _ _ _ _ 4 _ _ _ 9
            ")
        ]
    }

    #[test]
    fn develop_very_easy() {
        for mut board in get_easy() {
            while develop(&mut board) {}
            assert!(board.is_full());
        }
    }

    #[test]
    fn develop_hard() {
        for board in get_hard() {
            let solved = solve(&board);
            println!("{:?}", solved);
            assert!(solved.is_some());
            assert!(solved.unwrap().is_full());
        }
    }
}