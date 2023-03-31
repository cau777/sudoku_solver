// All the observations that the Wasm code can make about a cell
export type Message =
    { t: "found", ms: number } |
    { t: "tried", num: number, row: number, col: number } |
    { t: "gaveUp" } |
    { t: "canContainOnly", num: number, row: number, col: number } |
    { t: "numberOnlyFitsInRow", num: number, row: number } |
    { t: "numberOnlyFitsInCol", num: number, col: number } |
    { t: "numberOnlyFitsInBlock", num: number, row: number, col: number };