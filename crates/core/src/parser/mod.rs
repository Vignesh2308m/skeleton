use std::fs;
use std::io::Error;

pub mod pdf;
pub mod txt;
pub mod xlsx;

pub use self::pdf::Pdf;
pub use self::txt::Text;
pub use self::xlsx::Xlsx;

pub trait Parser {
    fn new(path: &str) -> Result<Self, Error>
    where
        Self: Sized;

    fn read(&mut self) -> Result<&[u8], Error>;

    fn metadata(&self) -> Result<ParserMetadata, Error>;
}

#[derive(Debug, Clone)]
pub struct ParserMetadata {
    pub path: String,
    pub kind: &'static str,
    pub size_bytes: u64,
}

impl ParserMetadata {
    pub fn from_path(path: &str, kind: &'static str) -> Result<Self, Error> {
        let metadata = fs::metadata(path)?;

        Ok(Self {
            path: path.to_string(),
            kind,
            size_bytes: metadata.len(),
        })
    }
}