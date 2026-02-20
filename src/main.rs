pub mod f64plus;
pub mod rational;
pub mod utils;
pub mod value;

use crate::{
    f64plus::FloatPlus,
    rational::Rational,
    value::{Complex, Quantity, SIDimension, Value},
};
use std::collections::HashMap;

fn main() {
    let q = Value::Quantity(Quantity {
        value: FloatPlus::Vector(vec![-3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0]),
        derivatives: HashMap::from([("x".into(), 1.0.into())]),
        dim: SIDimension {
            time: Rational::ZERO,
            length: Rational::new(3, 1),
            mass: Rational::ZERO,
            current: Rational::ZERO,
            temperature: Rational::ZERO,
            quantity: Rational::ZERO,
            luminous: Rational::ZERO,
        },
    });

    let e = Value::Rational(Rational::new(-2, 5));

    dbg!(q.pow(&e).unwrap().mul(&q));
}
