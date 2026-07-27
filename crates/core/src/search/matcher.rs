use std::path::PathBuf;

pub struct SearchMatch {
    pub file: PathBuf,
    pub start: u64,
    pub end: u64,
    pub metadata: MatchMetadata,
}

pub enum MatchMetadata {
    Text {
        line: usize,
        column: usize,
    },
    Pdf {
        page: usize,
    },
    Xlsx {
        sheet: String,
        row: u32,
        column: u32,
    },
}
