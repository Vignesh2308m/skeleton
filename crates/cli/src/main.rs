use std::io::{Error, Read};
use std::fs::{self, File};
use std::io::BufReader;

use crate::FileType::Txt;

enum FileType {
    Txt
}

struct Reader{
    buf : BufReader<File>,
    file_type : FileType
}

impl Reader {
    fn new(path:&str, file_type:FileType) -> Result<Reader, Error> {
        let file = fs::File::open(path)?;
        let buf = BufReader::new(file);

        Ok(Reader{
            buf:buf,
            file_type:file_type
        })
    }

    fn read(&mut self) -> Result<[u8;10], Error> {
        let mut content = [0;10];
        self.buf.read(&mut content)?;
        Ok(content)
    }
}

struct Search<T: core::cmp::PartialEq>{
    word_matches : Vec<T>
}

impl<T: core::cmp::PartialEq> Search<T> {
    fn compare(&mut self, a: T, b: T){
        if a == b {
            self.word_matches.push(a);
        }
    }
}

fn main() {
    let r = Reader::new("./data/intro.txt", Txt);
    if let Err(err) = r {
        println!("{}", err);
        return;
    }

    let c = r.ok().unwrap().read();

    if let Err(err) = c {
        println!("{}", err);
        return;
    }

    println!("{:?}", c.ok().unwrap());

    
}