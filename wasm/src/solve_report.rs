use crate::number_options::NumberOptions;

pub struct ReportStep<const SIZE: usize> {
    pub message: String,
    pub highlight_row: Option<u8>,
    pub highlight_col: Option<u8>,
    pub highlight_block: Option<[u8; 2]>,
    pub possibilities: Vec<Vec<NumberOptions<SIZE>>>
}
