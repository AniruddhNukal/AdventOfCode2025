use std::{fs, io, str::FromStr};

use thiserror::Error;

#[cfg(test)]
mod tests;

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
    #[error("The operation '{string}' is not defined.")]
    InvalidOp { string: String },
    #[error("Could not parse to number: {string}")]
    NumParseErr { string: String },
}

type XInt = u64;

#[derive(Clone, Debug, PartialEq)]
enum OpType {
    Add,
    Mult,
}

impl FromStr for OpType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(OpType::Add),
            "*" => Ok(OpType::Mult),
            s => Err(ParseError::InvalidOp {
                string: s.to_string(),
            }),
        }
    }
}

pub struct Operation {
    values: Vec<XInt>,
    op: OpType,
}

impl Operation {
    pub fn evaluate(&self) -> XInt {
        match self.op {
            OpType::Add => self.values.iter().sum::<XInt>(),
            OpType::Mult => self.values.iter().product::<XInt>(),
        }
    }
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

pub fn parse_file(contents: String) -> Result<Vec<Operation>, ParseError> {
    let lines = contents.trim().split('\n').collect::<Vec<_>>();
    let num_lines = lines
        .iter()
        .take(lines.len() - 1)
        .map(|&l| {
            l.split_whitespace()
                .map(|v| {
                    let r = XInt::from_str(v.trim());
                    r.map_err(|_| ParseError::NumParseErr {
                        string: v.trim().to_string(),
                    })
                })
                .collect::<Result<Vec<_>, ParseError>>()
        })
        .collect::<Result<Vec<_>, ParseError>>()?;
    let op_line = lines[lines.len() - 1]
        .split_whitespace()
        .map(OpType::from_str)
        .collect::<Result<Vec<_>, ParseError>>()?;

    Ok(op_line
        .iter()
        .enumerate()
        .map(|(i, op)| {
            let vals = num_lines.iter().map(|line| line[i]).collect::<Vec<_>>();
            Operation {
                op: op.clone(),
                values: vals,
            }
        })
        .collect::<Vec<_>>())
}

pub fn parse_file_2(contents: String) -> Result<Vec<Operation>, ParseError> {
    let lines = contents.split('\n').collect::<Vec<_>>();
    let line_len = lines[0].len();
    let lines = lines
        .iter()
        .filter(|line| line.len() == line_len)
        .collect::<Vec<_>>();

    let columns: Vec<String> = (0..line_len)
        .map(|i| {
            lines
                .iter()
                .map(|&line| {
                    line.char_indices()
                        .filter_map(|(j, c)| if i == j { Some(c) } else { None })
                        .next()
                        .unwrap()
                })
                .collect::<String>()
        })
        .collect();

    let pre_operations = columns
        .split(|sub| sub.trim().is_empty())
        .collect::<Vec<_>>();

    pre_operations.iter().map(|&pre_op| {
        let op = OpType::from_str(pre_op[0].chars().last().unwrap().to_string().as_str())?;
        let vals = pre_op.iter().map(|line| {
            line.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse::<XInt>().map_err(|_| { ParseError::NumParseErr { string: line.to_string() }})
        }).collect::<Result<Vec<_>, ParseError>>()?;

        Ok(Operation { op, values: vals })
    }).collect::<Result<Vec<_>, ParseError>>()

    // println!("{lines:?}");
    // println!("{columns:?}");
    // println!("{pre_operations:?}");
}
