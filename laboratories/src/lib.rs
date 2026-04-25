use std::{
    fmt::Display,
    fs, io,
    iter::Sum,
    ops::{Add, AddAssign},
    str::FromStr,
};

use thiserror::Error;

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
    #[error("The symbol '{string}' is not defined.")]
    InvalidSymbol { string: String },
    #[error("Could not parse to number: {string}")]
    NumParseErr { string: String },
}

#[derive(Clone, PartialEq, Copy)]
pub enum MapSym<
    T: AddAssign + Add<Output = T> + Clone + Copy + PartialEq + Default + From<u8> + ToString + Sum,
> {
    Space(T),
    Splitter,
}

impl<
    T: AddAssign + Add<Output = T> + Clone + Copy + PartialEq + Default + From<u8> + ToString + Sum,
> MapSym<T>
{
    fn from_char(c: char) -> Result<Self, ParseError> {
        Self::from_str(c.to_string().as_str())
    }

    fn increment(&mut self, by: &Self) {
        match self {
            MapSym::Splitter => (),
            MapSym::Space(s) => {
                *self = MapSym::Space(
                    *s + if let MapSym::Space(b) = by {
                        *b
                    } else {
                        T::default()
                    },
                )
            }
        }
    }
}

impl<
    T: AddAssign + Add<Output = T> + Clone + Copy + PartialEq + Default + From<u8> + ToString + Sum,
> FromStr for MapSym<T>
{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(MapSym::Space(0.into())),
            "S" => Ok(MapSym::Space(1.into())),
            "|" => Ok(MapSym::Space(1.into())),
            "^" => Ok(MapSym::Splitter),
            s => Err(ParseError::InvalidSymbol {
                string: s.to_string(),
            }),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Map<
    T: AddAssign + Add<Output = T> + Clone + Copy + PartialEq + Default + From<u8> + ToString + Sum,
> {
    values: Vec<Vec<MapSym<T>>>,
    propagation: usize,
}

impl<
    T: AddAssign + Add<Output = T> + Clone + Copy + PartialEq + Default + From<u8> + ToString + Sum,
> Map<T>
{
    pub fn new(values: Vec<Vec<MapSym<T>>>) -> Map<T> {
        Map::<T> {
            values,
            propagation: 0,
        }
    }

    pub fn propagate(&self) -> Map<T> {
        if self.propagation == self.values.len() - 1 {
            return self.clone();
        }

        let mut values = self.values.clone();

        let tor = self.values.get(self.propagation).unwrap();
        let tee = values.get_mut(self.propagation + 1).unwrap();

        let width = tee.len();

        for (i, sym) in tor.iter().enumerate() {
            match sym {
                MapSym::Splitter => continue,
                MapSym::Space(s) if *s == T::default() => continue,
                _ => (),
            };

            if let MapSym::Space(_) = tee[i] {
                tee[i].increment(sym);
            } else if tee[i] == MapSym::Splitter {
                if i == 0 {
                    tee[1].increment(sym);
                } else if i == width - 1 {
                    tee[width - 2].increment(sym);
                } else {
                    tee[i - 1].increment(sym);
                    tee[i + 1].increment(sym);
                }
            }
        }

        Map::<T> {
            values,
            propagation: self.propagation + 1,
        }
    }

    pub fn get_splits(&self) -> T {
        let line = self.values.get(self.propagation).unwrap();
        line.iter()
            .map(|sym| match sym {
                MapSym::Splitter => T::default(),
                MapSym::Space(s) => *s,
            })
            .sum::<T>()
    }
}

impl<
    T: AddAssign + Add<Output = T> + Clone + Copy + PartialEq + Default + From<u8> + ToString + Sum,
> Display for Map<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, line) in self.values.iter().enumerate() {
            for sym in line {
                let repr = match sym {
                    MapSym::Space(i) => i.to_string(),
                    MapSym::Splitter => "^".to_string(),
                };
                write!(f, "{}", repr)?;
            }
            if i != self.values.len() - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
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

pub fn parse_file<
    T: AddAssign + Add<Output = T> + Clone + Copy + PartialEq + Default + From<u8> + ToString + Sum,
>(
    contents: String,
) -> Result<Map<T>, ParseError> {
    let values = contents
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(MapSym::from_char)
                .collect::<Result<Vec<_>, ParseError>>()
        })
        .collect::<Result<Vec<_>, ParseError>>()?;

    Ok(Map::<T>::new(values))
}
