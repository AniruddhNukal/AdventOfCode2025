use super::*;

impl Reading {
    fn check(&self, expected: u64) {
        assert_eq!(self.reading(), expected);
    }
}

#[test]
fn test_reading() {
    let input = Reading { vals: vec![4, 3] };
    input.check(43);
}

#[test]
fn test_advance_beginning() {
    let mut input = Reading::new();
    input.check(0);
    input.advance(4);
    input.check(4);
    input.advance(3);
    input.check(43);
}

#[test]
fn test_advance_middle() {
    let mut input = Reading { vals: vec![2, 3] };
    input.check(23);
    input.advance(1);
    input.check(31);
    input.advance(3);
    input.check(33);
    input.advance(1);
    input.check(33);
    input.advance(4);
    input.check(34);
    input.advance(7);
    input.check(47);
    input.advance(2);
    input.check(72);
    input.advance(6);
    input.check(76);
    input.advance(9);
    input.check(79);
    input.advance(8);
    input.check(98);
}
