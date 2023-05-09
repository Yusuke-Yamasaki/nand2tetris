extern crate nand2tetris;

use nand2tetris::circuit::combinational_logic as comb;
#[test]
fn mux_test() {
    for a in [true, false] {
        for b in [true, false] {
            for sel in [true, false] {
                if sel {
                    assert_eq!(b, comb::mux(a, b, sel))
                } else {
                    assert_eq!(a, comb::mux(a, b, sel))
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
                assert_eq!((false, input), comb::dmux(input, sel))
            } else {
                assert_eq!((input, false), comb::dmux(input, sel))
            }
        }
    }
}
