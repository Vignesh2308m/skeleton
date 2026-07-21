use std::fs::File;
use std::io::{BufReader, Error};

use calamine::{Reader, open_workbook_auto};

use super::{DocumentParser, ParserMetadata};

const SIZE: usize = 32;

pub struct Xlsx {
    path: String,
    file_buffer: BufReader<File>,
    mem_buffer: Vec<u8>,
}

impl Xlsx {
    fn open(path: &str) -> Result<Xlsx, Error> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);

        Ok(Xlsx {
            path: path.to_string(),
            file_buffer: buffer,
            mem_buffer: vec![0u8; SIZE],
        })
    }
}

impl DocumentParser for Xlsx {
    fn new(path: &str) -> Result<Self, Error> {
        Self::open(path)
    }

    fn read(&mut self) -> Result<&[u8], Error> {
        let mut workbook = open_workbook_auto(&self.path)
            .map_err(|err| Error::new(std::io::ErrorKind::Other, err))?;

        let sheet_names = workbook.sheet_names();
        let summary = sheet_names.join("\n");
        self.mem_buffer = summary.into_bytes();

        Ok(&self.mem_buffer)
    }

    fn metadata(&self) -> Result<ParserMetadata, Error> {
        ParserMetadata::from_path(&self.path, "xlsx")
    }
}

mod tests {
    use crate::parser::xlsx::Xlsx;
    use crate::parser::DocumentParser;

    #[test]
    fn test_read_excel() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}/../../data/excel_test.xlsx", manifest_dir);

        let xlsx_file = Xlsx::new(&path);
        assert!(xlsx_file.is_ok(), "failed to open xlsx file: {}", path);

        let mut parser = xlsx_file.unwrap();
        let data = parser.read().expect("read failed");
        assert!(!data.is_empty(), "parsed data should not be empty");
    }
}