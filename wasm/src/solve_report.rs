use crate::number_options::NumberOptions;

pub struct ReportStep {
    pub message: String,
    pub highlight_row: Option<u8>,
    pub highlight_col: Option<u8>,
    pub highlight_block: Option<[u8; 2]>,
    pub possibilities: Vec<Vec<NumberOptions>>
}

pub struct SolvedResult {
    pub board: Vec<u8>,
    // pub candidates: Vec<Vec<u8>>,
    pub steps: Option<Vec<ReportStep>>,
}
