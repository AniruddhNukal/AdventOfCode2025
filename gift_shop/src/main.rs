use std::{error::Error, fs, io, ops::Add};

use thiserror::Error;

static EXPECT_MSG: &str = "If n is a silly number, sandwich(n) is guaranteed to exist.";

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
type RangeType = u64;
#[derive(Debug, PartialEq, Clone)]
struct Range {
    start: RangeType,
    end: RangeType,
}

#[derive(Debug, PartialEq, Clone)]
struct Digits(RangeType);

impl Add for Digits {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let Self(l) = self;
        let Self(r) = rhs;
        Self(l + r)
    }
}

struct SillyIterator {
    curr: RangeType,
}

impl Iterator for SillyIterator {
    type Item = RangeType;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr = next_silly_number(self.curr);
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

fn is_silly_number(n: RangeType) -> bool {
    let sandwich = match sandwich(n) {
        Some(sw) => sw,
        None => return false,
    };
    match n % sandwich {
        0 => true,
        _ => false,
    }
}

fn silly_numbers_in_range(r: Range) -> Vec<RangeType> {
    let Range { start, end } = r;
    let silly_iter = SillyIterator { curr: start - 1 };
    let silly_nums = silly_iter.take_while(|&n| n <= end).collect();

    silly_nums
}

fn next_silly_number(n: RangeType) -> RangeType {
    if is_silly_number(n) && is_silly_number(n + sandwich(n).expect(EXPECT_MSG)) {
        n + sandwich(n).expect(EXPECT_MSG)
    } else if is_silly_number(n) {
        let x = sandwich_digits(digits(n) + Digits(2)).expect("even digits + 2 is even");
        x * (x / 10)
    } else if let Some(sw) = sandwich(n) {
        n + sw - (n % sw)
    } else {
        let x = sandwich_digits(digits(n) + Digits(1)).expect("even digits + 2 is even");
        x * (x / 10)
    }
}

fn sandwich(n: RangeType) -> Option<RangeType> {
    sandwich_digits(digits(n))
}

fn digits(n: RangeType) -> Digits {
    if n == 0 {
        return Digits(1);
    }
    Digits(RangeType::ilog10(n) as RangeType + 1)
}

fn sandwich_digits(d: Digits) -> Option<RangeType> {
    let Digits(digits) = d;
    if digits % 2 == 1 {
        return None;
    }
    Some(RangeType::pow(10, digits as u32 / 2) + 1)
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
