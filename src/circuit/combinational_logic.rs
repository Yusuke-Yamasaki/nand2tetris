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

