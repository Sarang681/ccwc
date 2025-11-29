use std::{fs, path::PathBuf};

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

    pub file_path: PathBuf,
}

impl Args {
    pub fn execute(&self) {
        let file_contents = fs::read_to_string(&self.file_path).unwrap();
        let file_name = self.file_path.file_name().unwrap().to_str().unwrap();
        let mut result = String::new();

        if self.should_display_bytes {
            result.push_str(&file_contents.as_bytes().len().to_string());
        }

        println!("{} {}", result, file_name);
    }
}
