use std::fs::File;
use std::io::{BufReader, Error, Read};

use super::{DocumentParser, ParserMetadata};

pub struct Text {
    path: String,
    file_buffer: BufReader<File>,
    mem_buffer: Vec<u8>,
}

impl Text {
    fn open(path: &str) -> Result<Text, Error> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);

        Ok(Text {
            path: path.to_string(),
            file_buffer: buffer,
            mem_buffer: Vec::new(),
        })
    }
}

impl DocumentParser for Text {
    fn new(path: &str) -> Result<Self, Error> {
        Self::open(path)
    }

    fn read(&mut self) -> Result<&[u8], Error> {
        self.mem_buffer.clear();
        self.file_buffer.read_to_end(&mut self.mem_buffer)?;
        Ok(&self.mem_buffer)
    }

    fn metadata(&self) -> Result<ParserMetadata, Error> {
        ParserMetadata::from_path(&self.path, "text")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::DocumentParser;
    use crate::parser::txt::Text;

    #[test]
    fn test_sample() {
        let txt_file = Text::new("C:/Users/Vickynila/Projects/skeleton/data/intro.txt");
        if let Err(err) = &txt_file {
            println!("{}", err)
        }
        assert!(txt_file.is_ok());
    }

    #[test]
    fn test_read_text() {
        let txt_file = Text::new("C:/Users/Vickynila/Projects/skeleton/data/intro.txt");
        if let Err(err) = &txt_file {
            println!("{}", err);
            panic!("Error");
        }
        let mut txt_file = txt_file.unwrap();
        let value = txt_file.read().expect("read failed");
        assert!(!value.is_empty());
    }
}
