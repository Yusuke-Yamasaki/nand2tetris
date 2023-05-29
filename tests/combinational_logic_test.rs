extern crate nand2tetris;

use nand2tetris::circuit::basic_logic::Word;
use nand2tetris::circuit::combinational_logic as comb;
#[test]
fn mux_test() {
    for a in [true, false] {
        for b in [true, false] {
            for sel in [true, false] {
                if sel {
                    assert_eq!(b, comb::mux(a, b, sel));
                } else {
                    assert_eq!(a, comb::mux(a, b, sel));
                }
            }
        }
    }
}

#[test]
fn dmux_test() {
    for input in [true, false] {
        for sel in [true, false] {
            if sel {
                assert_eq!([false, input], comb::dmux(input, sel));
            } else {
                assert_eq!([input, false], comb::dmux(input, sel));
            }
        }
    }
}

#[test]
fn mux4way16_test() {
    let a: Word = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];
    let b: Word = [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];
    let c: Word = [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];
    let d: Word = [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];
    assert_eq!(a, comb::mux4way16(a, b, c, d, [false, false]));
    assert_eq!(b, comb::mux4way16(a, b, c, d, [true, false]));
    assert_eq!(c, comb::mux4way16(a, b, c, d, [false, true]));
    assert_eq!(d, comb::mux4way16(a, b, c, d, [true, true]));
}

#[test]
#[rustfmt::skip]
fn mux8way16_test() {
    let a: Word = [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];
    let b: Word = [
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];
    let c: Word = [
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];
    let d: Word = [
        true, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];
    let e: Word = [
        true, true, true, true, false, false, false, false, false, false, false, false, false,
        false, false, false,
    ];
    let f: Word = [
        true, true, true, true, true, false, false, false, false, false, false, false, false,
        false, false, false,
    ];
    let g: Word = [
        true, true, true, true, true, true, false, false, false, false, false, false, false,
        false, false, false,
    ];
    let h: Word = [
        true, true, true, true, true, true, true, false, false, false, false, false, false,
        false, false, false,
    ];
    assert_eq!(a, comb::mux8way16(a, b, c, d, e, f, g, h, [false, false, false]));
    assert_eq!(b, comb::mux8way16(a, b, c, d, e, f, g, h, [true, false, false]));
    assert_eq!(c, comb::mux8way16(a, b, c, d, e, f, g, h, [false, true, false]));
    assert_eq!(d, comb::mux8way16(a, b, c, d, e, f, g, h, [true, true, false]));
    assert_eq!(e, comb::mux8way16(a, b, c, d, e, f, g, h, [false, false, true]));
    assert_eq!(f, comb::mux8way16(a, b, c, d, e, f, g, h, [true, false, true]));
    assert_eq!(g, comb::mux8way16(a, b, c, d, e, f, g, h, [false, true, true]));
    assert_eq!(h, comb::mux8way16(a, b, c, d, e, f, g, h, [true, true, true]));
}

#[test]
fn dmux8way_test() {
    let input: bool = true;
    assert_eq!(
        [input, false, false, false, false, false, false, false],
        comb::dmux8way(input, [false, false, false])
    );
    assert_eq!(
        [false, input, false, false, false, false, false, false],
        comb::dmux8way(input, [true, false, false])
    );
    assert_eq!(
        [false, false, input, false, false, false, false, false],
        comb::dmux8way(input, [false, true, false])
    );
    assert_eq!(
        [false, false, false, input, false, false, false, false],
        comb::dmux8way(input, [true, true, false])
    );
    assert_eq!(
        [false, false, false, false, input, false, false, false],
        comb::dmux8way(input, [false, false, true])
    );
    assert_eq!(
        [false, false, false, false, false, input, false, false],
        comb::dmux8way(input, [true, false, true])
    );
    assert_eq!(
        [false, false, false, false, false, false, input, false],
        comb::dmux8way(input, [false, true, true])
    );
    assert_eq!(
        [false, false, false, false, false, false, false, input],
        comb::dmux8way(input, [true, true, true])
    );
}
