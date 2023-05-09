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
