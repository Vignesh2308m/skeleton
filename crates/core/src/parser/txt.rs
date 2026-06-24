use std::io::{Error, Read};
use std::fs::File;
use std::io::{BufReader};

const SIZE:usize = 1024;

struct Text{
    buffer: BufReader<File>,
    place_holder: [u8; SIZE],
    overflow: Vec<u8>,
    offset: usize,
}


impl Text{
    fn new(path:&str)-> Result<Text, Error>{

        let file = File::open(path)?;

        let buffer = BufReader::new(file);

        Ok(Text { buffer, place_holder:[0;SIZE], overflow:Vec::new(), offset:0})
    }
    fn read_line(&mut self) -> Result<&[u8],Error>{
        let n = self.buffer.read(&mut self.place_holder); 
        
        let i:usize = 0;
        for i in 0..SIZE{
            if self.place_holder[i] == b'\n'{
                break ;
            }
        }
        if i == 0{
            return Err(std::io::Error::from(
                           std::io::ErrorKind::UnexpectedEof,
                       ));
        }

        self.overflow.extend_from_slice(&self.place_holder);
        
        let mut start = 0;
        let mut end = 0;

        if self.offset == 0{
            self.offset += i;
            start = 0;
            end = i;
        } else {
            start = self.offset;
            end = self.offset + i;
            self.offset += i;
        }
        Ok(&self.overflow[start..end])

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