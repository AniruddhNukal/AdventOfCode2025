use std::ops::{Add, AddAssign};

use thiserror::Error;

const DIAL_SIZE: u8 = 100;

#[derive(Debug, Error, PartialEq)]
enum IOError {
    #[error("File {filename} not found.")]
    FileNotFound { filename: String },
}

#[derive(Debug, Error, PartialEq)]
enum ParseError {
    #[error("Direction could not be parsed from \"{input}\".")]
    InvalidDirection { input: String },
    #[error("Steps could not be parsed from \"{input}\".")]
    InvalidSteps { input: String },
    #[error("Could not parse from empty string.")]
    EmptyString,
}

// Rotation reps a rotation of the dial.
#[derive(Debug, PartialEq, Clone)]
struct Rotation {
    dir: Direction,
    steps: Steps,
}

// Direction reps the direction of the rotation.
#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Left,
    Right,
}

// Steps reps the number of steps in a rotation.
#[derive(Debug, PartialEq, Clone)]
struct Steps(u16);

// DialState reps a current state of a dial.
#[derive(Debug, PartialEq, Clone)]
struct DialState(u8);

// ZeroCount reps the number of times the dial crosses 0.
#[derive(Debug, PartialEq, Clone)]
struct ZeroCount(u16);

impl Add for ZeroCount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let ZeroCount(l) = self;
        let ZeroCount(r) = rhs;
        ZeroCount(l + r)
    }
}

