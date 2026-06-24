use std::io::{Error, Read};
use std::fs::File;
use std::io::{BufReader};

const SIZE:usize = 1024;

struct Text{
    buffer: BufReader<File>,
    place_holder: Vec<u8>,
    offset: usize,
}


impl Text{
    fn new(path:&str)-> Result<Text, Error>{

        let file = File::open(path)?;

        let buffer = BufReader::new(file);

        Ok(Text { buffer, place_holder:Vec::new(), offset:0})
    }
    fn read_line(&mut self) -> Result<&[u8],Error>{

        if self.offset > 4*SIZE{
            self.place_holder.drain(..self.offset);
            self.offset = 0;
        }

        let n = self.buffer.read(&mut self.place_holder[self.offset..self.offset+SIZE])?; 
        
        let mut start = 0;
        let mut end = 0;

        for i in 0..n{
            if self.place_holder[self.offset + i] == b'\n'{
                start = self.offset;
                end = self.offset + i;
                self.offset += i;
                break ;
            }
        }

        if n == 0{
            return Err(std::io::Error::from(
                           std::io::ErrorKind::UnexpectedEof,
                       ));
        }
          
        Ok(&self.place_holder[start..end])

    }
    
    fn close(){
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::txt::Text;

    #[test]
    fn test_sample(){
        let txt_file = Text::new("C:/Users/Vickynila/Projects/skeleton/data/intro.txt");
        if let Err(err) = &txt_file{
            println!("{}",err) 
        }
        assert!(txt_file.is_ok());
    }
}