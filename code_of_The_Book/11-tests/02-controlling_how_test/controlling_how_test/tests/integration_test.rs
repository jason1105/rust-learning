// Listing 11-13: An integration test of a function in the adder crate
use controlling_how_test;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, controlling_how_test::add_two(2));
}
