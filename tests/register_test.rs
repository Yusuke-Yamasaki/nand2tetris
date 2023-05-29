extern crate nand2tetris;

use nand2tetris::circuit::flip_flop as ff;

#[test]
fn dff_test() {
    let mut dff = ff::DFF::new();
    assert_eq!(false, dff.out());
    dff.clock(true);
    assert_eq!(true, dff.out());
    dff.clock(true);
    assert_eq!(true, dff.out());
    dff.clock(false);
    assert_eq!(false, dff.out());
}
