use std::{
    fs,
    io::{Read, stdin},
    path::PathBuf,
};

use clap::{Parser, arg};

#[derive(Parser)]
#[command(
    name = "ccwc",
    version = "1.0",
    about = "WC in Rust",
    long_about = "Command-line argument parser for the `ccwc` tool, a Rust implementation of WC (word count)."
)]
pub struct Args {
    /// Display the byte count
    #[arg(short = 'c', long = "bytes")]
    pub should_display_bytes: bool,
    /// Display the character count
    #[arg(short = 'm', long = "chars")]
    pub should_display_chars: bool,
    /// Display the line count
    #[arg(short = 'l', long = "lines")]
    pub should_display_lines: bool,
    /// Display the word count
    #[arg(short = 'w', long = "words")]
    pub should_display_words: bool,
    /// Optional path to the input file; reads from stdin if not provided
    pub file_path: Option<PathBuf>,
}

struct Count {
    bytes: Option<usize>,
    chars: Option<usize>,
    lines: Option<usize>,
    words: Option<usize>,
}

impl Count {
    fn new() -> Self {
        Self {
            bytes: None,
            chars: None,
            lines: None,
            words: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.bytes.is_none() && self.chars.is_none() && self.lines.is_none() && self.words.is_none()
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        if let Some(line_count) = self.lines {
            add_whitespace_if_not_empty(&mut result);
            result.push_str(&line_count.to_string());
        }
        if let Some(word_count) = self.words {
            add_whitespace_if_not_empty(&mut result);
            result.push_str(&word_count.to_string());
        }
        if let Some(byte_count) = self.bytes {
            add_whitespace_if_not_empty(&mut result);
            result.push_str(&byte_count.to_string());
        }
        if let Some(char_count) = self.chars {
            add_whitespace_if_not_empty(&mut result);
            result.push_str(&char_count.to_string());
        }

        result
    }
}

impl Args {
    pub fn execute(&self) {
        let file_contents: String;
        let file_name: &str;
        if let Some(file_path) = &self.file_path {
            file_contents = fs::read_to_string(file_path).unwrap();
            file_name = file_path.file_name().unwrap().to_str().unwrap();
        } else {
            file_name = "";
            file_contents = read_from_stdin();
        }
        let mut result = Count::new();

        if self.should_display_bytes {
            let byte_count = fetch_byte_count(&file_contents);
            result.bytes = Some(byte_count);
        }

        if self.should_display_lines {
            let line_count = fetch_line_count(&file_contents);
            result.lines = Some(line_count);
        }

        if self.should_display_words {
            let words_count = fetch_word_count(&file_contents);
            result.words = Some(words_count);
        }

        if self.should_display_chars {
            let chars_count = fetch_char_count(&file_contents);
            result.chars = Some(chars_count);
        }

        if result.is_empty() {
            result = Count {
                bytes: Some(fetch_byte_count(&file_contents)),
                chars: None,
                lines: Some(fetch_line_count(&file_contents)),
                words: Some(fetch_word_count(&file_contents)),
            };
        }

        println!("{} {}", result.to_string(), file_name);
    }
}

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read input from standard input");
    buffer
}

fn fetch_char_count(file_contents: &str) -> usize {
    let chars_vec: &Vec<char> = &file_contents.chars().collect();
    let chars_count = chars_vec.len();
    chars_count
}

fn fetch_word_count(file_contents: &str) -> usize {
    let words_vec: &Vec<&str> = &file_contents.split_whitespace().collect();
    let words_count = words_vec.len();
    words_count
}

fn fetch_line_count(file_contents: &str) -> usize {
    let lines_vec: &Vec<&str> = &file_contents.lines().collect();
    let line_count = lines_vec.len();
    line_count
}

fn fetch_byte_count(file_contents: &str) -> usize {
    file_contents.as_bytes().len()
}

fn add_whitespace_if_not_empty(result: &mut String) {
    if !result.is_empty() {
        result.push_str(" ");
    }
}
