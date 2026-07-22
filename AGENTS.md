# AGENTS.md

## Project overview
- This repository contains a Rust workspace with a core binary crate and a vendored PDF parser dependency under external/pdf-rs.
- The core crate implements parser, search, and printer traits for text, PDF, and XLSX inputs.

## Working conventions
- Prefer small, focused changes that preserve the parser -> search -> printer flow.
- Keep tests alongside the relevant module when adding behavior.
- Run relevant cargo tests after modifying parser, search, or printer behavior.

## Build and test
- Run the core tests with:
  - cargo test -p core
- Run a targeted test when iterating on one feature:
  - cargo test -p core <test_name> -- --nocapture

## Notes
- The sample data directory is under data/ and is used by the core binary for smoke tests.
- The PDF parser is provided by the vendored dependency in external/pdf-rs.
