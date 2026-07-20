use std::collections::HashMap;
use std::io::{Error, Read, Seek};
use std::fs::File;
use std::io::{BufReader};
use flate2::read::ZlibDecoder;
use std::convert::TryInto;
use std::ops::RemAssign;


const SIZE:usize = 32;

struct Xlsx{
    file_buffer: BufReader<File>,
    mem_buffer: Vec<u8>
}

impl Xlsx {
    fn new(path:&str)-> Result<Xlsx, Error>{

        let file = File::open(path)?;

        let buffer = BufReader::new(file);

        Ok(Xlsx { file_buffer: buffer, mem_buffer:vec![0u8;SIZE]} )

    }
    
    fn read(&mut self)->Result<&[u8],std::io::Error>{
        
        // Go to Eof 
        let file_size = self.file_buffer.seek(std::io::SeekFrom::End(0))?;
        let read_size = file_size.min(SIZE as u64) as usize;
        self.file_buffer.seek(std::io::SeekFrom::End(-(read_size as i64)))?;
        let n = self.file_buffer.read(&mut self.mem_buffer)?; 


        Ok(&self.mem_buffer)    
    }
}

mod tests{
    use crate::parser::xlsx::{self, Xlsx};

    #[test]
    fn test() {
        let xlsx_file = Xlsx::new("C:/Users/Vickynila/Projects/skeleton/data/excel_test.xlsx"); 

        if let Err(err) = xlsx_file {
            println!("{}", err);
            panic!("Error");
        }
        
        xlsx_file.unwrap().read().ok();

    }

}