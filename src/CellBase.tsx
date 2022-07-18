import React, {PropsWithChildren} from "react";

type Props = {
    highlighted: boolean;
}

export const CellBase: React.FC<PropsWithChildren<Props>> = (props) => {
    return (
        <div className={"cell-base " + (props.highlighted ? "highlighted" : "")}>
            {props.children}
        </div>
    )
}