use crate::{f64plus::FloatPlus, rational::Rational};
use std::collections::HashMap;

mod complex;
mod func;
mod ops;
mod quantity;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SIDimension {
    pub time: Rational,
    pub length: Rational,
    pub mass: Rational,
    pub current: Rational,
    pub temperature: Rational,
    pub quantity: Rational,
    pub luminous: Rational,
}

pub const DIMLESS: SIDimension = SIDimension {
    time: Rational::ZERO,
    length: Rational::ZERO,
    mass: Rational::ZERO,
    current: Rational::ZERO,
    temperature: Rational::ZERO,
    quantity: Rational::ZERO,
    luminous: Rational::ZERO,
};

pub fn mul_dims(a: SIDimension, b: SIDimension) -> SIDimension {
    SIDimension {
        time: a.time.add(b.time),
        length: a.length.add(b.length),
        mass: a.mass.add(b.mass),
        current: a.current.add(b.current),
        temperature: a.temperature.add(b.temperature),
        quantity: a.quantity.add(b.quantity),
        luminous: a.luminous.add(b.luminous),
    }
}

pub fn recip_dims(d: &SIDimension) -> SIDimension {
    SIDimension {
        time: d.time.negative(),
        length: d.length.negative(),
        mass: d.mass.negative(),
        current: d.current.negative(),
        temperature: d.temperature.negative(),
        quantity: d.quantity.negative(),
        luminous: d.luminous.negative(),
    }
}

pub fn pow_dims(d: &SIDimension, index: Rational) -> SIDimension {
    SIDimension {
        time: d.time.mul(index),
        length: d.length.mul(index),
        mass: d.mass.mul(index),
        current: d.current.mul(index),
        temperature: d.temperature.mul(index),
        quantity: d.quantity.mul(index),
        luminous: d.luminous.mul(index),
    }
}

#[derive(Clone, Debug)]
pub struct Quantity {
    pub value: FloatPlus,
    pub derivatives: HashMap<String, FloatPlus>,
    pub dim: SIDimension,
}

#[derive(Clone, Debug)]
pub struct Complex {
    pub real: FloatPlus,
    pub imag: FloatPlus,
    pub dim: SIDimension,
}

#[derive(Clone, Debug)]
pub enum Value {
    Rational(Rational),
    Quantity(Quantity),
    Complex(Complex),
}

impl Value {
    pub fn dimless(&self) -> bool {
        match self {
            Self::Rational(_) => true,
            Self::Quantity(q) => q.dim == DIMLESS,
            Self::Complex(c) => c.dim == DIMLESS,
        }
    }

    pub fn try_promote_quantity(&self) -> Option<Quantity> {
        match self {
            Self::Rational(r) => Some(Quantity::from_rational(*r)),
            Self::Quantity(q) => Some(q.clone()),
            Self::Complex(_) => None,
        }
    }

    pub fn promote_to_complex(&self) -> Complex {
        match self {
            Self::Rational(r) => Complex::from_rational(*r),
            Self::Quantity(q) => Complex::from_quantity(&q),
            Self::Complex(c) => c.clone(),
        }
    }
}
