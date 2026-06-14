# Skeleton - A Grep Project

![Logo](logo.png)

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

```
.
├── crates/
│   ├── cli/          # Command-line interface
│   └── core/         # Core grep implementation
├── data/             # Sample data for testing
├── Cargo.toml        # Workspace configuration
└── Readme.md         # This file
```

## Getting Started

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

## Development

This project serves as a learning platform for:
- Systems programming in Rust
- High-performance pattern matching algorithms
- Concurrent programming patterns
- File I/O optimization techniques
