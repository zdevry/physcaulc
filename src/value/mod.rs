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
    pub fn try_promote_quantity(&self) -> Option<Quantity> {
        match self {
            Self::Rational(r) => Some(Quantity {
                value: FloatPlus::Scalar(r.numerator as f64 / r.denominator as f64),
                derivatives: HashMap::new(),
                dim: DIMLESS,
            }),
            Self::Quantity(q) => Some(q.clone()),
            Self::Complex(_) => None,
        }
    }

    pub fn promote_to_complex(&self) -> Complex {
        match self {
            Self::Rational(r) => Complex {
                real: FloatPlus::Scalar(r.numerator as f64 / r.denominator as f64),
                imag: FloatPlus::Scalar(0.),
                dim: DIMLESS,
            },
            Self::Quantity(q) => Complex {
                real: q.value.clone(),
                imag: FloatPlus::Scalar(0.),
                dim: q.dim,
            },
            Self::Complex(c) => c.clone(),
        }
    }
}
