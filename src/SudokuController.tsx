import React, {useState} from "react";
import {SudokuBoard} from "./SudokuBoard";
import {Board} from "./board";
import init, {find_errors, random_board, solve} from "wasm";

type Props = {
    setLog: (log: string) => void;
}

export type Highlights = {
    highlightRow: number | null;
    highlightCol: number | null;
    highlightBlock: [number, number] | null;
}

type State = Highlights & {
    board: Board;
}

function defaultState(board: Board) {
    return {
        highlightBlock: null,
        highlightCol: null,
        highlightRow: null,
        board
    };
}

export const SudokuController: React.FC<Props> = (props) => {
    let [state, setState] = useState<State>(defaultState(Board.default(3)));
    
    function changeBoard(board: Board) {
        setState(s => ({...s, board}));
        check(board, false);
    }
    
    function check(board: Board, log: boolean) {
        init().then(() => {
            find_errors(board.toLiteral(), board.blockSize);
            let result = JSON.parse(find_errors(board.toLiteral(), board.blockSize));
            if (result) {
                switch (result.type) {
                    case "row":
                        if (log) props.setLog(`Your solution is wrong. See row ${result.value + 1}`);
                        setState(s => ({...s, highlightRow: result.value, highlightCol: null, highlightBlock: null}));
                        break;
                    case "col":
                        if (log) props.setLog(`Your solution is wrong. See column ${result.value + 1}`);
                        setState(s => ({...s, highlightRow: null, highlightCol: result.value, highlightBlock: null}));
                        break;
                    case "block":
                        if (log) props.setLog(`Your solution is wrong. See block ${result.value[0] + 1},${result.value[1] + 1}`);
                        setState(s => ({...s, highlightRow: null, highlightCol: null, highlightBlock: result.value}));
                        break;
                    
                }
            } else if (board.cells.every(o => o !== null)) {
                if (log) props.setLog("Your solution is right");
                setState(s => ({...s, highlightRow: null, highlightCol: null, highlightBlock: null}));
            } else {
                if (log) props.setLog("Your solution is incomplete");
            }
        })
    }
    
    function solveBoard(board: Board) {
        let start = Date.now();
        
        init().then(() => {
            let result = JSON.parse(solve(board.toLiteral(), board.blockSize, 0));
            if (result === null) {
                props.setLog("Couldn't find solution");
            } else {
                let solution = result.solution;
                props.setLog(`Found solution in ${Date.now() - start}ms`);
                let solved = Board.fromLiteral(solution, board.blockSize);
                console.log("res", result);
                setState(s => ({...s, board: solved}));
            }
        })
    }
    
    function clear() {
        setState(s => defaultState(Board.default(s.board.blockSize)));
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
            <SudokuBoard board={state.board} setBoard={changeBoard} highlightRow={state.highlightRow}
                         highlightCol={state.highlightCol} highlightBlock={state.highlightBlock}></SudokuBoard>
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
                <button onClick={() => check(state.board, true)}>Check</button>
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