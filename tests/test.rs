extern crate example;
extern crate utils;

use example::{Number, double};
use utils::equal;

#[test]
fn test() {
    assert!(equal(Number(0), double(Number(0))));
}
