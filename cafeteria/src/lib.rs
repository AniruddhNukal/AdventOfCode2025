use std::{fs, io};

use thiserror::Error;

#[cfg(test)]
mod tests;
pub mod func_tree;
pub mod tree;

type Int = u64;

#[derive(Debug, Error, PartialEq)]
pub enum IOError {
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
pub enum ParseError {
    #[error("expected 2 sections, found: {count}")]
    InvalidSectionCount { count: usize },
    #[error("failed to parse to int: {text}")]
    IntParseError { text: String },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Range {
    start: Int,
    end: Int,
}

impl Range {
    fn new(start: Int, end: Int) -> Range {
        Range { start, end }
    }

    fn is_in_range(&self, val: Int) -> bool {
        val >= self.start && val <= self.end
    }

    fn span(&self) -> Int {
        self.end - self.start + 1
    }
}

pub fn parse_file(contents: String) -> Result<(Vec<Range>, Vec<Int>), ParseError> {
    let (ranges, values) = split_collect_pair(&contents, "\n\n")?;

    let ranges = parse_ranges(ranges)?;
    let values = parse_values(values)?;

    Ok((ranges, values))
}

fn parse_ranges(input: &str) -> Result<Vec<Range>, ParseError> {
    input
        .split('\n')
        .map(parse_range)
        .collect::<Result<Vec<Range>, ParseError>>()
}

fn parse_range(input: &str) -> Result<Range, ParseError> {
    let (start, end) = split_collect_pair(input, "-")?;

    Ok(Range {
        start: str::parse::<Int>(start).map_err(|_| ParseError::IntParseError {
            text: start.to_string(),
        })?,
        end: str::parse::<Int>(end).map_err(|_| ParseError::IntParseError {
            text: end.to_string(),
        })?,
    })
}

fn parse_values(input: &str) -> Result<Vec<Int>, ParseError> {
    input
        .trim()
        .split('\n')
        .map(parse_value)
        .collect::<Result<Vec<Int>, ParseError>>()
}

fn parse_value(input: &str) -> Result<Int, ParseError> {
    input.parse::<Int>().map_err(|_| ParseError::IntParseError {
        text: input.to_string(),
    })
}

fn split_collect_pair<'a>(input: &'a str, pattern: &str) -> Result<(&'a str, &'a str), ParseError> {
    let v = input.split(pattern).collect::<Vec<_>>();
    if v.len() != 2 {
        return Err(ParseError::InvalidSectionCount { count: v.len() });
    }
    Ok((v[0], v[1]))
}

// IO
pub fn read_file(filename: &str) -> Result<String, IOError> {
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
