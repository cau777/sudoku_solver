import React, {useState} from "react";
import {SudokuCell} from "./SudokuCell";
import {CellNumber} from "./CellNumber";
import {Board} from "./board";

type Props = {
    blockSize: number;
    setNum: (row: number, col: number, num?: number) => void;
}

export const SudokuBoard: React.FC<Props> = (props) => {
    let blockSize = props.blockSize;
    let size = blockSize * blockSize;
    let rows = [];
    
    let [state, setState] = useState<Board>(Board.default(blockSize));
    
    let firstRow = [
        <td key={"hidden"} className={"block-col-start block-row-start hidden"}>
            <div className={"border"}></div>
        </td>
    ];
    
    for (let s = 0; s < size; s++) {
        firstRow.push(
            <td key={s} className={" block-row-start " + (s % blockSize === 0 ? " block-col-start " : "")}>
                <div className={"border"}>
                    <CellNumber num={s + 1}></CellNumber>
                </div>
            </td>
        );
    }
    
    rows.push(<tr key={"col nums"}>{firstRow}</tr>);
    
    let index = 0;
    for (let r = 0; r < size; r++) {
        let cells = [
            <td key={"row nums " + r} className={" block-col-start " +
                (r % blockSize === 0 ? " block-row-start " : "")}>
                <div className={"border"}>
                    <CellNumber num={r + 1}></CellNumber>
                </div>
            </td>
        ];
        
        for (let c = 0; c < size; c++) {
            cells.push(
                <td key={c} className={
                    (r % blockSize === 0 ? " block-row-start " : "") +
                    (c % blockSize === 0 ? " block-col-start " : "")}>
                    <div className={"border"}>
                        <SudokuCell index={2 + index++} num={state.get(r, c)}
                                    setNum={(value) => setState(state.copy().set(r, c, value))}></SudokuCell>
                    </div>
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