use std::io::Error;
use std::ops::Range;

use calamine::{open_workbook_auto, DataType, Reader};

use super::{DocumentParser, ParserMetadata, ParserMetadataDetails};

pub struct Xlsx {
    path: String,
    mem_buffer: Vec<u8>,
    metadata: XlsxMetadata,
    cell_ranges: Vec<(Range<usize>, XlsxMetadata)>,
}

#[derive(Clone)]
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
        Ok(Xlsx {
            path: path.to_string(),
            mem_buffer: Vec::new(),
            metadata: XlsxMetadata::new(),
            cell_ranges: Vec::new(),
        })
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

    fn metadata_at(&self, byte_offset: usize) -> ParserMetadataDetails {
        self.cell_ranges
            .iter()
            .find(|(range, _)| range.contains(&byte_offset))
            .map(|(_, location)| ParserMetadataDetails::Xlsx {
                sheet: location.sheet.clone(),
                row: location.row,
                column: location.column,
            })
            .unwrap_or(ParserMetadataDetails::Xlsx {
                sheet: String::new(),
                row: 0,
                column: 0,
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
        let mut text = Vec::new();

        self.metadata = XlsxMetadata::new();
        self.cell_ranges.clear();

        for sheet_name in sheet_names.iter() {
            if let Ok(range) = workbook.worksheet_range(sheet_name) {
                self.metadata.sheet = sheet_name.to_string();

                for (row_index, row) in range.rows().enumerate() {
                    for (column_index, cell) in row.iter().enumerate() {
                        if cell.is_empty() {
                            continue;
                        }
                        let location = XlsxMetadata {
                            sheet: sheet_name.to_string(),
                            row: row_index as u32 + 1,
                            column: column_index as u32 + 1,
                        };
                        self.metadata = location.clone();
                        if !text.is_empty() {
                            text.push(b'\n');
                        }
                        let start = text.len();
                        text.extend_from_slice(cell.to_string().as_bytes());
                        let end = text.len();
                        self.cell_ranges.push((start..end, location));
                    }
                }
            }
        }

        self.mem_buffer = text;

        Ok(&self.mem_buffer)
    }

    fn metadata(&self) -> Result<ParserMetadata, Error> {
        let mut metadata = ParserMetadata::from_path(&self.path, "xlsx")?;
        if let ParserMetadataDetails::Xlsx { sheet, row, column } = &mut metadata.details {
            *sheet = self.metadata.sheet.clone();
            *row = self.metadata.row;
            *column = self.metadata.column;
        }
        Ok(metadata)
    } 
}

#[cfg(test)]
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
    fn test_read_excel_reads_cell_values() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}/../../data/excel_test.xlsx", manifest_dir);

        let xlsx_file = Xlsx::new(&path);
        assert!(xlsx_file.is_ok(), "failed to open xlsx file: {}", path);

        let mut parser = xlsx_file.unwrap();
        let data = parser.read().expect("read failed");
        let output = String::from_utf8_lossy(data);

        assert!(
            !output.trim().is_empty(),
            "expected cell values in parser output, got: {output}"
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
        match &metadata.details {
            super::ParserMetadataDetails::Xlsx { sheet, row, column } => {
                assert!(
                    !sheet.is_empty(),
                    "expected parser metadata to track a sheet name"
                );
                assert!(*row > 0, "expected row metadata to be set");
                assert!(*column > 0, "expected column metadata to be set");
            }
            other => panic!("expected xlsx metadata but got: {other:?}"),
        }
    }
}
