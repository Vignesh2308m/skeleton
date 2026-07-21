use std::fs::File;
use std::io::{BufReader, Error};

use super::{DocumentParser, ParserMetadata};

const SIZE: usize = 1024;

pub struct Pdf {
    path: String,
    file_buffer: BufReader<File>,
    mem_buffer: Vec<u8>,
}

impl Pdf {
    fn open(path: &str) -> Result<Pdf, Error> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);

        Ok(Pdf {
            path: path.to_string(),
            file_buffer: buffer,
            mem_buffer: vec![0u8; SIZE],
        })
    }
}

impl DocumentParser for Pdf {
    fn new(path: &str) -> Result<Self, Error> {
        Self::open(path)
    }

    fn read(&mut self) -> Result<&[u8], Error> {
        //let path = self.path.clone();
        let file = pdf::file::FileOptions::uncached()
            .open(&self.path)
            .map_err(|err| Error::new(std::io::ErrorKind::Other, err))?;

        let mut summary = String::new();
        summary.push_str(&format!("pages: {}\n", file.num_pages()));

        if let Some(ref info) = file.trailer.info_dict {
            let title = info.title.as_ref().map(|p| p.to_string_lossy());
            if let Some(t) = title {
                summary.push_str(&format!("title: {}\n", t));
            }
        }

        self.mem_buffer = summary.into_bytes();
        Ok(&self.mem_buffer)
    }

    fn metadata(&self) -> Result<ParserMetadata, Error> {
        ParserMetadata::from_path(&self.path, "pdf")
    }
}

mod tests {
    use crate::parser::pdf::Pdf;
    use crate::parser::DocumentParser;

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
}