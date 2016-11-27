#[cfg(test)]
extern crate utils;

#[derive(Clone, Copy)]
pub struct Number(pub u32);

pub fn double(n: Number) -> Number {
    Number(2 * n.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::equal;

    #[test]
    fn test() {
        assert!(equal(Number(4), double(Number(2))));
    }
}
