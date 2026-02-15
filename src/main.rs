mod f64plus;
mod rational;
mod utils;
mod value;

use crate::value::{DIMLESS, Quantity};
use f64plus::FloatPlus;
use std::collections::HashMap;

fn main() {
    let x = Quantity {
        value: FloatPlus::Vector(vec![1., 2., 3.]),
        derivatives: HashMap::from([("s".to_string(), FloatPlus::Vector(vec![1., 2., 3.]))]),
        dim: DIMLESS,
    };

    let y = Quantity {
        value: FloatPlus::Scalar(2.5),
        derivatives: HashMap::from([("s".to_string(), FloatPlus::Scalar(1.5))]),
        dim: DIMLESS,
    };

    dbg!(y.div(&x));
}
