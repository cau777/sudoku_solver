import React, {useState} from "react";
import {SudokuBoard} from "./SudokuBoard";
import {Board, Highlights} from "./board";
import init, {find_errors, random_board, solve} from "wasm";
import {AllNull} from "./util";
import {useTranslation} from "react-i18next";
import {Message} from "./Message";

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
    message: Message;
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
    let {t} = useTranslation();
    
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
                        if (log)
                            props.setLog(t("wrongSolutionRow", {row: result.value + 1}));
                        setState(s => ({...s, highlightRow: result.value, highlightCol: null, highlightBlock: null}));
                        break;
                    case "col":
                        if (log)
                            props.setLog(t("wrongSolutionCol", {col: result.value + 1}));
                        setState(s => ({...s, highlightRow: null, highlightCol: result.value, highlightBlock: null}));
                        break;
                    case "block":
                        if (log)
                            props.setLog(t("wrongSolutionBlock", {blockRow: result.value[0] + 1, blockCol: result.value[1] + 1}))
                        setState(s => ({...s, highlightRow: null, highlightCol: null, highlightBlock: result.value}));
                        break;
                    
                }
            } else {
                setState(s => ({...s, highlightRow: null, highlightCol: null, highlightBlock: null}));
                if (log) {
                    if (board.cells.every(o => o !== null)) props.setLog(t("rightSolution"));
                    else props.setLog(t("incompleteSolution"));
                }
            }
            
        })
    }
    
    function solveBoard(board: Board, recordSteps: number) {
        init().then(() => {
            let result: Step[] | null = JSON.parse(solve(board.toLiteral(), board.blockSize, recordSteps));
            if (result === null) {
                props.setLog(t("noSolution"));
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
        // Translates the message using its key and other values as params
        let message = t(steps[index].message.t, {...steps[index].message});
        
        if (steps.length !== 1)
            // If there are multiple messages, display the step number and then the message
            props.setLog(t("step", {num: index+1, message}));
        else
            // If there's only one message, just display it
            props.setLog(message);
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
            props.setLog(t("generatedRandom",{time: Date.now() - start}));
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
                <button onClick={() => check(focusBoard, true)}>{t("checkButton")}</button>
                <button onClick={clear}>{t("clearButton")}</button>
                <div>
                    <hr/>
                </div>
                <div className={"subtitle"}>{t("generate")}</div>
                <button onClick={() => randomBoard(0.75)}>{t("generateRandomButton", {perc: 75})}</button>
                <button onClick={() => randomBoard(0.50)}>{t("generateRandomButton", {perc: 50})}</button>
                <button onClick={() => randomBoard(0.25)}>{t("generateRandomButton", {perc: 25})}</button>
                <button onClick={() => randomBoard(0.1)}>{t("generateRandomButton", {perc: 10})}</button>
                <div>
                    <hr/>
                </div>
                <div className={"subtitle"}>{t("solve")}</div>
                
                {state.steps === null ?
                    <>
                        <button onClick={() => solveBoard(state.board, 0)}>{t("solveButton")}</button>
                        <button onClick={() => solveBoard(state.board, 1000)}>{t("solveStepsButton")}</button>
                    </>
                    :
                    <>
                        <button onClick={hideSolution}>{t("hideSolutionButton")}</button>
                        <button disabled={state.currentStep <= 0}
                                onClick={() => changeCurrentStep(state.currentStep! - 1, state.steps!)}>
                            {t("prevStepButton")}
                        </button>
                        <button disabled={state.currentStep <= 9}
                                onClick={() => changeCurrentStep(state.currentStep! - 10, state.steps!)}>
                            {t("prev10StepsButton")}
                        </button>
                        <button disabled={state.currentStep >= state.steps!.length - 1}
                                onClick={() => changeCurrentStep(state.currentStep! + 1, state.steps!)}>
                            {t("nextStepButton")}
                        </button>
                        <button disabled={state.currentStep >= state.steps!.length - 10}
                                onClick={() => changeCurrentStep(state.currentStep! + 10, state.steps!)}>
                            {t("next10StepsButton")}
                        </button>
                    </>
                }
            
            </div>
        </div>
    )
}