impl AddAssign for ZeroCount {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

fn parse_rotations(los: Vec<String>) -> Result<Vec<Rotation>, ParseError> {
    los.into_iter().map(parse_single_rotation).collect()
}

fn parse_single_rotation(s: String) -> Result<Rotation, ParseError> {
    let mut s = s.chars();
    let dir = match s.next() {
        Some('L') => Direction::Left,
        Some('R') => Direction::Right,
        Some(input) => {
            return Err(ParseError::InvalidDirection {
                input: String::from(input),
            });
        }
        None => return Err(ParseError::EmptyString),
    };

    let s = s.collect::<String>();
    let steps = Steps(
        s.parse::<u16>()
            .map_err(|_| ParseError::InvalidSteps { input: s })?,
    );

    Ok(Rotation { dir, steps })
}

fn apply_rotations(init_ds: DialState, rots: Vec<Rotation>) -> ZeroCount {
    let mut count = ZeroCount(0);
    let mut ds = init_ds;
    let mut zc;

    for rot in rots {
        (ds, zc) = rotate_dial_any_click(ds, rot);
        count += zc;
    }

    count
}

fn rotate_dial(ds: DialState, r: Rotation) -> (DialState, ZeroCount) {
    let DialState(state) = ds;
    let Rotation {
        dir,
        steps: Steps(steps),
    } = r;

    let change = steps as i32
        * match dir {
            Direction::Left => -1,
            Direction::Right => 1,
        };

    let final_state = i32::rem_euclid(state as i32 + change, DIAL_SIZE as i32);
    let zc = if ((final_state != state as i32 + change) && (state != 0))
        || ((final_state == 0) && (steps != 0))
    {
        1
    } else {
        0
    };
    (DialState(final_state as u8), ZeroCount(zc))
}

fn rotate_dial_any_click(ds: DialState, r: Rotation) -> (DialState, ZeroCount) {
    let Rotation {
        dir,
        steps: Steps(s),
    } = r;
    let zc = ZeroCount(u16::div_euclid(s, DIAL_SIZE as u16));
    let s = Steps(u16::rem_euclid(s, DIAL_SIZE as u16));

    let (ds, zc_f) = rotate_dial(ds, Rotation { dir, steps: s });

    (ds, zc + zc_f)
}

// IO
// reads file and provides each line as a separate string.
fn read_file(filename: &str) -> Result<Vec<String>, IOError> {
    Ok(std::fs::read_to_string(filename)
        .map_err(|_| IOError::FileNotFound {
            filename: filename.to_string(),
        })?
        .lines()
        .map(String::from)
        .collect())
}

fn main() {
    let los = read_file("secret_entrance/input.txt").unwrap();
    let rots = parse_rotations(los).unwrap();
    let init_ds = DialState(50);
    let fc = apply_rotations(init_ds, rots);
    println!("{:?}", fc);
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Happy Path Tests ---

    #[test]
    fn test_parse_standard_rotations() {
        let input = vec!["L34".to_string(), "R13".to_string(), "L7".to_string()];

        // Wrap the expected vector in Ok()
        let expected = Ok(vec![
            Rotation {
                dir: Direction::Left,
                steps: Steps(34),
            },
            Rotation {
                dir: Direction::Right,
                steps: Steps(13),
            },
            Rotation {
                dir: Direction::Left,
                steps: Steps(7),
            },
        ]);

        assert_eq!(parse_rotations(input), expected);
    }

    #[test]
    fn test_parse_empty_input() {
        let input: Vec<String> = vec![];
        let expected = Ok(vec![]);

        assert_eq!(parse_rotations(input), expected);
    }

    // --- Error Path Tests ---

    #[test]
    fn test_fails_on_invalid_direction() {
        let input = vec!["L10".to_string(), "X20".to_string()];

        // Expect the function to abort and return the specific Err
        let expected = Err(ParseError::InvalidDirection {
            input: String::from("X"),
        });

        assert_eq!(parse_rotations(input), expected);
    }

    #[test]
    fn test_fails_on_invalid_number() {
        let input = vec!["Rxyz".to_string()];

        let expected = Err(ParseError::InvalidSteps {
            input: String::from("xyz"),
        });

        assert_eq!(parse_rotations(input), expected);
    }

    #[test]
    fn test_fails_on_empty_string_in_list() {
        let input = vec!["L5".to_string(), "".to_string()];

        let expected = Err(ParseError::EmptyString);

        assert_eq!(parse_rotations(input), expected);
    }

    // Rotation Standard Tests

    #[test]
    fn test_rotate_right_increases_value() {
        let initial = DialState(10);
        let rotation = Rotation {
            dir: Direction::Right,
            steps: Steps(5),
        };
        let expected = (DialState(15), ZeroCount(0));

        assert_eq!(rotate_dial(initial, rotation), expected);
    }

    #[test]
    fn test_rotate_left_decreases_value() {
        let initial = DialState(10);
        let rotation = Rotation {
            dir: Direction::Left,
            steps: Steps(5),
        };
        let expected = (DialState(5), ZeroCount(0));

        assert_eq!(rotate_dial(initial, rotation), expected);
    }

    // Zero Rotation Tests

    #[test]
    fn test_rotate_zero_right_does_nothing() {
        let initial = DialState(10);
        let rotation = Rotation {
            dir: Direction::Right,
            steps: Steps(0),
        };
        let expected = (DialState(10), ZeroCount(0));

        assert_eq!(rotate_dial(initial, rotation), expected);
    }

    #[test]
    fn test_rotate_zero_left_does_nothing() {
        let initial = DialState(10);
        let rotation = Rotation {
            dir: Direction::Left,
            steps: Steps(0),
        };
        let expected = (DialState(10), ZeroCount(0));

        assert_eq!(rotate_dial(initial, rotation), expected);
    }

    // Land on Zero Tests

    #[test]
    fn test_rotate_right_onto_zero() {
        let initial = DialState(95);
        let rotation = Rotation {
            dir: Direction::Right,
            steps: Steps(5),
        };
        let expected = (DialState(0), ZeroCount(1));

        assert_eq!(rotate_dial(initial, rotation), expected);
    }

    #[test]
    fn test_rotate_left_onto_zero() {
        let initial = DialState(30);
        let rotation = Rotation {
            dir: Direction::Left,
            steps: Steps(30),
        };
        let expected = (DialState(0), ZeroCount(1));

        assert_eq!(rotate_dial(initial, rotation), expected);
    }

    // --- Basic Movement (No Zeros) ---

    #[test]
    fn test_zero_steps_no_cross() {
        let state = DialState(50);
        let rot = Rotation {
            dir: Direction::Right,
            steps: Steps(0),
        };
        let expected = (DialState(50), ZeroCount(0));

        assert_eq!(rotate_dial_any_click(state, rot), expected);
    }

    #[test]
    fn test_normal_move_no_cross() {
        let state = DialState(10);
        // Move 50, landing cleanly on 60 without hitting the 100 boundary
        let rot = Rotation {
            dir: Direction::Right,
            steps: Steps(50),
        };
        let expected = (DialState(60), ZeroCount(0));

        assert_eq!(rotate_dial_any_click(state, rot), expected);
    }

    // --- Single Crossings / Exact Lands ---

    #[test]
    fn test_land_exactly_on_zero_right() {
        let state = DialState(94);
        // 94 + 6 = 100 (which wraps to 0 based on DIAL_SIZE)
        let rot = Rotation {
            dir: Direction::Right,
            steps: Steps(6),
        };
        let expected = (DialState(0), ZeroCount(1));

        assert_eq!(rotate_dial_any_click(state, rot), expected);
    }

    #[test]
    fn test_cross_zero_once_left() {
        let state = DialState(5);
        // Moving left 10 from 5: hits 0 at 5 steps, then goes to 95
        // (100 - 5 remaining steps)
        let rot = Rotation {
            dir: Direction::Left,
            steps: Steps(10),
        };
        let expected = (DialState(95), ZeroCount(1));

        assert_eq!(rotate_dial_any_click(state, rot), expected);
    }

    // --- Multi-Wrap / Heavy Rotations ---

    #[test]
    fn test_full_circle_rotation() {
        let state = DialState(10);
        // Exactly DIAL_SIZE (100) steps is one full rotation, crossing 0 exactly once
        let rot = Rotation {
            dir: Direction::Right,
            steps: Steps(DIAL_SIZE as u16),
        };
        let expected = (DialState(10), ZeroCount(1));

        assert_eq!(rotate_dial_any_click(state, rot), expected);
    }

    #[test]
    fn test_multiple_crossings_right() {
        let state = DialState(94);
        // +6 steps hits 0 (1st cross)
        // +100 steps hits 0 (2nd cross)
        // +100 steps hits 0 (3rd cross)
        // +10 steps lands on 10
        // Total steps = 6 + 100 + 100 + 10 = 216
        let rot = Rotation {
            dir: Direction::Right,
            steps: Steps(216),
        };
        let expected = (DialState(10), ZeroCount(3));

        assert_eq!(rotate_dial_any_click(state, rot), expected);
    }

    #[test]
    fn test_multiple_crossings_left() {
        let state = DialState(10);
        // -10 steps hits 0 (1st cross)
        // -100 steps hits 0 (2nd cross)
        // -5 steps lands on 95
        // Total steps = 10 + 100 + 5 = 115
        let rot = Rotation {
            dir: Direction::Left,
            steps: Steps(115),
        };
        let expected = (DialState(95), ZeroCount(2));

        assert_eq!(rotate_dial_any_click(state, rot), expected);
    }

    // --- Edge Cases ---

    #[test]
    fn test_start_on_zero_and_move_away() {
        let state = DialState(0);
        // If we start on 0 and move right 10, we don't count 0 again
        let rot = Rotation {
            dir: Direction::Right,
            steps: Steps(10),
        };
        let expected = (DialState(10), ZeroCount(0));

        assert_eq!(rotate_dial_any_click(state, rot), expected);
    }

    #[test]
    fn test_start_on_zero_and_do_full_circles() {
        let state = DialState(0);
        // Start on 0, do exactly two full loops, landing back on 0
        // 200 steps total
        let rot = Rotation {
            dir: Direction::Left,
            steps: Steps((DIAL_SIZE * 2) as u16),
        };
        let expected = (DialState(0), ZeroCount(2));

        assert_eq!(rotate_dial_any_click(state, rot), expected);
    }

    // --- The "Did I count my starting position?" Tests ---

    #[test]
    fn test_start_0_move_0() {
        let state = DialState(0);
        let rot = Rotation {
            dir: Direction::Right,
            steps: Steps(0),
        };
        // Starting on 0 and not moving should NOT count as a land/cross
        let expected = (DialState(0), ZeroCount(0));

        assert_eq!(rotate_dial_any_click(state, rot), expected);
    }

    #[test]
    fn test_start_0_move_away_left() {
        let state = DialState(0);
        let rot = Rotation {
            dir: Direction::Left,
            steps: Steps(1),
        };
        // Moving from 0 to 99 means you moved away. You didn't land or cross 0 again.
        let expected = (DialState(DIAL_SIZE - 1), ZeroCount(0));

        assert_eq!(rotate_dial_any_click(state, rot), expected);
    }

    // --- The "Almost Full Circle" Tests ---

    #[test]
    fn test_start_0_move_almost_full_circle() {
        let state = DialState(0);
        let rot = Rotation {
            dir: Direction::Right,
            steps: Steps((DIAL_SIZE - 1) as u16),
        };
        // Moving from 0 to 99 (on a 100 dial). Never hit 0 again.
        let expected = (DialState(DIAL_SIZE - 1), ZeroCount(0));

        assert_eq!(rotate_dial_any_click(state, rot), expected);
    }

    #[test]
    fn test_start_1_move_almost_full_circle_left() {
        let state = DialState(1);
        let rot = Rotation {
            dir: Direction::Left,
            steps: Steps((DIAL_SIZE - 1) as u16),
        };
        // 1 -> 0 (cross 1) -> 99 -> ... -> 2. Exactly one crossing.
        // If your math is overcounting, it might output 2 here.
        let expected = (DialState(2), ZeroCount(1));

        assert_eq!(rotate_dial_any_click(state, rot), expected);
    }

    // --- The Exact Multiple Boundary Tests ---
    // These are notorious for double-counting if you use division AND modulo separately

    #[test]
    fn test_start_1_land_exactly_on_0_left() {
        let state = DialState(1);
        let rot = Rotation {
            dir: Direction::Left,
            steps: Steps(1),
        };
        let expected = (DialState(0), ZeroCount(1));

        assert_eq!(rotate_dial_any_click(state, rot), expected);
    }

    #[test]
    fn test_start_99_move_right_full_circle() {
        let state = DialState(DIAL_SIZE - 1);
        let rot = Rotation {
            dir: Direction::Right,
            steps: Steps(DIAL_SIZE as u16),
        };
        // 99 -> 0 (cross 1) -> 1 -> ... -> 99.
        // Even though we moved 100 steps, we only hit 0 once.
        let expected = (DialState(DIAL_SIZE - 1), ZeroCount(1));

        assert_eq!(rotate_dial_any_click(state, rot), expected);
    }

    #[test]
    fn test_start_0_move_exactly_one_circle_left() {
        let state = DialState(0);
        let rot = Rotation {
            dir: Direction::Left,
            steps: Steps(DIAL_SIZE as u16),
        };
        // 0 -> 99 -> ... -> 0. Hits zero exactly at the end.
        // Often overcounted as 2 if the starting 0 isn't ignored.
        let expected = (DialState(0), ZeroCount(1));

        assert_eq!(rotate_dial_any_click(state, rot), expected);
    }
}
