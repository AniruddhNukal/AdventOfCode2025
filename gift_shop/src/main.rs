use itertools::Itertools;
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

#[derive(Debug, Error, PartialEq)]
enum ParseError {
    #[error("cannot parse empty string")]
    EmptyInput,
    #[error("missing '-' separator in '{input}'")]
    MissingSeparator { input: String },
    #[error("too many '-' separators in '{input}'")]
    TooManySeparators { input: String },
    #[error("start of range is greater than the end in '{input}'")]
    InvalidRangeOrder { input: String },
    #[error("failed to parse number: '{input}'")]
    InvalidNumber { input: String },
}

// Range reps an inclusive range of values to be checked.
type RangeType = u128;
#[derive(Debug, PartialEq, Clone)]
struct Range {
    start: RangeType,
    end: RangeType,
}

struct SillyIterator {
    curr: RangeType,
    degree: u32,
}

impl SillyIterator {
    fn new(start: RangeType, k: u32) -> Self {
        Self {
            curr: start - 1, // - 1 to count start if it is a silly number too
            degree: k,
        }
    }
}

impl Iterator for SillyIterator {
    type Item = RangeType;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr = next_silly_number(self.curr, self.degree);
        Some(self.curr)
    }
}

fn parse_file(s: String) -> Result<Vec<Range>, ParseError> {
    s.trim().split(",").map(parse_range).collect()
}

fn parse_range(s: &str) -> Result<Range, ParseError> {
    if s == "" {
        return Err(ParseError::EmptyInput);
    }
    let vals: Vec<&str> = s.split("-").collect();
    match vals.len() {
        0 => unreachable!("&str::split() should always return at least one element."),
        1 => {
            return Err(ParseError::MissingSeparator {
                input: s.to_string(),
            });
        }
        2 => (),
        _ => {
            return Err(ParseError::TooManySeparators {
                input: s.to_string(),
            });
        }
    }
    let start = vals[0]
        .parse::<RangeType>()
        .map_err(|_| ParseError::InvalidNumber {
            input: vals[0].to_string(),
        })?;
    let end = vals[1]
        .parse::<RangeType>()
        .map_err(|_| ParseError::InvalidNumber {
            input: vals[1].to_string(),
        })?;

    if start > end {
        return Err(ParseError::InvalidRangeOrder {
            input: s.to_string(),
        });
    }

    Ok(Range { start, end })
}

fn club(k: u32, p: u32) -> RangeType {
    (0..k).map(|n| RangeType::pow(10, n * p)).sum()
}

fn silly_numbers_in_range(r: Range) -> Vec<RangeType> {
    let Range { start, end } = r;
    let k_2 = SillyIterator::new(start, 2).take_while(|&n| n <= end);
    let k_3 = SillyIterator::new(start, 3).take_while(|&n| n <= end);
    let k_5 = SillyIterator::new(start, 5).take_while(|&n| n <= end);
    let k_7 = SillyIterator::new(start, 7).take_while(|&n| n <= end);

    k_2.chain(k_3).chain(k_5).chain(k_7).unique().collect()
}

fn next_silly_number(n: RangeType, k: u32) -> RangeType {
    let d = match n {
        0 => 1, // to cover the start = 1 case: iterator.start = start -1;
        _ => RangeType::ilog10(n) + 1,
    };
    let p = d / k;
    let c = club(k, p);
    if d == p * k && (n != RangeType::pow(10, d) - 1) {
        (n / c) * c + c
    } else {
        club(k, p + 1) * RangeType::pow(10, p)
    }
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
    let contents = read_file("gift_shop/input.txt")?;
    let ranges = parse_file(contents)?;
    let silly_sum: RangeType = ranges.into_iter().flat_map(silly_numbers_in_range).sum();
    println!("{silly_sum}");

    Ok(())
}

#[cfg(test)]
mod tests;
