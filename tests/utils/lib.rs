extern crate example;

use example::Number;

pub fn equal(lhs: Number, rhs: Number) -> bool {
    lhs.0 == rhs.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use example::Number;

    #[test]
    fn test() {
        assert_eq!(equal(Number(0), Number(0)), true);
        assert_eq!(equal(Number(1), Number(1)), true);
        assert_eq!(equal(Number(0), Number(1)), false);
        assert_eq!(equal(Number(1), Number(0)), false);
    }
}
