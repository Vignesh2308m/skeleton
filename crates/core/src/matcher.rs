use std::fs::File;
use std::io::{BufReader, Read};



pub struct Match{
    pub line_no: usize,
    pub start: usize,
    pub end: usize,
}

pub struct MemBuffer {
    place_holder: [u8; 5],
    overlap: Vec<u8>,
}

pub fn find_match(mut buf: BufReader<File>, pattern: &[u8])-> Result<Vec<Match>,std::io::Error>{
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

        let chunk = &mem_buf.place_holder[..n];

        let mut w = 0;

        for split in 1..=mem_buf.overlap.len() {
            if mem_buf.overlap.ends_with(&pattern[..split])
                && chunk.starts_with(&pattern[split..])
            {
                matches.push(
                    Match { line_no: line_no, start: offset-split+1, end: offset-split+size}
                );
            }
            
            if mem_buf.overlap[w] == b'\n'{
                line_no += 1;
            }

            w += 1;
        }

        for (idx, i) in chunk.windows(size).enumerate(){
            if i == pattern{
                matches.push(
                    Match { line_no: line_no, start: offset+idx, end: offset+idx+size-1 }
                );
            }
            
            if chunk[w-mem_buf.overlap.len()] == b'\n'{
                line_no += 1;
            }

            w += 1;
        }
        if n>size{
            mem_buf.overlap.clear();
            mem_buf.overlap.extend_from_slice(&chunk[n-size+1..]);
        }

        offset+=n;
    }
    Ok(matches)
}