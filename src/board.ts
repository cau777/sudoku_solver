
export class Board {
    readonly size: number;
    
    public constructor(public blockSize: number,
                       public cells: (number | null)[]) {
        this.size = blockSize * blockSize;
    }
    
    // Create an empty board
    public static default(blockSize: number) {
        return new Board(blockSize, new Array(blockSize * blockSize * blockSize * blockSize).fill(null))
    }
    
    // A literal a continuous string representation of the board, in the format "1 2 3 _ _ 6 7 8 _"
    // that can contain new lines
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
    
    // Deep copy of the object
    public copy() {
        return new Board(this.blockSize, [...this.cells]);
    }
    
    // Converts the board to the format "1 2 3 _ _ 6 7 8 _"
    public toLiteral() {
        return this.cells.reduce((acc, value) => acc + (value ?? "_") + " ", "");
    }
}

export type Highlights = {
    highlightRow: number | null;
    highlightCol: number | null;
    highlightBlock: [number, number] | null;
}