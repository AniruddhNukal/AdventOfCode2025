use std::{error::Error, fs, io};

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
enum IOError {
    #[error("file not found: {filename}")]
    FileNotFound { filename: String },
    #[error("permission denied: {filename}")]
    PermissionDenied { filename: String },
    #[error("invalid UTF-8 in file: {filename}")]
    InvalidUtf8 { filename: String },
    #[error("failed to read {filename}: {message}")]
    Other { filename: String, message: String },
}

struct Reading {
    vals: Vec<u8>,
}

impl Reading {
    fn new() -> Self {
        Reading { vals: vec![0; 12] }
    }

    fn advance(&mut self, digit: u8) {
        self.vals.push(digit);
        for i in 0..(self.vals.len() - 1) {
            if self.vals[i + 1] > self.vals[i] {
                self.vals.remove(i);
                return;
            }
        }
        self.vals.pop();
    }

    fn reading(&self) -> u64 {
        self.vals
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &n)| n as u64 * u64::pow(10, i as u32))
            .sum()
    }
}

fn parse_file(contents: String) -> Vec<Vec<u8>> {
    contents
        .split_whitespace()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).expect("Digit not parsed.") as u8)
                .collect()
        })
        .collect()
}

fn measure(array: Vec<u8>) -> u64 {
    let mut r = Reading::new();
    for i in array {
        r.advance(i);
    }

    r.reading()
}

// IO
fn read_file(filename: &str) -> Result<String, IOError> {
    fs::read_to_string(filename).map_err(|e| {
        let name = filename.to_string();

        match e.kind() {
            io::ErrorKind::NotFound => IOError::FileNotFound { filename: name },
            io::ErrorKind::PermissionDenied => IOError::PermissionDenied { filename: name },
            // fs::read_to_string returns InvalidData when the file isn't valid UTF-8
            io::ErrorKind::InvalidData => IOError::InvalidUtf8 { filename: name },
            _ => IOError::Other {
                filename: name,
                message: e.to_string(),
            },
        }
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_file("lobby/input.txt")?;
    let arrays = parse_file(contents);
    let x = arrays
        .into_iter()
        .map(measure)
        .map(|n| n as u64)
        .sum::<u64>();
    println!("{x}");

    Ok(())
}

#[cfg(test)]
mod tests;
