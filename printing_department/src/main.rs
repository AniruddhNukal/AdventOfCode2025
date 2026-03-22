use std::{error::Error, fmt::Display, fs, io, ops::AddAssign};

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
    #[error("unsupported character: [{escaped_char}]")]
    InvalidCharacter { escaped_char: String },
    #[error("unequal line length: expected {expected_len}, found {found_len}")]
    UnequalLineLength {
        expected_len: usize,
        found_len: usize,
    },
    #[error("check n x l == len failed: n = {n}, l = {l}, len = {len}")]
    MultiplicationCheckFailed { n: usize, l: usize, len: usize },
    #[error("no lines in input data.")]
    NoLines,
}

#[derive(PartialEq)]
struct Floor {
    vals: Vec<char>,
    n: usize,
    l: usize,
}

impl Display for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut chars = self.vals.iter();
        for _ in 0..self.l {
            writeln!(f, "{}", chars.by_ref().take(self.n).collect::<String>())?
        }

        Ok(())
    }
}

fn parse_file(contents: String) -> Result<Floor, ParseError> {
    let mut n_poss: Option<usize> = None;
    let mut line_count: usize = 0;
    let mut num_lines: usize = 0;
    let mut vals: Vec<char> = vec![];

    for c in contents.chars() {
        match c {
            '.' | '@' | 'x' => {
                line_count.add_assign(1);
                vals.push(c);
            }
            '\r' => (), // hacky handling of windows CRLF
            '\n' => {
                match n_poss {
                    None => n_poss = Some(line_count),
                    Some(x) if x == line_count => num_lines.add_assign(1),
                    Some(x) => {
                        return Err(ParseError::UnequalLineLength {
                            expected_len: x,
                            found_len: line_count,
                        });
                    }
                };
                line_count = 0;
            }
            _ => {
                return Err(ParseError::InvalidCharacter {
                    escaped_char: c.escape_default().to_string(),
                });
            }
        }
    }

    num_lines.add_assign(1);

    if n_poss.ok_or(ParseError::NoLines)? * num_lines != vals.len() {
        return Err(ParseError::MultiplicationCheckFailed {
            n: n_poss.unwrap(), // already checked that n is non-empty in if condition
            l: num_lines,
            len: vals.len(),
        });
    }

    Ok(Floor {
        vals,
        n: n_poss.unwrap(), // already checked that n is non-empty in if condition above
        l: num_lines,
    })
}

fn remove_possible(init: Floor) -> u16 {
    let mut states: Vec<Floor> = vec![init];
    let mut removed: u16 = 0;

    while states.len() == 1 || states[states.len() - 1] != states[states.len() - 2] {
        let (state, count) = remove_once(&states[states.len() - 1]);
        states.push(state);
        removed.add_assign(count);
    }

    removed
}

fn remove_once(init: &Floor) -> (Floor, u16) {
    let mut removed: u16 = 0;
    let mut new_vals: Vec<char> = vec![];

    for (idx, &c) in init.vals.iter().enumerate() {
        match c {
            '@' if less_neighbours(init, idx) => {
                removed.add_assign(1);
                new_vals.push('x');
            }
            '@' | '.' | 'x' => new_vals.push(c),
            _ => unreachable!(),
        }
    }

    (
        Floor {
            vals: new_vals,
            n: init.n,
            l: init.l,
        },
        removed,
    )
}

fn less_neighbours(state: &Floor, idx: usize) -> bool {
    let poss_neighbours: Vec<usize> = neighbouring_idxs(idx, state.n, state.l);
    let nearby_rolls = poss_neighbours
        .into_iter()
        .map(|i| (state.vals.get(i) == Some(&'@')) as u8)
        .sum::<u8>();

    nearby_rolls <= 3
}

fn neighbouring_idxs(idx: usize, n: usize, l: usize) -> Vec<usize> {
    match idx {
        0 => vec![1, n, n + 1],
        x if x == n - 1 => vec![x - 1, x + n - 1, x + n],
        x if x == (l - 1) * n => vec![x - n, x - n + 1, x + 1],
        x if x == l * n - 1 => vec![x - n - 1, x - n, x - 1],

        x if (1..=(n - 2)).contains(&x) => vec![x - 1, x + 1, x + n - 1, x + n, x + n + 1],
        x if (((l - 1) * n + 1)..=(l * n - 2)).contains(&x) => {
            vec![x - n - 1, x - n, x - n + 1, x - 1, x + 1]
        }
        x if x % n == 0 => vec![x - n, x - n + 1, x + 1, x + n, x + n + 1],
        x if (x + 1) % n == 0 => vec![x - n - 1, x - n, x - 1, x + n - 1, x + n],

        x if x < l * n => vec![
            x - n - 1,
            x - n,
            x - n + 1,
            x - 1,
            x + 1,
            x + n - 1,
            x + n,
            x + n + 1,
        ],
        _ => vec![],
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
    let contents = read_file("printing_department/input.txt")?;
    let init = parse_file(contents)?;
    let count = remove_possible(init);

    println!("{count}");
    // println!("{state}");

    Ok(())
}
