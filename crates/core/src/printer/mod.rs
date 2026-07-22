use std::io::{Error, Write};

use crate::matcher::Match;

pub trait Printer {
    fn print(&self, matches: &[Match], writer: &mut dyn Write) -> Result<(), Error>;
}

pub struct PrettyPrinter;

impl Printer for PrettyPrinter {
    fn print(&self, matches: &[Match], writer: &mut dyn Write) -> Result<(), Error> {
        for m in matches {
            writeln!(writer, "{}| {}, {}", m.line_no, m.start, m.end)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{PrettyPrinter, Printer};
    use crate::matcher::Match;

    #[test]
    fn printer_formats_matches() {
        let printer = PrettyPrinter;
        let matches = vec![Match {
            line_no: 3,
            start: 10,
            end: 15,
        }];

        let mut output = Vec::new();
        printer
            .print(&matches, &mut output)
            .expect("printer should format matches");

        let rendered = String::from_utf8(output).expect("output should be utf-8");
        assert!(rendered.contains("3| 10, 15"));
    }
}
