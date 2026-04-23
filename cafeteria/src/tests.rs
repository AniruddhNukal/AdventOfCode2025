use crate::tree::{RangeNode, RangeTree};

use super::*;

#[test]
fn test_given_example() {
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    let parsed = parse_file(input.to_owned());
    assert_eq!(
        parsed,
        Ok((
            vec![
                Range::new(3, 5),
                Range::new(10, 14),
                Range::new(16, 20),
                Range::new(12, 18)
            ],
            vec![1, 5, 8, 11, 17, 32]
        ))
    );

    let (ranges, values) = parsed.unwrap(); // unwrapping because above assert ensures value is known to be Ok()

    let mut tree = RangeTree::new();
    tree.add(ranges[0]);
    assert_eq!(
        &tree,
        &RangeTree::with_root(Box::new(RangeNode::new(Range::new(3, 5))))
    );

    tree.add(ranges[1]);
    assert_eq!(
        &tree,
        &RangeTree::with_root(Box::new(RangeNode::with_children(
            Range::new(3, 5),
            None,
            Some(Box::new(RangeNode::new(Range::new(10, 14))))
        )))
    );

    tree.add(ranges[2]);
    assert_eq!(
        &tree,
        &RangeTree::with_root(Box::new(RangeNode::with_children(
            Range::new(3, 5),
            None,
            Some(Box::new(RangeNode::with_children(
                Range::new(10, 14),
                None,
                Some(Box::new(RangeNode::new(Range::new(16, 20))))
            )))
        )))
    )

}
