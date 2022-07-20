import React from "react";

type Props = {
    index: number;
    value: number | null;
    setNum: (val: number | null) => void;
    max: number;
}

export const NumberCell: React.FC<Props> = (props) => {
    return (
        <input tabIndex={props.index} className={"sudoku-cell"} value={(props.value ?? "") + ""} onChange={e => {
            let num = Number.parseInt(e.currentTarget.value);
            if (isNaN(num) || num < 1)
                props.setNum(null);
            else if (num > props.max)
                props.setNum(props.max);
            else
                props.setNum(num)
        }}></input>
    )
}