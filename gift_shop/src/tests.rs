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

#[test]
fn test_is_silly_number_false() {
    let input = 123;
    let expected = false;

    assert_eq!(is_silly_number(input), expected);
}

#[test]
fn test_is_silly_number_2_digit() {
    let input = 33;
    let expected = true;

    assert_eq!(is_silly_number(input), expected);
}

#[test]
fn test_is_silly_number_4_digit() {
    let input = 1010;
    let expected = true;

    assert_eq!(is_silly_number(input), expected);
}

#[test]
fn test_is_silly_number_8_digit() {
    let input = 12341234;
    let expected = true;

    assert_eq!(is_silly_number(input), expected);
}

#[test]
fn test_silly_numbers_in_range_same_size_small() {
    let input = Range::new(10, 45);
    let expected = vec![11, 22, 33, 44];

    assert_eq!(silly_numbers_in_range(input), expected);
}

#[test]
fn test_silly_numbers_in_range_same_size_medium() {
    let input = Range::new(1000, 1600);
    let expected = vec![1010, 1111, 1212, 1313, 1414, 1515];

    assert_eq!(silly_numbers_in_range(input), expected);
}

#[test]
fn test_silly_numbers_in_range_same_size_large() {
    let input = Range::new(100000, 107000);
    let expected = vec![100100, 101101, 102102, 103103, 104104, 105105, 106106];

    assert_eq!(silly_numbers_in_range(input), expected);
}

#[test]
fn test_silly_numbers_in_range_even_to_odd_digits() {
    let input = Range::new(60, 130);
    let expected = vec![66, 77, 88, 99];

    assert_eq!(silly_numbers_in_range(input), expected);
}

#[test]
fn test_silly_numbers_in_range_odd_to_odd_digits() {
    let input = Range::new(131, 10501);
    let expected = (10..=99).map(|n| n * 101).collect::<Vec<RangeType>>();

    assert_eq!(silly_numbers_in_range(input), expected);
}

#[test]
fn test_silly_numbers_in_range_even_to_even_digits() {
    let input = Range::new(80, 1400);
    let expected = vec![88, 99, 1010, 1111, 1212, 1313];

    assert_eq!(silly_numbers_in_range(input), expected)
}

#[test]
fn test_silly_numbers_in_range_odd_to_even_digits() {
    let input = Range::new(159, 1421);
    let expected = vec![1010, 1111, 1212, 1313, 1414];

    assert_eq!(silly_numbers_in_range(input), expected);
}

#[test]
fn test_digits_1_digit() {
    let input = 3;
    let expected = Digits(1);

    assert_eq!(digits(input), expected)
}

#[test]
fn test_digits_2_digit() {
    let input = 32;
    let expected = Digits(2);

    assert_eq!(digits(input), expected)
}

#[test]
fn test_digits_3_digit() {
    let input = 380;
    let expected = Digits(3);

    assert_eq!(digits(input), expected)
}

#[test]
fn test_digits_4_digit() {
    let input = 1000;
    let expected = Digits(4);

    assert_eq!(digits(input), expected)
}

#[test]
fn test_digits_5_digit() {
    let input = 99999;
    let expected = Digits(5);

    assert_eq!(digits(input), expected)
}

#[test]
fn test_sandwich_digits_2_digits() {
    let input = Digits(2);
    let expected = Some(11);

    assert_eq!(sandwich_digits(input), expected);
}

#[test]
fn test_sandwich_digits_3_digits() {
    let input = Digits(3);
    let expected = None;

    assert_eq!(sandwich_digits(input), expected);
}

#[test]
fn test_sandwich_digits_4_digits() {
    let input = Digits(4);
    let expected = Some(101);

    assert_eq!(sandwich_digits(input), expected);
}

#[test]
fn test_sandwich_digits_5_digits() {
    let input = Digits(5);
    let expected = None;

    assert_eq!(sandwich_digits(input), expected);
}

#[test]
fn test_next_silly_number_1_digit() {
    let input = 6;
    let expected = 11;

    assert_eq!(next_silly_number(input), expected);
}

#[test]
fn test_next_silly_number_2_digits() {
    let input = 37;
    let expected = 44;

    assert_eq!(next_silly_number(input), expected);
}

#[test]
fn test_next_silly_number_3_digits() {
    let input = 159;
    let expected = 1010;

    assert_eq!(next_silly_number(input), expected);
}

#[test]
fn test_next_silly_number_4_digits() {
    let input = 4920;
    let expected = 4949;

    assert_eq!(next_silly_number(input), expected);
}
