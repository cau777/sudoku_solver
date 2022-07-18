import React from "react";
import {NumberCell} from "./NumberCell";
import {ColRowCell} from "./ColRowCell";
import {Board} from "./board";
import {CellBase} from "./CellBase";

type Props = {
    board: Board;
    highlight: [number, number]|null;
    setBoard: (board: Board) => void;
}

export const SudokuBoard: React.FC<Props> = (props) => {
    let blockSize = props.board.blockSize;
    let size = props.board.size;
    let board = props.board;
    let rows = [];
    
    function updateBoard(row: number, col: number, value: number | null) {
        props.setBoard(board.copy().set(row, col, value));
    }
    
    let firstRow = [
        <td key={"hidden"} className={"block-col-start block-row-start hidden"}>
            <CellBase highlighted={false}></CellBase>
        </td>
    ];
    
    for (let s = 0; s < size; s++) {
        firstRow.push(
            <td key={s} className={" block-row-start " + (s % blockSize === 0 ? " block-col-start " : "")}>
                <CellBase highlighted={props.highlight !== null && s === props.highlight[1]}>
                    <ColRowCell num={s + 1}></ColRowCell>
                </CellBase>
            </td>
        );
    }
    
    rows.push(<tr key={"col nums"}>{firstRow}</tr>);
    
    let index = 0;
    for (let r = 0; r < size; r++) {
        let cells = [
            <td key={"row nums " + r} className={" block-col-start " +
                (r % blockSize === 0 ? " block-row-start " : "")}>
                <CellBase highlighted={props.highlight !== null && r === props.highlight[0]}>
                    <ColRowCell num={r + 1}></ColRowCell>
                </CellBase>
            </td>
        ];
        
        for (let c = 0; c < size; c++) {
            cells.push(
                <td key={c} className={
                    (r % blockSize === 0 ? " block-row-start " : "") +
                    (c % blockSize === 0 ? " block-col-start " : "")}>
                    <CellBase highlighted={props.highlight !== null && (r === props.highlight[0] || c === props.highlight[1])}>
                        <NumberCell index={2 + index++} num={board.get(r, c)}
                                    setNum={(value) => updateBoard(r, c, value)}></NumberCell>
                    </CellBase>
                </td>
            );
        }
        rows.push(
            <tr key={r}>
                {cells}
            </tr>
        )
    }
    
    return (
        <div className={"sudoku-board"}>
            <table>
                <tbody>
                {rows}
                </tbody>
            </table>
        </div>
    )
}