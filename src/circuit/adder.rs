use super::basic_logic::and;
use super::basic_logic::xor;
use super::basic_logic::or;
use super::basic_logic::Word;

pub fn half_adder(a: bool, b: bool) -> [bool; 2] {
    [
        xor(a, b), // sum
        and(a, b), // carry
    ]
}

pub fn full_adder(a: bool, b: bool, c: bool) -> [bool; 2] {
    [
        half_adder(half_adder(a, b)[0], c)[0], // sum
        or(half_adder(half_adder(a, b)[0], c)[1], half_adder(a, b)[1]), // carry
    ]
}

pub fn add16(a: Word, b: Word) -> Word {
    let c0 = full_adder(a[0], b[0], false);
    let c1 = full_adder(a[1], b[1], c0[1]);
    let c2 = full_adder(a[2], b[2], c1[1]);
    let c3 = full_adder(a[3], b[3], c2[1]);
    let c4 = full_adder(a[4], b[4], c3[1]);
    let c5 = full_adder(a[5], b[5], c4[1]);
    let c6 = full_adder(a[6], b[6], c5[1]);
    let c7 = full_adder(a[7], b[7], c6[1]);
    let c8 = full_adder(a[8], b[8], c7[1]);
    let c9 = full_adder(a[9], b[9], c8[1]);
    let c10 = full_adder(a[10], b[10], c9[1]);
    let c11 = full_adder(a[11], b[11], c10[1]);
    let c12 = full_adder(a[12], b[12], c11[1]);
    let c13 = full_adder(a[13], b[13], c12[1]);
    let c14 = full_adder(a[14], b[14], c13[1]);
    let c15 = full_adder(a[15], b[15], c14[1]);
    [
        c0[0], c1[0], c2[0], c3[0], c4[0], c5[0], c6[0], c7[0], c8[0], c9[0], c10[0], c11[0],
        c12[0], c13[0], c14[0], c15[0],
    ]
}

pub fn inc16(a: Word) -> Word {
    add16(
        a,
        [
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false,
        ],
    )
}