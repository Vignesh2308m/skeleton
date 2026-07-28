use std::io::Error;

use crate::parser::{ParserMetadata, ParserMetadataDetails};

pub mod dynamic;
pub mod r#static;

/// Parses web content fetched from a URL into searchable bytes.
pub trait WebParser {
    fn new(url: &str) -> Result<Self, Error>
    where
        Self: Sized;

    fn read(&mut self) -> Result<&[u8], Error>;

    fn metadata(&self) -> Result<ParserMetadata, Error>;

    /// Describes the web content location that contains a byte in `read()`'s output.
    fn metadata_at(&self, _byte_offset: usize) -> ParserMetadataDetails {
        self.metadata()
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
    use std::io::Error;

    use super::WebParser;
    use crate::parser::{ParserMetadata, ParserMetadataDetails};

    struct TestWebParser {
        content: Vec<u8>,
    }

    impl WebParser for TestWebParser {
        fn new(_url: &str) -> Result<Self, Error> {
            Ok(Self {
                content: b"example web content".to_vec(),
            })
        }

        fn read(&mut self) -> Result<&[u8], Error> {
            Ok(&self.content)
        }

        fn metadata(&self) -> Result<ParserMetadata, Error> {
            Ok(ParserMetadata {
                path: "https://example.com".to_string(),
                kind: "web",
                size_bytes: self.content.len() as u64,
                details: ParserMetadataDetails::Text,
            })
        }
    }

    #[test]
    fn web_parser_exposes_document_parser_style_defaults() {
        let mut parser = TestWebParser::new("https://example.com").expect("parser opens URL");

        assert_eq!(
            parser.read().expect("parser reads content"),
            b"example web content"
        );
        assert!(matches!(parser.metadata_at(0), ParserMetadataDetails::Text));
        assert_eq!(parser.current_page(), 0);
        assert_eq!(parser.current_sheet(), "");
        assert_eq!(parser.current_row(), 0);
        assert_eq!(parser.current_column(), 0);
    }
}
