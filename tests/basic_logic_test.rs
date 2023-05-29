extern crate nand2tetris;

use nand2tetris::circuit::basic_logic as base;

#[test]
fn nand_test() {
    assert_eq!(true, base::nand(false, false));
    assert_eq!(true, base::nand(false, true));
    assert_eq!(true, base::nand(true, false));
    assert_eq!(false, base::nand(true, true));
}

#[test]
fn not_test() {
    assert_eq!(true, base::not(false));
    assert_eq!(false, base::not(true));
}

#[test]
fn and_test() {
    assert_eq!(false, base::and(false, false));
    assert_eq!(false, base::and(false, true));
    assert_eq!(false, base::and(true, false));
    assert_eq!(true, base::and(true, true));
}

#[test]
fn or_test() {
    assert_eq!(false, base::or(false, false));
    assert_eq!(true, base::or(false, true));
    assert_eq!(true, base::or(true, false));
    assert_eq!(true, base::or(true, true));
}

#[test]
fn xor_test() {
    assert_eq!(false, base::xor(false, false));
    assert_eq!(true, base::xor(false, true));
    assert_eq!(true, base::xor(true, false));
    assert_eq!(false, base::xor(true, true));
}
