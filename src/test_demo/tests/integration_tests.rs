use test_demo;
mod common;

#[test]
fn it_adds_two() {
    assert_eq!(4, 2+2);
    common::setup();
}