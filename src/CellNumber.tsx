import React from "react";

type Props = {
    num: number;
}

export const CellNumber: React.FC<Props> = (props) => {
    return (
        <div className={"cell-number"}>
            {props.num}
        </div>
    )
}