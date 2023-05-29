extern crate nand2tetris;

use nand2tetris::circuit::adder as add;
use nand2tetris::circuit::basic_logic::Word;

fn int2word(num: i16) -> Word {
    let mut word = [false; 16];
    let bin = format!("{:16b}", num).to_string();
    for (i, bit) in bin.chars().rev().enumerate() {
        if bit == '1' {
            word[i] = true;
        }
    }
    word
}

#[test]
fn half_adder_test() {
    // (sum, carry)
    assert_eq!([false, false], add::half_adder(false, false));
    assert_eq!([true, false], add::half_adder(true, false));
    assert_eq!([true, false], add::half_adder(false, true));
    assert_eq!([false, true], add::half_adder(true, true));
}

#[test]
fn full_adder_test() {
    assert_eq!([false, false], add::full_adder(false, false, false));
    assert_eq!([true, false], add::full_adder(false, false, true));
    assert_eq!([true, false], add::full_adder(false, true, false));
    assert_eq!([false, true], add::full_adder(false, true, true));
    assert_eq!([true, false], add::full_adder(true, false, false));
    assert_eq!([false, true], add::full_adder(true, false, true));
    assert_eq!([false, true], add::full_adder(true, true, false));
    assert_eq!([true, true], add::full_adder(true, true, true));
}

#[test]
fn add16_test() {
    assert_eq!(int2word(25), add::add16(int2word(10), int2word(15)));
    assert_eq!(int2word(5), add::add16(int2word(0), int2word(5)));
    assert_eq!(int2word(0), add::add16(int2word(-5), int2word(5)));
}

#[test]
fn inc16_test() {
    assert_eq!(int2word(25), add::inc16(int2word(24)));
    assert_eq!(int2word(0), add::inc16(int2word(-1)));
    assert_eq!(int2word(-20), add::inc16(int2word(-21)));
}
