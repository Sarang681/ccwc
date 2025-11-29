# ccwc - Rust Word Count Utility

ccwc is a Rust implementation of the classic Unix wc (word count) command. It efficiently counts lines, words, bytes, and characters from files or standard input using the clap argument parser.

## Features

- Counts lines (-l), words (-w), bytes (-c), and characters (-m)

- Supports optional file input (defaults to stdin)

- Zero dependencies beyond standard Rust CLI tooling

- Fast and memory-efficient for large files
  
## Installation
1. Clone the repository:
```bash
git clone https://github.com/Sarang681/ccwc.git
cd ccwc
```

2. Build with Cargo:
```bash
cargo build --release
```

3. Use the binary:
```bash
./target/release/ccwc [OPTIONS] [FILE]
```

## Usage
```bash
ccwc [OPTIONS] [FILE]

FLAGS:
    -c, --bytes     Display byte count
    -m, --chars     Display character count
    -l, --lines     Display line count
    -w, --words     Display word count

ARGS:
    <FILE>    Optional input file (reads from stdin if omitted)
```

## Examples

Count lines and words in a file:
```bash
ccwc -l -w README.md
```

Count bytes from stdin:
```bash
echo "Hello world" | ccwc -c
```

View full help:
```bash
ccwc --help
```

## Building from Source
Requires Rust 1.70+ and Cargo. The project uses:

- clap for argument parsing
- std for file I/O and counting

## License

MIT License - see LICENSE file for details.
