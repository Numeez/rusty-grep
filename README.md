# rusty-grep

A fast, flexible, and colorful `grep`-like command-line tool written in Rust.

## Features

- Search for patterns in files (single or multiple).
- Supports recursive search through directories.
- Optional **case-insensitive** search.
- Supports **regex** patterns.
- Color highlighting for matched text.
- Option to show line numbers, counts only, or file headers.
- Skips binary and irrelevant file types automatically.

## Installation

You can install `rusty-grep` directly from this GitHub repository using Cargo:

```bash
cargo install --git https://github.com/Numeez/rusty-grep.git
