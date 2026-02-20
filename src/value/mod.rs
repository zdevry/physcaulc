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

impl SIDimension {
    pub const DIMLESS: Self = Self {
        time: Rational::ZERO,
        length: Rational::ZERO,
        mass: Rational::ZERO,
        current: Rational::ZERO,
        temperature: Rational::ZERO,
        quantity: Rational::ZERO,
        luminous: Rational::ZERO,
    };

    pub fn mul(&self, other: &Self) -> Self {
        Self {
            time: self.time.add(other.time),
            length: self.length.add(other.length),
            mass: self.mass.add(other.mass),
            current: self.current.add(other.current),
            temperature: self.temperature.add(other.temperature),
            quantity: self.quantity.add(other.quantity),
            luminous: self.luminous.add(other.luminous),
        }
    }

    pub fn reciprocal(&self) -> Self {
        Self {
            time: self.time.negative(),
            length: self.length.negative(),
            mass: self.mass.negative(),
            current: self.current.negative(),
            temperature: self.temperature.negative(),
            quantity: self.quantity.negative(),
            luminous: self.luminous.negative(),
        }
    }

    pub fn pow(&self, e: Rational) -> Self {
        Self {
            time: self.time.mul(e),
            length: self.length.mul(e),
            mass: self.mass.mul(e),
            current: self.current.mul(e),
            temperature: self.temperature.mul(e),
            quantity: self.quantity.mul(e),
            luminous: self.luminous.mul(e),
        }
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

impl From<Rational> for Value {
    fn from(value: Rational) -> Self {
        Self::Rational(value)
    }
}

impl From<Quantity> for Value {
    fn from(value: Quantity) -> Self {
        Self::Quantity(value)
    }
}

impl From<Complex> for Value {
    fn from(value: Complex) -> Self {
        Self::Complex(value)
    }
}

impl Value {
    pub fn dimless(&self) -> bool {
        match self {
            Self::Rational(_) => true,
            Self::Quantity(q) => q.dim == SIDimension::DIMLESS,
            Self::Complex(c) => c.dim == SIDimension::DIMLESS,
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
