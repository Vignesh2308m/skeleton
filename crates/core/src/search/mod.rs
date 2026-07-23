use std::io::{Error, ErrorKind};

use crate::search::matcher::{SearchMatch, MatchMetadata};
use crate::parser::DocumentParser;
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

        let metadata = self.metadata()?;
        let data = self.read()?;
        let mut matches = Vec::new();
        let mut line_no = 0usize;

        for (index, window) in data.windows(pattern.len()).enumerate() {
            if window == pattern {
                let meta = match metadata.kind {
                    "text" => MatchMetadata::Text {
                        line: line_no,
                        column: index,
                    },
                    "pdf" => MatchMetadata::Pdf { page: 0 },
                    "xlsx" => MatchMetadata::Xlsx {
                        sheet: "".to_string(),
                        row: 0,
                        column: 0,
                    },
                    _ => MatchMetadata::Text {
                        line: line_no,
                        column: index,
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
    use crate::parser::txt::Text;
    use crate::parser::DocumentParser;

    #[test]
    fn search_trait_finds_pattern_in_text() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}/../../data/intro.txt", manifest_dir);

        let mut parser = Text::new(&path).expect("failed to open text file");
        let matches = parser.search(b"abcde").expect("search failed");

        assert!(!matches.is_empty(), "expected at least one match");
    }
}
