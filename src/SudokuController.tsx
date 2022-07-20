import React, {useState} from "react";
import {SudokuBoard} from "./SudokuBoard";
import {Board, Highlights} from "./board";
import init, {find_errors, random_board, solve} from "wasm";
import {AllNull} from "./util";

type Props = {
    setLog: (log: string) => void;
}

type State = (Solution | AllNull<Solution>) & Highlights & {
    board: Board;
}

type Solution = {
    steps: Step[];
    currentStep: number;
}

type Step = Highlights & {
    message: string;
    literal: string;
}

function defaultState(board: Board): State {
    return {
        highlightBlock: null,
        highlightCol: null,
        highlightRow: null,
        steps: null,
        currentStep: null,
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
            } else {
                setState(s => ({...s, highlightRow: null, highlightCol: null, highlightBlock: null}));
                if (log) {
                    if (board.cells.every(o => o !== null)) props.setLog("Your solution is right");
                    else props.setLog("Your solution is incomplete");
                }
            }
            
        })
    }
    
    function solveBoard(board: Board, recordSteps: number) {
        init().then(() => {
            let result: Step[] | null = JSON.parse(solve(board.toLiteral(), board.blockSize, recordSteps));
            if (result === null) {
                props.setLog("Couldn't find solution");
            } else {
                setState(s => ({...s, currentStep: 0, steps: result!}));
                changeCurrentStep(0, result);
            }
        })
    }
    
    function hideSolution() {
        setState(s => ({...s, currentStep: null, steps: null}));
    }
    
    function changeCurrentStep(index: number, steps: Step[]) {
        if (index < 0 || index >= steps.length) return;
        
        props.setLog(steps[index].message);
        setState(s => ({...s, currentStep: index, steps}));
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
            hideSolution();
            setState(s => ({...s, board}));
            props.setLog(`Generated random board in ${Date.now() - start}ms`);
        })
    }
    
    let focus = state.steps !== null ? state.steps[state.currentStep] : state;
    let focusBoard =state.steps !== null ? Board.fromLiteral(state.steps[state.currentStep].literal, state.board.blockSize) : state.board;
    
    return (
        <div className={"sudoku-controller"}>
            <SudokuBoard board={focusBoard} setBoard={changeBoard} highlightRow={focus.highlightRow}
                highlightCol={focus.highlightCol} highlightBlock={focus.highlightBlock}
                readonly={state.steps !== null}></SudokuBoard>
            <div className={"buttons"}>
                <select defaultValue={3}
                        onChange={(e) => setState(defaultState(Board.default(Number.parseInt(e.currentTarget.value))))}>
                    <option value={2}>4x4</option>
                    <option value={3}>9x9</option>
                    <option value={4}>16x16</option>
                </select>
                <button onClick={() => check(focusBoard, true)}>Check</button>
                <button onClick={clear}>Clear</button>
                <div>
                    <hr/>
                </div>
                <button onClick={() => randomBoard(0.75)}>Random 75%</button>
                <button onClick={() => randomBoard(0.50)}>Random 50%</button>
                <button onClick={() => randomBoard(0.25)}>Random 25%</button>
                <button onClick={() => randomBoard(0.1)}>Random 10%</button>
                <div>
                    <hr/>
                </div>
                
                {state.steps === null ?
                    <>
                        <button onClick={() => solveBoard(state.board, 0)}>Solve</button>
                        <button onClick={() => solveBoard(state.board, 200)}>Solve step-by-step</button>
                    </>
                    :
                    <>
                        <button onClick={hideSolution}>Hide solution</button>
                        <button disabled={state.currentStep <= 0}
                                onClick={() => changeCurrentStep(state.currentStep! - 1, state.steps!)}>Prev step
                        </button>
                        <button disabled={state.currentStep >= state.steps!.length - 1}
                                onClick={() => changeCurrentStep(state.currentStep! + 1, state.steps!)}>Next step
                        </button>
                    </>
                }
            
            </div>
        </div>
    )
}