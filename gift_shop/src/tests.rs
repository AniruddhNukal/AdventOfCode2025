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
fn test_silly_numbers_in_range_same_size_small() {
    let input = Range::new(10, 45);
    let expected = vec![11, 22, 33, 44];
    let mut usv = silly_numbers_in_range(input);
    usv.sort();

    assert_eq!(usv, expected);
}

#[test]
fn test_silly_numbers_in_range_same_size_medium() {
    let input = Range::new(1000, 1600);
    let expected = vec![1010, 1111, 1212, 1313, 1414, 1515];
    let mut usv = silly_numbers_in_range(input);
    usv.sort();

    assert_eq!(usv, expected);
}

#[test]
fn test_silly_numbers_in_range_same_size_large() {
    let input = Range::new(100000, 107000);
    let expected = vec![
        100100, 101010, 101101, 102102, 103103, 104104, 105105, 106106,
    ];
    let mut usv = silly_numbers_in_range(input);
    usv.sort();

    assert_eq!(usv, expected);
}

#[test]
fn test_silly_numbers_in_range_even_to_odd_digits() {
    let input = Range::new(60, 130);
    let expected = vec![66, 77, 88, 99, 111];
    let mut usv = silly_numbers_in_range(input);
    usv.sort();

    assert_eq!(usv, expected);
}

#[test]
fn test_silly_numbers_in_range_odd_to_odd_digits() {
    let input = Range::new(131, 10501);
    let expected_2 = (10..=99).map(|n| n * 101);
    let expected_3 = (2..=9).map(|n| n * 111);
    let mut expected = expected_2.chain(expected_3).collect::<Vec<_>>();
    let mut usv = silly_numbers_in_range(input);
    usv.sort();
    expected.sort();

    assert_eq!(usv, expected);
}

#[test]
fn test_silly_numbers_in_range_even_to_even_digits() {
    let input = Range::new(80, 1400);
    let expected = vec![
        88, 99, 111, 222, 333, 444, 555, 666, 777, 888, 999, 1010, 1111, 1212, 1313,
    ];
    let mut usv = silly_numbers_in_range(input);
    usv.sort();

    assert_eq!(usv, expected);
}

#[test]
fn test_silly_numbers_in_range_odd_to_even_digits() {
    let input = Range::new(159, 1421);
    let expected = vec![
        222, 333, 444, 555, 666, 777, 888, 999, 1010, 1111, 1212, 1313, 1414,
    ];
    let mut usv = silly_numbers_in_range(input);
    usv.sort();

    assert_eq!(usv, expected);
}

#[test]
fn test_next_silly_number_1_digit() {
    let input = 6;
    let expected = 11;

    assert_eq!(next_silly_number(input, 2), expected);
}

#[test]
fn test_next_silly_number_2_digits() {
    let input = 37;
    let expected = 44;

    assert_eq!(next_silly_number(input, 2), expected);
}

#[test]
fn test_next_silly_number_3_digits() {
    let input = 159;
    let expected = 1010;

    assert_eq!(next_silly_number(input, 2), expected);
}

#[test]
fn test_next_silly_number_4_digits() {
    let input = 4920;
    let expected = 4949;

    assert_eq!(next_silly_number(input, 2), expected);
}

#[test]
fn test_iterator_functionality_iter_2() {
    let input = (0..2).collect::<Vec<_>>();
    let expected = vec![0, 1];

    assert_eq!(input, expected);
}

#[test]
fn test_iterator_functionality_iter_3() {
    let input = (0..3).collect::<Vec<_>>();
    let expected = vec![0, 1, 2];

    assert_eq!(input, expected);
}

#[test]
fn test_iterator_functionality_iter_5() {
    let input = (0..5).collect::<Vec<_>>();
    let expected = vec![0, 1, 2, 3, 4];

    assert_eq!(input, expected);
}

#[test]
fn test_iterator_functionality_iter_7() {
    let input = (0..7).collect::<Vec<_>>();
    let expected = vec![0, 1, 2, 3, 4, 5, 6];

    assert_eq!(input, expected);
}

