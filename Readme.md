# Skeleton - A Grep Project

<p align="center">
  <img src="./imgs/logo.png" alt="Image" width="400" height="400" />
</p>
Skeleton is a sophisticated implementation of `grep` designed to explore advanced Rust programming concepts and demonstrate high-performance search capabilities.

## Features

- **Basic grep** - Core pattern matching and text search functionality
- **Recursive directory search** - Search through entire directory trees
- **Regex support** - Full regular expression pattern matching
- **Multithreaded search** - Leverage multiple CPU cores for faster searching
- **Memory-mapped files** - Efficient file I/O using memory mapping
- **Producer-consumer pipeline** - Elegant task distribution and processing
- **Custom thread pool** - Fine-grained control over threading behavior
- **Aho-Corasick integration** - Multiple pattern matching optimization
- **Custom regex engine** - Tailored regex implementation for specific use cases
- **Distributed grep** - Search across multiple machines in a network

## Project Structure

```text
.
├── crates/
│   └── core/
│       ├── src/
│       │   ├── main.rs
│       │   ├── matcher.rs
│       │   ├── parser/
│       │   │   ├── mod.rs
│       │   │   ├── txt.rs
│       │   │   ├── pdf.rs
│       │   │   └── xlsx.rs
│       │   ├── search/
│       │   └── printer/
│       └── tests/
├── data/
├── external/
│   ├── calamine/
│   └── pdf-rs/
└── Cargo.toml
```

## Build

```bash
cargo build -p core
```

## Run

Run the binary with a search pattern and optionally a directory to scan:

```bash
cargo run -p core -- "abcde"
```

If you do not provide a directory, the binary uses the sample data directory under data/.

## Example

```bash
cargo run -p core -- "abcde"
```

Example output for a match looks like:

```text
C:\...\data\intro.txt
0| 0, 5
```

If no matches are found, the program prints:

```text
no
```

## Testing

Run the core test suite with:

```bash
cargo test -p core
```

## Notes

- The parser implementations currently emit lightweight parsed representations for each document type.
- The XLSX parser exposes cell addresses like A1, B1, and so on.
- The PDF parser exposes page markers such as page:1.
- The search layer works over the parsed bytes returned by each parser.
