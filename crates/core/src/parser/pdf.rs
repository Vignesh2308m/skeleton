use std::io::Error;
use std::ops::Range;

use pdf::content::{Op, TextDrawAdjusted};

use super::{DocumentParser, ParserMetadata, ParserMetadataDetails};

pub struct Pdf {
    path: String,
    mem_buffer: Vec<u8>,
    metadata: PdfMetadata,
    page_ranges: Vec<(Range<usize>, usize)>,
}

pub struct PdfMetadata {
    pub page: usize,
}

impl PdfMetadata {
    fn new() -> PdfMetadata {
        PdfMetadata { page: 0 }
    }
}
impl Pdf {
    fn open(path: &str) -> Result<Pdf, Error> {
        Ok(Pdf {
            path: path.to_string(),
            mem_buffer: Vec::new(),
            metadata: PdfMetadata::new(),
            page_ranges: Vec::new(),
        })
    }
}

impl DocumentParser for Pdf {
    fn new(path: &str) -> Result<Self, Error> {
        Self::open(path)
    }

    fn read(&mut self) -> Result<&[u8], Error> {
        let file = pdf::file::FileOptions::uncached()
            .open(&self.path)
            .map_err(|err| Error::new(std::io::ErrorKind::Other, err))?;

        self.metadata.page = file.num_pages() as usize;
        self.mem_buffer.clear();
        self.page_ranges.clear();
        let resolver = file.resolver();

        for (page_index, page) in file.pages().enumerate() {
            let page = page.map_err(|err| Error::other(err))?;
            let start = self.mem_buffer.len();
            if let Some(contents) = &page.contents {
                for op in contents
                    .operations(&resolver)
                    .map_err(|err| Error::other(err))?
                {
                    match op {
                        Op::TextDraw { text } => self
                            .mem_buffer
                            .extend_from_slice(text.to_string_lossy().as_bytes()),
                        Op::TextDrawAdjusted { array } => {
                            for item in array {
                                if let TextDrawAdjusted::Text(text) = item {
                                    self.mem_buffer
                                        .extend_from_slice(text.to_string_lossy().as_bytes());
                                }
                            }
                        }
                        Op::TextNewline => self.mem_buffer.push(b'\n'),
                        _ => {}
                    }
                }
            }
            let end = self.mem_buffer.len();
            if end > start {
                self.page_ranges.push((start..end, page_index + 1));
                self.mem_buffer.push(b'\n');
            }
        }
        Ok(&self.mem_buffer)
    }

    fn metadata(&self) -> Result<ParserMetadata, Error> {
        let mut metadata = ParserMetadata::from_path(&self.path, "pdf")?;
        if let ParserMetadataDetails::Pdf { page } = &mut metadata.details {
            *page = self.metadata.page;
        }
        Ok(metadata)
    }

    fn current_page(&self) -> usize {
        self.metadata.page
    }

    fn metadata_at(&self, byte_offset: usize) -> ParserMetadataDetails {
        let page = self
            .page_ranges
            .iter()
            .find(|(range, _)| range.contains(&byte_offset))
            .map(|(_, page)| *page)
            .unwrap_or(0);
        ParserMetadataDetails::Pdf { page }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::DocumentParser;
    use crate::parser::pdf::Pdf;

    #[test]
    fn test_read_pdf() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}/../../data/test.pdf", manifest_dir);

        let pdf_file = Pdf::new(&path);
        assert!(pdf_file.is_ok(), "failed to open pdf file: {}", path);

        let mut parser = pdf_file.unwrap();
        let data = parser.read().expect("read failed");
        assert!(!data.is_empty(), "parsed data should not be empty");
    }

    #[test]
    fn test_read_pdf_extracts_text() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}/../../data/test.pdf", manifest_dir);

        let pdf_file = Pdf::new(&path);
        assert!(pdf_file.is_ok(), "failed to open pdf file: {}", path);

        let mut parser = pdf_file.unwrap();
        let data = parser.read().expect("read failed");
        let output = String::from_utf8_lossy(data);

        assert!(
            !output.trim().is_empty(),
            "expected extracted PDF text, got: {output}"
        );
    }

    #[test]
    fn test_read_pdf_updates_page_metadata() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}/../../data/test.pdf", manifest_dir);

        let pdf_file = Pdf::new(&path);
        assert!(pdf_file.is_ok(), "failed to open pdf file: {}", path);

        let mut parser = pdf_file.unwrap();
        parser.read().expect("read failed");

        assert!(
            parser.metadata.page > 0,
            "expected parser metadata to track a page number after reading"
        );
    }
}
