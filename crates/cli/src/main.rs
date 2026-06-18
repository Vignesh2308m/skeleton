use std::env;
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
    place_holder: [u8; 1024],
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
    let mut matches: Vec<Match> =Vec::new(); 
    let mut line_no: usize = 1;
    let mut offset: usize = 0;
    let window_size = pattern.len();

    let mut mem_buffer = MemBuffer{
        place_holder : [0; 1024],
        overlap : Vec::new()
    };
    
    loop {
        let n = buf.read(&mut mem_buffer.place_holder)?;

        let window = 0;

        if n==0{
            break;
        }

        let chunk = &mem_buffer.place_holder[..n];

        let mut w =0;
        while mem_buffer.overlap.len() > 0 {
            if (chunk[..w]==pattern[..w]) && (mem_buffer.overlap[w..]== pattern[w..]){
                matches.push(
                    Match { line_no:line_no, start: offset+w, end: offset+w+window_size }
                );
            }
            w+=1;
        }

        for i in chunk.windows(window_size){
            if i == pattern{
                matches.push(
                    Match { line_no:line_no, start: offset+w, end: offset+w+window_size }
                );
            }
            w+=1;
        }
        offset += n;
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

    let mut buffer = BufReader::new(file);

    let matches = find_match(buffer, arg.pattern.as_slice())?; 
    
    print_pretty(matches)?;
    
    Ok(())
}