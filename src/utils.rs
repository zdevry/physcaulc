use crate::value::SIDimension;

pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn format_lengths_unequal_msg(m: usize, n: usize) -> String {
    format!("Vector lengths not equal ({m} != {n})")
}

pub fn format_units_unequal_msg(a: SIDimension, b: SIDimension) -> String {
    "Incompatible units".into()
}
