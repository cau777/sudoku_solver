# Sudoku Solver

An algorithm made in rust to solve sudoku puzzles of various sizes efficiently. It's implemented in Rust and runs in the
browser using Web Assembly. The project includes a modern interface made in React to visualise the algorithm
step-by-step.

## Features
* 3 board options: 4x4 9x9 16x16
* Generation of random Sudoku puzzles
* Algorithm made in Rust to solve puzzles in milliseconds
* Step-by-step visualization of the solution with explanations

## How the [algorithm](https://github.com/cau777/sudoku_solver/blob/master/wasm/src/sudoku_solver.rs) works
It was inspired by some real-world Sudoku solving techniques, and aims to minimize guesses. The algorithm has a recursive 
idea, but is actually implemented iteratively using a Stack, It also uses bitwise operations
whenever possible to improve performance massively.
1) Load the board from a string representation
2) Search for cell whose value can be inferred. This is done in 2 ways:
   * Sole candidate: when a cell can only contain one number, because all the other ones are already taken in the row/column/block.
   * Unique candidate: when, in a row/column/block, a number can only be put in one cell. Because every number must appear
once in every row/column/block, if only one cell can fit a determined number, it's definitely there.
3) If the step 2 had success, complete that cell and do it again.
4) Check if the board is complete, if so, return.
5) Now, only guesses remain, so we have to find the cell with the least number of candidates.
6) Guess a number on that cell and execute step 2 in the modified board.

This [website](https://www.conceptispuzzles.com/index.aspx?uri=puzzle/sudoku/techniques) explains some of the logic.

## Screenshots
* ![Empty board](https://github.com/cau777/sudoku_solver/blob/master/screenshots/empty_board.png)
* ![Solution step](https://github.com/cau777/sudoku_solver/blob/master/screenshots/solution_step.png)