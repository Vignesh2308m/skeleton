use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub struct SearchMatch {
    pub file: PathBuf,
    pub start: u64,
    pub end: u64,
    pub metadata: MatchMetadata,
}

pub enum MatchMetadata {
    Text { line: usize, column: usize },
    Pdf { page: usize },
    Xlsx { sheet: String, row: u32, column: u32 },
}

/// SIMD-accelerated substring search over the provided buffer. Returns file-aware `SearchMatch`s.
pub fn find_match(mut buf: BufReader<File>, path: PathBuf, pattern: &[u8]) -> Result<Vec<SearchMatch>, std::io::Error> {
    let mut data = Vec::new();
    buf.read_to_end(&mut data)?;

    if pattern.is_empty() {
        return Ok(Vec::new());
    }

    let finder = memchr::memmem::Finder::new(pattern);
    let mut matches: Vec<SearchMatch> = Vec::new();

    // Precompute newline indices for fast line number lookup.
    let mut newline_positions: Vec<usize> = Vec::new();
    for (i, &b) in data.iter().enumerate() {
        if b == b'\n' {
            newline_positions.push(i);
        }
    }

    for m in finder.find_iter(&data) {
        let start = m as u64;
        let end = (m + pattern.len() - 1) as u64;

        let newline_count = match newline_positions.binary_search(&m) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        let column = if newline_count == 0 {
            m
        } else {
            let prev = newline_positions[newline_count - 1];
            m - prev - 1
        };

        let metadata = MatchMetadata::Text {
            line: newline_count,
            column,
        };

        matches.push(SearchMatch { file: path.clone(), start, end, metadata });
    }

    Ok(matches)
}
