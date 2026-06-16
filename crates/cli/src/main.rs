use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() -> Result<(), std::io::Error> {
    // Getting input arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("number of arguments mismatches");
    }

    let path = &args[1];
    let pattern = args[2].as_bytes();

    // Loading File Buffer
    let file = File::open(path)?;

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

        for i in chunk.windows(pattern.len()) {
            if i == pattern {
                println!(
                    "line {}, start {}, end {}",
                    line_no,
                    offset + window,
                    offset + window + pattern.len()
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