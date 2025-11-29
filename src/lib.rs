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
        if let Some(byte_count) = self.bytes {
            result.push_str(&byte_count.to_string());
        }
        if let Some(char_count) = self.chars {
            add_whitespace_if_not_empty(&mut result);
            result.push_str(&char_count.to_string());
        }
        if let Some(word_count) = self.words {
            add_whitespace_if_not_empty(&mut result);
            result.push_str(&word_count.to_string());
        }
        if let Some(line_count) = self.lines {
            add_whitespace_if_not_empty(&mut result);
            result.push_str(&line_count.to_string());
        }
        result
    }
}

impl Args {
    pub fn execute(&self) {
        let file_contents = fs::read_to_string(&self.file_path).unwrap();
        let file_name = self.file_path.file_name().unwrap().to_str().unwrap();
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
                bytes: None,
                chars: Some(fetch_char_count(&file_contents)),
                lines: Some(fetch_line_count(&file_contents)),
                words: Some(fetch_word_count(&file_contents)),
            };
        }

        println!("{} {}", result.to_string(), file_name);
    }
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
