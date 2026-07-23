use std::env;
use std::fs;
use std::io::{Error, ErrorKind::InvalidInput, Write};

pub mod parser;
pub mod printer;
pub mod search;

use crate::parser::{DocumentParser, Pdf, Text, Xlsx};
use crate::printer::{PrettyPrinter, Printer};
use crate::search::Search;

struct Args {
    pattern: Vec<u8>,
    search_dir: String,
}

fn get_args() -> Result<Args, Error> {
    let mut args = env::args();

    args.next();

    let pattern = args
        .next()
        .ok_or_else(|| Error::new(InvalidInput, "Missing pattern"))?
        .into_bytes();

    let search_dir = args
        .next()
        .unwrap_or_else(|| format!("{}/../../data", env!("CARGO_MANIFEST_DIR")));

    Ok(Args { pattern, search_dir })
}

fn search_directory(
    search_dir: &str,
    pattern: &[u8],
    printer: &impl Printer,
    writer: &mut dyn Write,
) -> Result<bool, Error> {
    let mut found_matches = false;
    let mut entries = fs::read_dir(search_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    entries.sort();

    for path in entries {
        let path_str = path.to_string_lossy().into_owned();
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_lowercase();

        let matches = match extension.as_str() {
            "txt" => {
                let mut parser = Text::new(&path_str)?;
                parser.search(pattern)?
            }
            "pdf" => {
                let mut parser = Pdf::new(&path_str)?;
                parser.search(pattern)?
            }
            "xlsx" => {
                let mut parser = Xlsx::new(&path_str)?;
                parser.search(pattern)?
            }
            _ => continue,
        };

        if matches.is_empty() {
            continue;
        }

        found_matches = true;
        writeln!(writer, "{}", path_str)?;
        printer.print(&matches, writer)?;
    }

    Ok(found_matches)
}

fn main() -> Result<(), Error> {
    let arg = get_args()?;
    let printer = PrettyPrinter;
    let mut stdout = std::io::stdout();

    let found_matches = search_directory(&arg.search_dir, &arg.pattern, &printer, &mut stdout)?;

    if !found_matches {
        writeln!(stdout, "no")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{PrettyPrinter, search_directory};

    #[test]
    fn search_directory_finds_matches_in_data_files() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let search_dir = format!("{}/../../data", manifest_dir);
        let printer = PrettyPrinter;
        let mut output = Vec::new();

        let found = search_directory(&search_dir, b"abcde", &printer, &mut output)
            .expect("searching the data directory should work");

        assert!(found, "expected at least one match in the data directory");
        let rendered = String::from_utf8(output).expect("output should be utf-8");
        assert!(rendered.contains("intro.txt"));
    }
}