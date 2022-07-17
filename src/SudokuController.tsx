import React from "react";
import {SudokuBoard} from "./SudokuBoard";

export const SudokuController: React.FC = (props) => {
    return (
        <div>
            <SudokuBoard blockSize={5} setNum={() => {
            }}></SudokuBoard>
        </div>
    )
}