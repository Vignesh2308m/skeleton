use std::collections::HashMap;
use std::io::Error;
use std::{env};
use std::fs::File;
use std::io::ErrorKind::InvalidInput;
use std::io::{BufReader};

const SIZE:usize = 1024;

struct Txt{
    buffer: BufReader<File>,
    place_holder: [u8; SIZE],
    overflow: Vec<u8>,
    line_offset: HashMap<u8,u8>
}

impl Txt{
    fn new(path:&str)-> Result<Txt, Error>{

        let file = File::open(path)?;

        let buffer = BufReader::new(file);

        Ok(Txt { buffer, place_holder:[0;SIZE], overflow:Vec::new(), line_offset:HashMap::new()})
    }
    fn read(){
        todo!();
    }
    
    fn close(){
        todo!();
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::txt::Txt;

    #[test]
    fn test_sample(){
        let txt_file = Txt::new("C:/Users/Vickynila/Projects/skeleton/data/intro.txt");
        if let Err(err) = &txt_file{
            println!("{}",err) 
        }
        assert!(txt_file.is_ok());
    }
}