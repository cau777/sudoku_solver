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
    private size: number;
    // [key: number]: number | null;
    
    public constructor(private blockSize: number,
                       private cells: (number | null)[]) {
        this.size = blockSize * blockSize;
        // cells.forEach((value, index) => this[index] = value);
    }
    
    public static default(blockSize: number) {
        return new Board(blockSize, new Array(blockSize * blockSize * blockSize * blockSize).fill(null))
    }
    
    public get(row: number, col: number) {
        // return this[row * this.size + col];
        return this.cells[row * this.size + col];
    }
    
    public set(row: number, col: number, value: number | null) {
        // this[row * this.size + col] = value;
        this.cells[row * this.size + col] = value;
        return this;
    }
    
    public copy() {
        return new Board(this.blockSize, [...this.cells]);
    }
}