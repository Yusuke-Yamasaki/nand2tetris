use super::basic_logic::and;
use super::basic_logic::xor;

pub fn half_adder(a: bool, b: bool) -> [bool; 2] {
    [
        xor(a, b), // sum
        and(a, b), // carry
    ]
}
