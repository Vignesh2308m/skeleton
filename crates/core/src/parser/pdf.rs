use std::io::{Error, Read, Seek};
use std::fs::File;
use std::io::{BufReader};
use std::convert::TryInto;
use std::ops::RemAssign;

const SIZE:usize = 1024;

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
        // Go to Eof 
        let file_size = self.file_buffer.seek(std::io::SeekFrom::End(0))?;
        let read_size = file_size.min(SIZE as u64) as usize;
        self.file_buffer.seek(std::io::SeekFrom::End(-(read_size as i64)))?;
        let n = self.file_buffer.read(&mut self.mem_buffer)?; 

        
        //Finding startxref
        let sxref = self.bounded_seg_search( b"startxref\r\n", 0, n )?;
        let eof = self.bounded_seg_search( b"\r\n%%EOF", 0 , n )?;
        
        let mut xref_pos:u64 = 0;
        for i  in &self.mem_buffer[sxref+b"startxref\r\n".len()..eof]{
            xref_pos = xref_pos*10 + (*i - b'0') as u64;
        }

        //Finding Xref table
        self.file_buffer.seek(std::io::SeekFrom::Start(xref_pos))?;
        let n = self.file_buffer.read(&mut self.mem_buffer)?; 
        let x = self.bounded_seg_search(b"xref",0,n)?;
        let y = self.bounded_seg_search(b"trailer",x,n)?;

        println!("{}", String::from_utf8_lossy(&self.mem_buffer[x..y]));
        Ok(&self.mem_buffer)
    }

    fn bounded_seg_search(&self,key:&[u8], start:usize, end:usize)->Result<usize, std::io::Error>{
        let x = self.mem_buffer[start..end].windows(key.len()).position(|x| x == key).expect("Unable to find start");
        Ok(x)
    }
    
    fn nested_seg_search(&self, key:&[u8], start:usize, end:usize)->Result<Vec<usize>, std::io::Error>{
        let x:Vec<usize> = self.mem_buffer[start..end].windows(key.len()).enumerate().filter(|(_, x)| *x == key).map(|(i,_)| i).collect();
        Ok(x)
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