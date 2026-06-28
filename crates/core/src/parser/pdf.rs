use std::io::{Error, Read};
use std::fs::File;
use std::io::{BufReader};

const SIZE:usize = 4096;

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

    fn read(){
        todo!();
    }
    
}