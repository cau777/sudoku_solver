import React from "react";

type Props = {
    index: number;
    num: number | null;
    setNum: (val: number | null) => void;
}

const ignoredKeys = ["Tab", "Control", "Shift", "Alt", "CapsLock", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12"];

export const NumberCell: React.FC<Props> = (props) => {
    function keyDown(e: React.KeyboardEvent<HTMLInputElement>) {
        let key = e.key;
        console.log(key);
        if (ignoredKeys.includes(key)) return;
        
        let value = Number.parseInt(key);
        if (Number.isNaN(value) || value < 1 || value > 9)
            props.setNum(null);
        else
            props.setNum(value);
        
        e.preventDefault();
    }
    
    return (
        <input tabIndex={props.index} className={"sudoku-cell"} defaultValue={props.num ?? ""} onKeyDown={keyDown}></input>
    )
}