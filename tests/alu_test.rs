extern crate nand2tetris;

use nand2tetris::circuit::alu::alu;
use nand2tetris::circuit::basic_logic::and16;
use nand2tetris::circuit::basic_logic::not16;
use nand2tetris::circuit::basic_logic::or16;
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
fn alu_test() {
    let a = int2word(15);
    let b = int2word(10);

    assert_eq!(
        (int2word(0), true, false),
        alu(a, b, true, false, true, false, true, false)
    );
    assert_eq!(
        (int2word(1), false, false),
        alu(a, b, true, true, true, true, true, true)
    );
    assert_eq!(
        (int2word(-1), false, true),
        alu(a, b, true, true, true, false, true, false)
    );
    assert_eq!(
        (int2word(15), false, false),
        alu(a, b, false, false, true, true, false, false)
    );
    assert_eq!(
        (int2word(10), false, false),
        alu(a, b, true, true, false, false, false, false)
    );
    assert_eq!(
        (not16(a), false, true),
        alu(a, b, false, false, true, true, false, true)
    );
    assert_eq!(
        (not16(b), false, true),
        alu(a, b, true, true, false, false, false, true)
    );
    assert_eq!(
        (int2word(-15), false, true),
        alu(a, b, false, false, true, true, true, true)
    );
    assert_eq!(
        (int2word(-10), false, true),
        alu(a, b, true, true, false, false, true, true)
    );
    assert_eq!(
        (int2word(16), false, false),
        alu(a, b, false, true, true, true, true, true)
    );
    assert_eq!(
        (int2word(11), false, false),
        alu(a, b, true, true, false, true, true, true)
    );
    assert_eq!(
        (int2word(14), false, false),
        alu(a, b, false, false, true, true, true, false)
    );
    assert_eq!(
        (int2word(9), false, false),
        alu(a, b, true, true, false, false, true, false)
    );
    assert_eq!(
        (int2word(25), false, false),
        alu(a, b, false, false, false, false, true, false)
    );
    assert_eq!(
        (int2word(5), false, false),
        alu(a, b, false, true, false, false, true, true)
    );
    assert_eq!(
        (int2word(-5), false, true),
        alu(a, b, false, false, false, true, true, true)
    );
    assert_eq!(
        (and16(a, b), false, false),
        alu(a, b, false, false, false, false, false, false)
    );
    assert_eq!(
        (or16(a, b), false, false),
        alu(a, b, false, true, false, true, false, true)
    );
}
