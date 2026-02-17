mod f64plus;
mod rational;
mod utils;
mod value;

use crate::{
    rational::Rational,
    value::{DIMLESS, Quantity, Value},
};
use f64plus::FloatPlus;
use std::collections::HashMap;

fn main() {
    let p = Value::Rational(Rational::new(32, 657));
    let q = Value::Rational(Rational::new(133, 38));

    dbg!(p.add(&q));
}
