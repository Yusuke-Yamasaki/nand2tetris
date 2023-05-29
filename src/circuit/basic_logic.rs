pub type Word = [bool; 16];

pub fn nand(a: bool, b: bool) -> bool {
    !(a & b)
}

pub fn not(a: bool) -> bool {
    nand(a, a)
}

pub fn and(a: bool, b: bool) -> bool {
    not(nand(a, b))
}

pub fn or(a: bool, b: bool) -> bool {
    nand(nand(a, a), nand(b, b))
}

pub fn xor(a: bool, b: bool) -> bool {
    nand(nand(not(a), b), nand(a, not(b)))
}

pub fn not16(a: Word) -> Word {
    [
        not(a[0]),
        not(a[1]),
        not(a[2]),
        not(a[3]),
        not(a[4]),
        not(a[5]),
        not(a[6]),
        not(a[7]),
        not(a[8]),
        not(a[9]),
        not(a[10]),
        not(a[11]),
        not(a[12]),
        not(a[13]),
        not(a[14]),
        not(a[15]),
    ]
}

pub fn and16(a: Word, b: Word) -> Word {
    [
        and(a[0], b[0]),
        and(a[1], b[1]),
        and(a[2], b[2]),
        and(a[3], b[3]),
        and(a[4], b[4]),
        and(a[5], b[5]),
        and(a[6], b[6]),
        and(a[7], b[7]),
        and(a[8], b[8]),
        and(a[9], b[9]),
        and(a[10], b[10]),
        and(a[11], b[11]),
        and(a[12], b[12]),
        and(a[13], b[13]),
        and(a[14], b[14]),
        and(a[15], b[15]),
    ]
}

pub fn or16(a: Word, b: Word) -> Word {
    [
        or(a[0], b[0]),
        or(a[1], b[1]),
        or(a[2], b[2]),
        or(a[3], b[3]),
        or(a[4], b[4]),
        or(a[5], b[5]),
        or(a[6], b[6]),
        or(a[7], b[7]),
        or(a[8], b[8]),
        or(a[9], b[9]),
        or(a[10], b[10]),
        or(a[11], b[11]),
        or(a[12], b[12]),
        or(a[13], b[13]),
        or(a[14], b[14]),
        or(a[15], b[15]),
    ]
}
