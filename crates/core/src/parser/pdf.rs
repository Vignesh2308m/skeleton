use std::io::{Error, Read, Seek};
use std::fs::File;
use std::io::{BufReader};

const SIZE:usize = 32;

enum Layout{
    HEADER,
    XREFTABLE,
    STARTXREF,
    TRAILER,
    EOF,
}

struct Pdf{ 
    file_buffer: BufReader<File>,
    mem_buffer: Vec<u8>,
    line_offset: usize,
    byte_offset: usize, 
}

impl Pdf{
    fn new(path:&str)-> Result<Pdf, Error>{

        let file = File::open(path)?;

        let buffer = BufReader::new(file);

        Ok(Pdf{ file_buffer: buffer, mem_buffer:vec![0u8;SIZE], line_offset:0, byte_offset:0})
    }

    fn read(&mut self)-> Result<&[u8], std::io::Error>{
        // Finding Eof 
        let file_size = self.file_buffer.seek(std::io::SeekFrom::End(0))?;

        let read_size = file_size.min(SIZE as u64) as usize;

        self.file_buffer.seek(std::io::SeekFrom::End(-(read_size as i64)))?;

        let n = self.file_buffer.read(&mut self.mem_buffer)?; 

        //Finding startxref
        let sx = b"startxref\r\n";
        let eof = b"\r\n%%EOF\r\n";
        let mut startxref_byte = 0usize;
        let mut eof_byte = 0usize;

        for (i, x) in self.mem_buffer[..n].windows(sx.len()).enumerate(){
            if x == sx{
                startxref_byte = i + sx.len(); 
            }
        }

        for (i, x) in self.mem_buffer[startxref_byte..n].windows(eof.len()).enumerate(){
            if x == eof{
                eof_byte = i + startxref_byte; 
            }
        }

        println!("Found {},{}:", startxref_byte, eof_byte);

        println!("{:?}", String::from_utf8(Vec::from(&self.mem_buffer[..n])));

        Ok(&self.mem_buffer)
    }
    
}

mod tests{
    use crate::parser::pdf::{self, Pdf};

    #[test]
    fn test() {
        let pdf_file = Pdf::new("C:/Users/Vickynila/Projects/skeleton/data/test.pdf"); 

        if let Err(err) = pdf_file {
            println!("{}", err);
            panic!("Error");
        }
        
        pdf_file.unwrap().read().ok();

    }

}