#[test]
fn test_iterator_functionality_specific_1() {
    let input = SillyIterator::new(80, 3)
        .take_while(|&n| n <= 1400)
        .collect::<Vec<_>>();
    let expected = vec![111, 222, 333, 444, 555, 666, 777, 888, 999];

    assert_eq!(input, expected);
}

#[test]
fn test_iterator_functionality_specific_2() {
    let input = next_silly_number(80, 3);
    let expected = 111;

    assert_eq!(input, expected);
}

#[test]
fn test_silly_number_degree_two() {
    assert_eq!(next_silly_number(99, 2), 1010);
    assert_eq!(next_silly_number(9999, 2), 100100);
    assert_eq!(next_silly_number(999999, 2), 10001000);
    assert_eq!(next_silly_number(99999999, 2), 1000010000);
}

#[test]
fn test_silly_number_degree_three() {
    assert_eq!(next_silly_number(999, 3), 101010);
    assert_eq!(next_silly_number(999999, 3), 100100100);
    assert_eq!(next_silly_number(999999999, 3), 100010001000);
}

#[test]
fn test_silly_number_degree_five() {
    assert_eq!(next_silly_number(99999, 5), 1010101010);
    assert_eq!(next_silly_number(9999999999, 5), 100100100100100);
}

#[test]
fn test_silly_number_degree_seven() {
    let n = 9999999;
    let k = 7;
    let d = RangeType::ilog10(n) + 1;
    assert_eq!(d, 7);
    let p = d / k;
    assert_eq!(p, 1);
    let c = club(k, p);
    assert_eq!(c, 1111111);
    assert_eq!(next_silly_number(n, k), 10101010101010);
}

fn assert_vec_eq_ignore_order(mut actual: Vec<u128>, mut expected: Vec<u128>) {
    actual.sort_unstable();
    expected.sort_unstable();
    assert_eq!(actual, expected);
}

#[test]
fn test_silly_numbers_in_range_example() {
    // 11-22 still has two invalid IDs, 11 and 22.
    assert_vec_eq_ignore_order(
        silly_numbers_in_range(parse_range("11-22").unwrap()),
        vec![11, 22],
    );

    // 95-115 now has two invalid IDs, 99 and 111.
    assert_vec_eq_ignore_order(
        silly_numbers_in_range(parse_range("95-115").unwrap()),
        vec![99, 111],
    );

    // 998-1012 now has two invalid IDs, 999 and 1010.
    assert_vec_eq_ignore_order(
        silly_numbers_in_range(parse_range("998-1012").unwrap()),
        vec![999, 1010],
    );

    // 1188511880-1188511890 still has one invalid ID, 1188511885.
    assert_vec_eq_ignore_order(
        silly_numbers_in_range(parse_range("1188511880-1188511890").unwrap()),
        vec![1188511885],
    );

    // 222220-222224 still has one invalid ID, 222222.
    assert_vec_eq_ignore_order(
        silly_numbers_in_range(parse_range("222220-222224").unwrap()),
        vec![222222],
    );

    // 1698522-1698528 still contains no invalid IDs.
    assert_vec_eq_ignore_order(
        silly_numbers_in_range(parse_range("1698522-1698528").unwrap()),
        vec![], // Empty vector for no IDs
    );

    // 446443-446449 still has one invalid ID, 446446.
    assert_vec_eq_ignore_order(
        silly_numbers_in_range(parse_range("446443-446449").unwrap()),
        vec![446446],
    );

    // 38593856-38593862 still has one invalid ID, 38593859.
    assert_vec_eq_ignore_order(
        silly_numbers_in_range(parse_range("38593856-38593862").unwrap()),
        vec![38593859],
    );

    // 565653-565659 now has one invalid ID, 565656.
    assert_vec_eq_ignore_order(
        silly_numbers_in_range(parse_range("565653-565659").unwrap()),
        vec![565656],
    );

    // 824824821-824824827 now has one invalid ID, 824824824.
    assert_vec_eq_ignore_order(
        silly_numbers_in_range(parse_range("824824821-824824827").unwrap()),
        vec![824824824],
    );

    // 2121212118-2121212124 now has one invalid ID, 2121212121.
    assert_vec_eq_ignore_order(
        silly_numbers_in_range(parse_range("2121212118-2121212124").unwrap()),
        vec![2121212121],
    );
}
