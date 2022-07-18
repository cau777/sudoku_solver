// export type Board = (number | null)[];
//
// export function defaultBoard(size: number) {
//     return new Array(size).fill(null);
// }
//
// export function setCell(board: Board, row: number, col: number, size: number, value: number | undefined) {
//     board[row + "-" + col] =
// }

export class Board {
    readonly size: number;
    
    public constructor(public blockSize: number,
                       public cells: (number | null)[]) {
        this.size = blockSize * blockSize;
    }
    
    public static default(blockSize: number) {
        return new Board(blockSize, new Array(blockSize * blockSize * blockSize * blockSize).fill(null))
    }
    
    public static fromLiteral(literal: string, blockSize: number) {
        let array = literal
            .replace("\n", " ")
            .split(" ")
            .filter(o => o.length !== 0)
            .map(o => Number.parseInt(o))
            .map(o => Number.isNaN(o) ? null : o);
        
        return new Board(blockSize, array);
    }
    
    public get(row: number, col: number) {
        return this.cells[row * this.size + col];
    }
    
    public set(row: number, col: number, value: number | null) {
        this.cells[row * this.size + col] = value;
        return this;
    }
    
    public copy() {
        return new Board(this.blockSize, [...this.cells]);
    }
    
    public toLiteral() {
        return this.cells.reduce((acc, value) => acc + (value ?? "_") + " ", "");
    }
}