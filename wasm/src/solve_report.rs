pub struct ReportStep<const SIZE: usize, const BLOCK_SIZE: usize> {
    pub message: String,
    pub highlight_row: Option<u8>,
    pub highlight_col: Option<u8>,
    pub highlight_block: Option<[u8; 2]>,
    pub literal: String
}