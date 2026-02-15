use crate::{f64plus::FloatPlus, rational::Rational};
use std::collections::HashMap;

mod ops;

// Canonical definitive order:
// time, length, mass, current, temperature, quantity of substance, luminous intensity
pub type Dimension = [Rational; 7];

pub const DIMLESS: Dimension = [Rational::ZERO; 7];

pub fn mul_dims(a: Dimension, b: Dimension) -> Dimension {
    let mut result = [Rational::ZERO; 7];
    for i in 0..7 {
        result[i] = a[i].add(b[i])
    }
    result
}

pub fn recip_dims(d: &Dimension) -> Dimension {
    let mut result = [Rational::ZERO; 7];
    for i in 0..7 {
        result[i] = d[i].negative();
    }
    result
}

#[derive(Clone, Debug)]
pub struct Quantity {
    pub value: FloatPlus,
    pub derivatives: HashMap<String, FloatPlus>,
    pub dim: Dimension,
}

#[derive(Clone, Debug)]
pub enum Value {
    Rational(Rational),
    Quantity(Quantity),
}
