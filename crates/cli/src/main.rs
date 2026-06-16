use std::env;
use std::fmt::Error;
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

fn find_match(buf: BufReader<File>, pattern: &[u8])-> Result<Vec<Match>,Error>{
    todo!()
}

fn print_pretty(matches: Vec<Match>) -> Result<(), Error>{
    todo!()
}


fn main() -> Result<(), std::io::Error> {

    let arg = get_args()?;

    // Loading File Buffer
    let file = File::open(arg.path)?;

    let mut buffer = BufReader::new(file);

    let mut place_holder = [0; 1024];

    let mut line_no: usize = 1;
    let mut offset: usize = 0;

    loop {
        let n = buffer.read(&mut place_holder)?;

        if n == 0 {
            break;
        }

        let chunk = &place_holder[..n];

        let mut window = 0;

        for i in chunk.windows(arg.pattern.len()) {
            if i == arg.pattern {
                println!(
                    "line {}, start {}, end {}",
                    line_no,
                    offset + window,
                    offset + window + arg.pattern.len()
                );
            }

            // Move to next line if current byte is '\n'
            if chunk[window] == b'\n' {
                line_no += 1;
            }

            window += 1;
        }

        // Handle any remaining bytes after the last window
        for &b in &chunk[window..] {
            if b == b'\n' {
                line_no += 1;
            }
        }

        offset += n;
    }

    Ok(())
}