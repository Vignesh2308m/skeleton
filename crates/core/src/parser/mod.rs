use std::fs;
use std::io::Error;

pub mod pdf;
pub mod txt;
pub mod xlsx;

pub use self::pdf::Pdf;
pub use self::txt::Text;
pub use self::xlsx::Xlsx;

pub trait DocumentParser {
    fn new(path: &str) -> Result<Self, Error>
    where
        Self: Sized;

    fn read(&mut self) -> Result<&[u8], Error>;

    fn metadata(&self) -> Result<ParserMetadata, Error>;

    fn current_page(&self) -> usize {
        0
    }

    fn current_sheet(&self) -> String {
        String::new()
    }

    fn current_row(&self) -> u32 {
        0
    }

    fn current_column(&self) -> u32 {
        0
    }
}

#[derive(Debug, Clone)]
pub struct ParserMetadata {
    pub path: String,
    pub kind: &'static str,
    pub size_bytes: u64,
    pub details: ParserMetadataDetails,
}

#[derive(Debug, Clone)]
pub enum ParserMetadataDetails {
    Text,
    Pdf { page: usize },
    Xlsx { sheet: String, row: u32, column: u32 },
}

impl ParserMetadata {
    pub fn from_path(path: &str, kind: &'static str) -> Result<Self, Error> {
        let metadata = fs::metadata(path)?;

        let details = match kind {
            "text" => ParserMetadataDetails::Text,
            "pdf" => ParserMetadataDetails::Pdf { page: 0 },
            "xlsx" => ParserMetadataDetails::Xlsx {
                sheet: String::new(),
                row: 0,
                column: 0,
            },
            _ => ParserMetadataDetails::Text,
        };

        Ok(Self {
            path: path.to_string(),
            kind,
            size_bytes: metadata.len(),
            details,
        })
    }
}