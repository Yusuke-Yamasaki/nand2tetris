use super::basic_logic::and;
use super::basic_logic::not;
use super::basic_logic::or;
use super::basic_logic::Word;

pub fn mux(a: bool, b: bool, sel: bool) -> bool {
    or(and(a, not(sel)), and(b, sel))
}

pub fn dmux(input: bool, sel: bool) -> [bool; 2] {
    [and(input, not(sel)), and(input, sel)]
}

pub fn mux16(a: Word, b: Word, sel: bool) -> Word {
    [
        mux(a[0], b[0], sel),
        mux(a[1], b[1], sel),
        mux(a[2], b[2], sel),
        mux(a[3], b[3], sel),
        mux(a[4], b[4], sel),
        mux(a[5], b[5], sel),
        mux(a[6], b[6], sel),
        mux(a[7], b[7], sel),
        mux(a[8], b[8], sel),
        mux(a[9], b[9], sel),
        mux(a[10], b[10], sel),
        mux(a[11], b[11], sel),
        mux(a[12], b[12], sel),
        mux(a[13], b[13], sel),
        mux(a[14], b[14], sel),
        mux(a[15], b[15], sel),
    ]
}
#[rustfmt::skip]
pub fn or8way(a: [bool; 8]) -> bool {
    or(a[0],or(a[1],or(a[2], or(a[3], or(a[4], or(a[5], or(a[6], a[7])))))))
}

pub fn mux4way16(a: Word, b: Word, c: Word, d: Word, sel: [bool; 2]) -> Word {
    mux16(mux16(a, b, sel[0]), mux16(c, d, sel[0]), sel[1])
}

pub fn mux8way16(
    a: Word,
    b: Word,
    c: Word,
    d: Word,
    e: Word,
    f: Word,
    g: Word,
    h: Word,
    sel: [bool; 3],
) -> Word {
    mux16(
        mux4way16(a, b, c, d, [sel[0], sel[1]]),
        mux4way16(e, f, g, h, [sel[0], sel[1]]),
        sel[2],
    )
}

pub fn dmux4way(input: bool, sel: [bool; 2]) -> [bool; 4] {
    [
        and(input, and(not(sel[0]), not(sel[1]))),
        and(input, and(not(sel[0]), sel[1])),
        and(input, and(sel[0], not(sel[1]))),
        and(input, and(sel[0], sel[1])),
    ]
}

pub fn dmux8way(input: bool, sel: [bool; 3]) -> [bool; 8] {
    [
        and(input, and(not(sel[0]), and(not(sel[1]), not(sel[2])))),
        and(input, and(sel[0], and(not(sel[1]), not(sel[2])))),
        and(input, and(not(sel[0]), and(sel[1], not(sel[2])))),
        and(input, and(sel[0], and(sel[1], not(sel[2])))),
        and(input, and(not(sel[0]), and(not(sel[1]), sel[2]))),
        and(input, and(sel[0], and(not(sel[1]), sel[2]))),
        and(input, and(not(sel[0]), and(sel[1], sel[2]))),
        and(input, and(sel[0], and(sel[1], sel[2]))),
    ]
}
