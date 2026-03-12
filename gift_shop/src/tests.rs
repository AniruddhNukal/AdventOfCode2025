use super::*;

impl Range {
    fn new(start: RangeType, end: RangeType) -> Range {
        Range { start, end }
    }
}

#[test]
fn test_parse_file_empty_string() {
    let input = String::from("");
    let expected = Err(ParseError::EmptyInput);

    assert_eq!(parse_file(input), expected);
}

#[test]
fn test_parse_file_single_range() {
    let input = String::from("12-33");
    let expected = vec![parse_range("12-33")].into_iter().collect();

    assert_eq!(parse_file(input), expected);
}

#[test]
fn test_parse_file_two_ranges() {
    let input = String::from("11-22,90-106");
    let expected = vec![parse_range("11-22"), parse_range("90-106")]
        .into_iter()
        .collect();

    assert_eq!(parse_file(input), expected);
}

#[test]
fn test_parse_file_many_ranges() {
    let input = String::from("11-22,90-106,456-1290,38593856-38593862");
    let expected = vec![
        parse_range("11-22"),
        parse_range("90-106"),
        parse_range("456-1290"),
        parse_range("38593856-38593862"),
    ]
    .into_iter()
    .collect();

    assert_eq!(parse_file(input), expected);
}

#[test]
fn test_parse_range_small() {
    let input = "11-22";
    let expected = Ok(Range::new(11, 22));

    assert_eq!(parse_range(input), expected);
}

#[test]
fn test_parse_range_large() {
    let input = "38593856-38593862";
    let expected = Ok(Range::new(38593856, 38593862));

    assert_eq!(parse_range(input), expected);
}

#[test]
fn test_parse_range_error_empty_string() {
    let input = "";
    let expected = Err(ParseError::EmptyInput);

    assert_eq!(parse_range(input), expected);
}

#[test]
fn test_parse_range_error_missing_sep() {
    let input = "23424";
    let expected = Err(ParseError::MissingSeparator {
        input: String::from("23424"),
    });

    assert_eq!(parse_range(input), expected);
}

#[test]
fn test_parse_range_error_range_order() {
    let input = "56-45";
    let expected = Err(ParseError::InvalidRangeOrder {
        input: String::from("56-45"),
    });

    assert_eq!(parse_range(input), expected);
}

#[test]
fn test_parse_range_error_invalid_number_letters() {
    let input = "a34-kjfs";
    let expected = Err(ParseError::InvalidNumber {
        input: String::from("a34"),
    });

    assert_eq!(parse_range(input), expected);
}

#[test]
fn test_parse_range_error_invalid_number_decimals() {
    let input = "1.20-123";
    let expected = Err(ParseError::InvalidNumber {
        input: String::from("1.20"),
    });

    assert_eq!(parse_range(input), expected);
}

#[test]
fn test_parse_range_error_multiple_seps() {
    let input = "1-23-45";
    let expected = Err(ParseError::TooManySeparators {
        input: input.to_string(),
    });

    assert_eq!(parse_range(input), expected);
}
