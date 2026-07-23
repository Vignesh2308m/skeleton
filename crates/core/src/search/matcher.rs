use std::fs::File;
use std::io::{BufReader, Read};

pub struct Match {
    pub line_no: usize,
    pub start: usize,
    pub end: usize,
}

/// SIMD-accelerated substring search using the `memchr` crate's `memmem::Finder`.
pub fn find_match(mut buf: BufReader<File>, pattern: &[u8]) -> Result<Vec<Match>, std::io::Error> {
    let mut data = Vec::new();
    buf.read_to_end(&mut data)?;

    if pattern.is_empty() {
        return Ok(Vec::new());
    }

    let finder = memchr::memmem::Finder::new(pattern);
    let mut matches: Vec<Match> = Vec::new();

    // Precompute newline indices for fast line number lookup.
    let mut newline_positions: Vec<usize> = Vec::new();
    for (i, &b) in data.iter().enumerate() {
        if b == b'\n' {
            newline_positions.push(i);
        }
    }

    for m in finder.find_iter(&data) {
        let start = m;
        let end = m + pattern.len() - 1;

        // Count number of newlines strictly before `start` to derive 0-based line number.
        let newline_count = match newline_positions.binary_search(&start) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        matches.push(Match { line_no: newline_count, start, end });
    }

    Ok(matches)
}
