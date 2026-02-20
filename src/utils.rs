use crate::value::SIDimension;

pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn format_lengths_unequal_msg(m: usize, n: usize) -> String {
    format!("Vector lengths not equal ({m} != {n})")
}

pub fn format_units_unequal_msg(_: &SIDimension, _: &SIDimension) -> String {
    "Incompatible units".into()
}

pub fn format_dimless_function_msg(funcname: &str) -> String {
    format!("{funcname} function cannot accept value with units")
}

pub fn format_unitless_index_msg(_: SIDimension) -> String {
    "Unitless index".into()
}

pub fn format_unitless_base_msg(_: SIDimension) -> String {
    "Unitless base".into()
}
