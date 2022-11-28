use json::{JsonValue, object};

pub enum Message {
    Found(u64),
    Tried(u8, usize, usize),
    GaveUp,
    CanContainOnly(u8, usize, usize),
    NumberOnlyFitsInCol(u8, usize),
    NumberOnlyFitsInRow(u8, usize),
    NumberOnlyFitsInBlock(u8, usize, usize),
}

impl Message {
    pub fn to_object(self) -> JsonValue {
        use Message::*;
        match self {
            Found(ms) => object! {
                t: "found",
                ms: ms
            },
            Tried(num, row, col) => object! {
                t: "tried",
                num: num,
                row: row,
                col: col
            },
            GaveUp => object! {
                t: "gaveUp"
            },
            CanContainOnly(num, row, col)=>object! {
                t: "canContainOnly",
                num: num,
                row: row,
                col: col
            },
            NumberOnlyFitsInRow(num, row) => object! {
                t: "numberOnlyFitsInRow",
                num: num,
                row: row
            },
            NumberOnlyFitsInCol(num, col) => object! {
                t: "numberOnlyFitsInCol",
                num: num,
                col: col
            },
            NumberOnlyFitsInBlock(num, row, col) => object! {
                t: "numberOnlyFitsInBlock",
                num: num,
                row: row,
                col: col
            }
        }
    }
}

pub struct ReportStep<const SIZE: usize, const BLOCK_SIZE: usize> {
    pub message: Message,
    pub highlight_row: Option<u8>,
    pub highlight_col: Option<u8>,
    pub highlight_block: Option<[u8; 2]>,
    pub literal: String
}
