use super::basic_logic::and;
use super::basic_logic::xor;

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
