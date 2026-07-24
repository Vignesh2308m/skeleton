use std::io::{Error, ErrorKind};

use crate::search::matcher::{SearchMatch, MatchMetadata};
use crate::parser::{DocumentParser, ParserMetadataDetails};
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
        let page = self.current_page();
        let sheet = self.current_sheet();
        let row = self.current_row();
        let column = self.current_column();
        let mut matches = Vec::new();
        let mut line_no = 0usize;

        for (index, window) in data.windows(pattern.len()).enumerate() {
            if window == pattern {
                let meta = match &metadata.details {
                    ParserMetadataDetails::Text => MatchMetadata::Text {
                        line: line_no,
                        column: index,
                    },
                    ParserMetadataDetails::Pdf { page } => MatchMetadata::Pdf { page: *page },
                    ParserMetadataDetails::Xlsx { sheet, row, column } => MatchMetadata::Xlsx {
                        sheet: sheet.clone(),
                        row: *row,
                        column: *column,
                    },
                };

                matches.push(SearchMatch {
                    file: PathBuf::from(metadata.path.clone()),
                    start: index as u64,
                    end: (index + pattern.len() - 1) as u64,
                    metadata: meta,
                });
            }

            if data.get(index) == Some(&b'\n') {
                line_no += 1;
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

        assert!(!matches.is_empty(), "expected at least one match");
    }

    #[test]
    fn search_trait_uses_pdf_page_number() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}/../../data/test.pdf", manifest_dir);

        let mut parser = Pdf::new(&path).expect("failed to open pdf file");
        let matches = parser.search(b"page:").expect("search failed");

        assert!(!matches.is_empty(), "expected at least one pdf match");
        match &matches[0].metadata {
            MatchMetadata::Pdf { page } => assert!(*page > 0, "expected a non-zero pdf page number"),
            MatchMetadata::Text { .. } => panic!("expected pdf metadata but got text metadata"),
            MatchMetadata::Xlsx { .. } => panic!("expected pdf metadata but got xlsx metadata"),
        }
    }
}
