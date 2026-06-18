use std::{env, mem};
use std::fs::File;
use std::io::ErrorKind::InvalidInput;
use std::io::{BufReader, Read};

struct Args{
    path: String,
    pattern: Vec<u8>
}
struct Match{
    line_no: usize,
    start: usize,
    end: usize,
}

struct MemBuffer {
    place_holder: [u8; 5],
    overlap: Vec<u8>,
}

fn get_args() -> Result<Args, std::io::Error> {
    let mut args = env::args();

    args.next(); // executable name

    let path = args
        .next()
        .ok_or_else(|| std::io::Error::new(InvalidInput, "Missing path"))?;

    let pattern = args
        .next()
        .ok_or_else(|| std::io::Error::new(InvalidInput, "Missing pattern"))?
        .into_bytes();

    Ok(Args { path, pattern })
}

fn find_match(mut buf: BufReader<File>, pattern: &[u8])-> Result<Vec<Match>,std::io::Error>{
    let mut offset = 0;
    let mut line_no = 0;
    let size = pattern.len();
    let mut matches: Vec<Match> = Vec::new();
    let mut mem_buf = MemBuffer{
        place_holder:[0;5],
        overlap:Vec::new()
    };

    loop{
        let n = buf.read(&mut mem_buf.place_holder)?;

        if n == 0{
            break;
        }
        println!("{}", n);

        let chunk = &mem_buf.place_holder[..n];
        println!("{:?}", mem_buf.overlap);
        println!("{:?}", chunk);

        let mut temp:Vec<u8> = Vec::new();

        for i in 0..mem_buf.overlap.len(){
            temp.clear();
            temp.extend_from_slice(&mem_buf.overlap[i..]);
            temp.extend_from_slice(&chunk[..i+1]);

            if &temp == pattern{
                print!("YES");
            }
        }

        for i in chunk.windows(size){
            //println!("{:?}",i);
            if i == pattern{
                print!("YES");
            }
        }
        if n>size{
            mem_buf.overlap.clear();
            mem_buf.overlap.extend_from_slice(&chunk[n-size+1..]);
        }
        //println!("{:?}", mem_buf.overlap);
    }
    Ok(matches)
}

fn print_pretty(matches: Vec<Match>) -> Result<(), std::io::Error>{
    for m in matches{
        println!("{}| {}, {}", m.line_no, m.start, m.end);
    }
    Ok(())
}


fn main() -> Result<(), std::io::Error> {

    let arg = get_args()?;

    // Loading File Buffer
    let file = File::open(arg.path)?;

    let buffer = BufReader::new(file);

    let matches = find_match(buffer, arg.pattern.as_slice())?; 
    
    print_pretty(matches)?;
    
    Ok(())
}