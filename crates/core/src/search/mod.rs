use std::io::{Error, ErrorKind};

use crate::parser::{DocumentParser, ParserMetadataDetails};
use crate::search::matcher::{MatchMetadata, SearchMatch};
use std::path::PathBuf;

pub mod matcher;

pub trait Search {
    fn search(&mut self, pattern: &[u8]) -> Result<Vec<SearchMatch>, Error>;
}

impl<T> Search for T
where
    T: DocumentParser,
{
    fn search(&mut self, pattern: &[u8]) -> Result<Vec<SearchMatch>, Error> {
        if pattern.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "search pattern must not be empty",
            ));
        }

        let data = self.read()?.to_vec();
        let metadata = self.metadata()?;
        let mut matches = Vec::new();

        for (index, window) in data.windows(pattern.len()).enumerate() {
            if window == pattern {
                let meta = match self.metadata()?.details {
                    ParserMetadataDetails::Text => {
                        let line_start = data[..index]
                            .iter()
                            .rposition(|byte| *byte == b'\n')
                            .map_or(0, |pos| pos + 1);
                        MatchMetadata::Text {
                            line: data[..index].iter().filter(|byte| **byte == b'\n').count() + 1,
                            column: index - line_start + 1,
                        }
                    }
                    ParserMetadataDetails::Pdf { page } => MatchMetadata::Pdf { page },
                    ParserMetadataDetails::Xlsx { sheet, row, column } => {
                        MatchMetadata::Xlsx { sheet, row, column }
                    }
                };

                matches.push(SearchMatch {
                    file: PathBuf::from(metadata.path.clone()),
                    start: index as u64,
                    end: (index + pattern.len() - 1) as u64,
                    metadata: meta,
                });
            }
        }

        Ok(matches)
    }
}

#[cfg(test)]
mod tests {
    use super::Search;
    use crate::parser::pdf::Pdf;
    use crate::parser::txt::Text;
    use crate::parser::DocumentParser;
    use crate::search::matcher::MatchMetadata;

    #[test]
    fn search_trait_finds_pattern_in_text() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}/../../data/intro.txt", manifest_dir);

        let mut parser = Text::new(&path).expect("failed to open text file");
        let matches = parser.search(b"abcde").expect("search failed");

        assert_eq!(matches.len(), 3, "expected one match on each line");
        match &matches[2].metadata {
            MatchMetadata::Text { line, column } => {
                assert_eq!((*line, *column), (3, 1));
            }
            _ => panic!("expected text metadata"),
        }
    }

    #[test]
    fn search_trait_uses_pdf_page_number() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}/../../data/test.pdf", manifest_dir);

        let mut parser = Pdf::new(&path).expect("failed to open pdf file");
        let pattern = parser.read().expect("PDF text extraction failed")[..1].to_vec();
        let matches = parser.search(&pattern).expect("search failed");

        assert!(!matches.is_empty(), "expected at least one pdf match");
        match &matches[0].metadata {
            MatchMetadata::Pdf { page } => {
                assert!(*page > 0, "expected a non-zero pdf page number")
            }
            MatchMetadata::Text { .. } => panic!("expected pdf metadata but got text metadata"),
            MatchMetadata::Xlsx { .. } => panic!("expected pdf metadata but got xlsx metadata"),
        }
    }

    #[test]
    fn search_trait_decodes_unicode_mapped_pdf_text() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}/../../data/test.pdf", manifest_dir);

        let mut parser = Pdf::new(&path).expect("failed to open pdf file");
        let matches = parser.search(b"a").expect("search failed");

        assert_eq!(matches.len(), 3, "expected all three PDF text matches");
        assert!(matches
            .iter()
            .all(|m| matches!(m.metadata, MatchMetadata::Pdf { page: 1 })));
    }
}
