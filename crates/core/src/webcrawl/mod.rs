use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::io; // Use std::io for io::Error
use std::path::PathBuf;
use std::str::Utf8Error; // For Utf8Error

// Removed the dependency on `crate::parser`
// use crate::parser::{ParserMetadata, ParserMetadataDetails};

pub mod dynamic;
pub mod r#static;

/// Custom error type for web parsing.
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Reqwest(reqwest::Error), // Requires 'reqwest' dependency
    Utf8(Utf8Error),
    // Add other specific errors if needed
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::Utf8(err)
    }
}

/// Defines the detailed metadata of a parsed document based on its type.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParserMetadataDetails {
    Text,
    Pdf {
        page: usize,
    },
    Xlsx {
        sheet: String,
        row: u32,
        column: u32,
    },
    Web,
    // Add more document types as needed
}

impl Default for ParserMetadataDetails {
    fn default() -> Self {
        ParserMetadataDetails::Text
    }
}

/// Holds comprehensive metadata for a parsed document.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ParserMetadata {
    pub path: String,
    pub kind: &'static str,
    pub file_name: String,
    pub extension: String,
    pub size_bytes: u64,
    pub last_modified: Option<DateTime<Utc>>,
    pub language: Option<String>,
    pub details: ParserMetadataDetails,
    pub checksum: String,
    pub tags: HashMap<String, String>,
}

impl ParserMetadata {
    /// Creates a new `ParserMetadata` instance from a given path and document kind.
    pub fn from_path(path_str: &str, kind: &'static str) -> Result<Self, io::Error> {
        let path = PathBuf::from(path_str);
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        let extension = path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();

        let metadata = std::fs::metadata(&path)?;
        let last_modified: Option<DateTime<Utc>> = metadata.modified().ok().map(DateTime::from);

        Ok(Self {
            path: path_str.to_string(),
            kind,
            file_name,
            extension,
            size_bytes: metadata.len(),
            last_modified,
            language: None, // Default to None, can be set later
            details: ParserMetadataDetails::default(),
            checksum: String::new(), // Default to empty, can be computed later
            tags: HashMap::new(),    // Default to empty, can be set later
        })
    }
}

/// Parses web content fetched from a URL into searchable bytes.
pub trait WebParser {
    fn new(url: &str) -> Result<Self, Error>
    where
        Self: Sized;

    async fn read(&mut self) -> Result<&[u8], Error>;

    async fn metadata(&self) -> Result<ParserMetadata, Error>;

    /// Describes the web content location that contains a byte in `read()`'s output.
    async fn metadata_at(&self, _byte_offset: usize) -> ParserMetadataDetails {
        self.metadata()
            .await
            .map(|metadata| metadata.details)
            .unwrap_or(ParserMetadataDetails::Text)
    }

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

#[cfg(test)]
mod tests {
    use super::{Error, ParserMetadata, ParserMetadataDetails, WebParser}; // Use our custom types

    struct TestWebParser {
        content: Vec<u8>,
    }

    impl WebParser for TestWebParser {
        fn new(_url: &str) -> Result<Self, Error> {
            Ok(Self {
                content: b"example web content".to_vec(),
            })
        }

        async fn read(&mut self) -> Result<&[u8], Error> {
            Ok(&self.content)
        }

        async fn metadata(&self) -> Result<ParserMetadata, Error> {
            Ok(ParserMetadata {
                path: "https://example.com".to_string(),
                kind: "web",
                file_name: "example.com".to_string(),
                extension: "html".to_string(),
                size_bytes: self.content.len() as u64,
                last_modified: None,
                language: None,
                details: ParserMetadataDetails::Web,
                checksum: String::new(),
                tags: HashMap::new(),
            })
        }
    }

    #[tokio::test]
    async fn web_parser_exposes_document_parser_style_defaults() {
        let mut parser = TestWebParser::new("https://example.com").expect("parser opens URL");

        assert_eq!(
            parser.read().await.expect("parser reads content"),
            b"example web content"
        );
        assert!(matches!(
            parser.metadata_at(0).await,
            ParserMetadataDetails::Web
        )); // Changed to Web
        assert_eq!(parser.current_page(), 0);
        assert_eq!(parser.current_sheet(), "");
        assert_eq!(parser.current_row(), 0);
        assert_eq!(parser.current_column(), 0);
    }
}
