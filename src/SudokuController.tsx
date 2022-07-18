import React, {useState} from "react";
import {SudokuBoard} from "./SudokuBoard";
import {Board} from "./board";
import init, {find_errors, random_board, solve} from "wasm";

type Props = {
    setLog: (log: string) => void;
}

type State = {
    board: Board;
    highlight: [number, number] | null;
}

export const SudokuController: React.FC<Props> = (props) => {
    let [state, setState] = useState<State>({board: Board.default(3), highlight: null});
    
    function changeBoard(board: Board) {
        init().then(() => {
            let result = find_errors(board.toLiteral(), board.blockSize);
            setState(s => ({...s, highlight: JSON.parse(result)}));
        })
        setState(s => ({...s, board: board}));
    }
    
    function check() {
        let board = state!.board;
        if (!board.cells.every(o => o !== null)) {
            props.setLog("Your solution is incomplete");
            return;
        }
        
        init().then(() => {
            let result = JSON.parse(find_errors(board.toLiteral(), board.blockSize));
            if (result)
                props.setLog(`Your solution is wrong. See number at row ${result[0] + 1} column ${result[1] + 1}`);
            else
                props.setLog("Your solution is right");
        })
    }
    
    function solveBoard(board: Board) {
        let start = Date.now();
        
        init().then(() => {
            let result = solve(board.toLiteral(), board.blockSize, 0); // TODO: steps
            if (result.length === 0) {
                props.setLog("Couldn't find solution");
            } else {
                props.setLog(`Found solution in ${Date.now() - start}ms`);
                let solved = Board.fromLiteral(result, board.blockSize);
                console.log("res", result);
                setState(s => ({...s, board: solved}));
            }
        })
    }
    
    function clear() {
        setState({board: Board.default(state!.board.blockSize), highlight: null});
    }
    
    function randomBoard(coverage: number) {
        init().then(() => {
            let start = Date.now();
            let blockSize = state!.board.blockSize;
            let result = random_board(coverage, blockSize);
            let board = Board.fromLiteral(result, blockSize);
            setState(s => ({...s, board}));
            props.setLog(`Generated random board in ${Date.now() - start}ms`)
        })
    }
    
    return (
        <div className={"sudoku-controller"}>
            <SudokuBoard highlight={state.highlight} board={state.board} setBoard={changeBoard}></SudokuBoard>
            <div className={"buttons"}>
                <select defaultValue={3}
                        onChange={(e) => setState({
                            ...state,
                            board: Board.default(Number.parseInt(e.currentTarget.value))
                        })}>
                    <option value={2}>4x4</option>
                    <option value={3}>9x9</option>
                    <option value={4}>16x16</option>
                </select>
                <button onClick={check}>Check</button>
                <button onClick={() => solveBoard(state.board)}>Solve</button>
                <button onClick={clear}>Clear</button>
                <div>
                    <hr/>
                </div>
                <button onClick={() => randomBoard(0.75)}>Random 75%</button>
                <button onClick={() => randomBoard(0.50)}>Random 50%</button>
                <button onClick={() => randomBoard(0.25)}>Random 25%</button>
                <button onClick={() => randomBoard(0.1)}>Random 10%</button>
            </div>
        </div>
    )
}