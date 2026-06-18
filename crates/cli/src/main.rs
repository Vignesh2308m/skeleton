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
    let mut place_holder = [0; 1024];
    let mut matches: Vec<Match> =Vec::new(); 
    let mut line_no: usize = 1;
    let mut offset: usize = 0;

    let mut mem_buffer = MemBuffer {
    place_holder: [0; 1024],
    overlap: Vec::new(),
};

    loop {
    let n = buf.read(&mut mem_buffer.place_holder)?;

    if n == 0 {
        break;
    }

    let chunk = &mem_buffer.place_holder[..n];

    let mut search_buf =
        Vec::with_capacity(mem_buffer.overlap.len() + chunk.len());

    search_buf.extend_from_slice(&mem_buffer.overlap);
    search_buf.extend_from_slice(chunk);

    for (idx, win) in search_buf.windows(pattern.len()).enumerate() {
        if win == pattern {
            matches.push(Match {
                line_no,
                start: offset + idx.saturating_sub(mem_buffer.overlap.len()),
                end: offset
                    + idx.saturating_sub(mem_buffer.overlap.len())
                    + pattern.len(),
            });
        }
    }

    for &b in chunk {
        if b == b'\n' {
            line_no += 1;
        }
    }

    let overlap_len = pattern.len().saturating_sub(1).min(search_buf.len());

    mem_buffer.overlap.clear();
    mem_buffer
        .overlap
        .extend_from_slice(&search_buf[search_buf.len() - overlap_len..]);

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