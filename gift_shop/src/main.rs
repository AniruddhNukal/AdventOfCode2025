use std::{fs, io};

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

#[derive(Debug, Error, PartialEq)]
enum ParseError {
    #[error("cannot parse empty string")]
    EmptyInput,
    #[error("missing '-' separator in '{input}'")]
    MissingSeparator { input: String },
    #[error("start of range is greater than the end in '{input}'")]
    InvalidRangeOrder { input: String },
    #[error("failed to parse number: '{input}'")]
    InvalidNumber { input: String },
    #[error("number is too large for the range type: '{input}'")]
    Overflow { input: String },
}

// Range reps an inclusive range of values to be checked.
type RangeType = u32;
#[derive(Debug, PartialEq, Clone)]
struct Range {
    start: RangeType,
    end: RangeType,
}

fn parse_file(s: String) -> Result<Vec<Range>, ParseError> {
    Ok(vec![])
}

fn parse_range(s: &str) -> Result<Range, ParseError> {
    Ok(Range { start: 0, end: 0 })
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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests;
