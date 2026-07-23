use std::io::{Error, Write};

use crate::search::matcher::{SearchMatch, MatchMetadata};

pub trait Printer {
    fn print(&self, matches: &[SearchMatch], writer: &mut dyn Write) -> Result<(), Error>;
}

pub struct PrettyPrinter;

impl Printer for PrettyPrinter {
    fn print(&self, matches: &[SearchMatch], writer: &mut dyn Write) -> Result<(), Error> {
        for m in matches {
            match &m.metadata {
                MatchMetadata::Text { line, .. } => {
                    writeln!(writer, "{}| {}, {}", line, m.start, m.end)?;
                }
                MatchMetadata::Pdf { page } => {
                    writeln!(writer, "page {}| {}, {}", page, m.start, m.end)?;
                }
                MatchMetadata::Xlsx { sheet, row, column } => {
                    writeln!(writer, "{}:{}:{}| {}, {}", sheet, row, column, m.start, m.end)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{PrettyPrinter, Printer};
    use crate::search::matcher::{SearchMatch, MatchMetadata};

    #[test]
    fn printer_formats_matches() {
        let printer = PrettyPrinter;
        let matches = vec![SearchMatch {
            file: std::path::PathBuf::from("/tmp/xyz"),
            start: 10,
            end: 15,
            metadata: MatchMetadata::Text { line: 3, column: 10 },
        }];

        let mut output = Vec::new();
        printer
            .print(&matches, &mut output)
            .expect("printer should format matches");

        let rendered = String::from_utf8(output).expect("output should be utf-8");
        assert!(rendered.contains("3| 10, 15"));
    }
}
