use clap::{Parser, arg};

#[derive(Parser)]
#[command(name = "ccwc", version = "1.0", about = "WC in Rust", long_about = None)]
pub struct Args {
    #[arg(short = 'c', long = "bytes")]
    pub should_display_bytes: bool,
    #[arg(short = 'm', long = "chars")]
    pub should_display_chars: bool,
    #[arg(short = 'l', long = "lines")]
    pub should_display_lines: bool,
    #[arg(short = 'w', long = "words")]
    pub should_display_words: bool,
}
