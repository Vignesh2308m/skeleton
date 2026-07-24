use std::fs::File;
use std::io::{BufReader, Error};

use calamine::{Reader, open_workbook_auto};

use super::{DocumentParser, ParserMetadata};

const SIZE: usize = 32;

pub struct Xlsx {
    path: String,
    file_buffer: BufReader<File>,
    mem_buffer: Vec<u8>,
    metadata: XlsxMetadata,
}

pub struct XlsxMetadata {
    pub sheet: String,
    pub row: u32,
    pub column: u32,
}

impl XlsxMetadata {
    fn new() -> Self {
        Self {
            sheet: String::new(),
            row: 0,
            column: 0,
        }
    }
}

impl Xlsx {
    fn open(path: &str) -> Result<Xlsx, Error> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);

        Ok(Xlsx {
            path: path.to_string(),
            file_buffer: buffer,
            mem_buffer: vec![0u8; SIZE],
            metadata: XlsxMetadata::new(),
        })
    }

    fn cell_reference(row: usize, column: usize) -> String {
        let mut column_name = String::new();
        let mut column_index = column as u32 + 1;

        while column_index > 0 {
            column_index -= 1;
            let remainder = (column_index % 26) as u8;
            column_name.push((b'A' + remainder) as char);
            column_index /= 26;
        }

        column_name.chars().rev().collect::<String>() + &(row + 1).to_string()
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
        let mut cell_addresses = Vec::new();

        self.metadata = XlsxMetadata::new();

        for sheet_name in sheet_names.iter() {
            if let Ok(range) = workbook.worksheet_range(sheet_name) {
                self.metadata.sheet = sheet_name.to_string();

                for (row_index, row) in range.rows().enumerate() {
                    for (column_index, _) in row.iter().enumerate() {
                        self.metadata.row = row_index as u32;
                        self.metadata.column = column_index as u32;

                        if !cell_addresses.is_empty() {
                            cell_addresses.push(b'\n');
                        }

                        let cell_ref = Self::cell_reference(row_index, column_index);
                        cell_addresses.extend_from_slice(cell_ref.as_bytes());
                    }
                }
            }
        }

        self.mem_buffer = cell_addresses;

        Ok(&self.mem_buffer)
    }

    fn metadata(&self) -> Result<ParserMetadata, Error> {
        let mut metadata = ParserMetadata::from_path(&self.path, "xlsx")?;
        metadata.sheet = self.metadata.sheet.clone();
        metadata.row = self.metadata.row;
        metadata.column = self.metadata.column;
        Ok(metadata)
    }

    fn current_sheet(&self) -> String {
        self.metadata.sheet.clone()
    }

    fn current_row(&self) -> u32 {
        self.metadata.row
    }

    fn current_column(&self) -> u32 {
        self.metadata.column
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

    #[test]
    fn test_read_excel_reads_cell_addresses() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}/../../data/excel_test.xlsx", manifest_dir);

        let xlsx_file = Xlsx::new(&path);
        assert!(xlsx_file.is_ok(), "failed to open xlsx file: {}", path);

        let mut parser = xlsx_file.unwrap();
        let data = parser.read().expect("read failed");
        let output = String::from_utf8_lossy(data);

        assert!(
            output.contains("A1") || output.contains("B1") || output.contains("C1"),
            "expected cell address bytes in parser output, got: {output}"
        );
    }

    #[test]
    fn test_read_excel_updates_cell_metadata() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}/../../data/excel_test.xlsx", manifest_dir);

        let xlsx_file = Xlsx::new(&path);
        assert!(xlsx_file.is_ok(), "failed to open xlsx file: {}", path);

        let mut parser = xlsx_file.unwrap();
        parser.read().expect("read failed");

        let metadata = parser.metadata().expect("metadata failed");
        assert!(!metadata.sheet.is_empty(), "expected parser metadata to track a sheet name");
        assert!(metadata.row >= 0, "expected row metadata to be set");
        assert!(metadata.column >= 0, "expected column metadata to be set");
    }
}