mod rational;
mod utils;
mod value;

use crate::{
    rational::Rational,
    value::{DIMLESS, Quantity, Value},
};

fn main() {
    let r1 = Value::Vector(
        vec![Quantity {
            value: 0.5,
            derivatives: std::collections::HashMap::new(),
        }],
        [
            Rational::new(1, 1),
            Rational::ZERO,
            Rational::ZERO,
            Rational::ZERO,
            Rational::ZERO,
            Rational::ZERO,
            Rational::ZERO,
        ],
    );
    // let r1 = Value::Rational(Rational::new(1, 2));
    let r2 = Value::Rational(Rational::new(1, 3));

    dbg!(r1.div(&r1));
}
