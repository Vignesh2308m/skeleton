use std::fs::File;
use std::io::{BufReader, Error, Read};

use super::{Parser, ParserMetadata};

const SIZE: usize = 4096;

pub struct Text {
    path: String,
    file_buffer: BufReader<File>,
    mem_buffer: Vec<u8>,
    line_offset: usize,
    byte_offset: usize,
}

impl Text {
    fn open(path: &str) -> Result<Text, Error> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);

        Ok(Text {
            path: path.to_string(),
            file_buffer: buffer,
            mem_buffer: vec![0u8; SIZE],
            line_offset: 0,
            byte_offset: 0,
        })
    }

    fn read_line(&mut self) -> Result<&[u8], Error> {
        let mut start = 0;
        let mut end = 0;
        let mut found = false;

        loop {
            if self.line_offset == self.mem_buffer.len() {
                self.mem_buffer.resize(self.mem_buffer.len() + SIZE, 0u8);
            }

            let n = self.file_buffer.read(&mut self.mem_buffer[self.line_offset..])?;
            self.line_offset += n;

            if self.line_offset <= self.byte_offset {
                return Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof));
            }

            for i in self.byte_offset..self.line_offset {
                if self.mem_buffer[i] == b'\n' {
                    start = self.byte_offset;
                    end = i;
                    self.byte_offset = i + 1;
                    found = true;
                    break;
                }
            }

            if found {
                break;
            }
        }
        Ok(&self.mem_buffer[start..end])
    }
}

impl Parser for Text {
    fn new(path: &str) -> Result<Self, Error> {
        Self::open(path)
    }

    fn read(&mut self) -> Result<&[u8], Error> {
        self.read_line()
    }

    fn metadata(&self) -> Result<ParserMetadata, Error> {
        ParserMetadata::from_path(&self.path, "text")
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::txt::Text;
    use crate::parser::Parser;

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
        while let Ok(value) = txt_file.read_line() {
            println!("{:?}", &String::from_utf8(Vec::from(value)));
        }
    }
}