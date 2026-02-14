use super::{DIMLESS, Dimension, Quantity, Value};
use crate::rational::Rational;

impl Quantity {
    pub fn negative(&self) -> Self {
        let mut result_derivatives = self.derivatives.clone();
        for (_, d) in result_derivatives.iter_mut() {
            *d = -*d;
        }
        Quantity {
            value: -self.value,
            derivatives: result_derivatives,
        }
    }

    pub fn reciprocal(&self) -> Self {
        let mut result_derivatives = self.derivatives.clone();
        for (_, d) in result_derivatives.iter_mut() {
            *d = -*d / (self.value * self.value);
        }
        Quantity {
            value: 1. / self.value,
            derivatives: result_derivatives,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut result_derivatives = self.derivatives.clone();

        for (var, d) in &other.derivatives {
            match result_derivatives.get_mut(var) {
                Some(rd) => *rd = *rd + d,
                None => {
                    result_derivatives.insert(var.clone(), *d);
                }
            }
        }

        Quantity {
            value: self.value + other.value,
            derivatives: result_derivatives,
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        self.add(&other.negative())
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut result_derivatives = self.derivatives.clone();

        for (_, d) in result_derivatives.iter_mut() {
            *d = *d * other.value;
        }

        for (var, d) in &other.derivatives {
            match result_derivatives.get_mut(var) {
                Some(rd) => {
                    *rd = *rd + self.value * d;
                }
                None => {
                    result_derivatives.insert(var.clone(), self.value * d);
                }
            }
        }

        Quantity {
            value: self.value * other.value,
            derivatives: result_derivatives,
        }
    }

    pub fn div(&self, other: &Self) -> Self {
        self.mul(&other.reciprocal())
    }
}

impl Value {
    pub fn Derationalise(&self) -> (Vec<Quantity>, Dimension) {
        match self {
            Value::Vector(v, d) => (v.clone(), *d),
            Value::Rational(r) => {
                let q = Quantity {
                    value: r.numerator as f64 / r.denominator as f64,
                    derivatives: std::collections::HashMap::new(),
                };
                (vec![q], DIMLESS)
            }
        }
    }

    pub fn negative(&self) -> Self {
        match self {
            Value::Rational(r) => Value::Rational(r.negative()),
            Value::Vector(vq, d) => {
                Value::Vector(vq.into_iter().map(|q| q.negative()).collect(), *d)
            }
        }
    }

    pub fn reciprocal(&self) -> Self {
        match self {
            Value::Rational(r) => Value::Rational(r.reciprocal()),
            Value::Vector(vq, d) => Value::Vector(
                vq.into_iter().map(|q| q.reciprocal()).collect(),
                super::recip_dims(d),
            ),
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self, String> {
        match (self, other) {
            (Value::Rational(rl), Value::Rational(rr)) => return Ok(Value::Rational(rl.add(*rr))),
            _ => (),
        }

        let (vql, dl) = self.Derationalise();
        let (vqr, dr) = other.Derationalise();

        if dl != dr {
            return Err("Dimensions don't match".to_string());
        }

        let add_results = (&vql)
            .into_iter()
            .zip(&vqr)
            .map(|(ql, qr)| ql.add(qr))
            .collect::<Vec<Quantity>>();

        Ok(Value::Vector(add_results, dl))
    }

    pub fn sub(&self, other: &Self) -> Result<Self, String> {
        self.add(&other.negative())
    }

    pub fn mul(&self, other: &Self) -> Self {
        match (self, other) {
            (Value::Rational(rl), Value::Rational(rr)) => return Value::Rational(rl.mul(*rr)),
            _ => (),
        }

        let (vql, dl) = self.Derationalise();
        let (vqr, dr) = other.Derationalise();

        let mul_results = (&vql)
            .into_iter()
            .zip(&vqr)
            .map(|(ql, qr)| ql.mul(qr))
            .collect::<Vec<Quantity>>();

        Value::Vector(mul_results, super::mul_dims(dl, dr))
    }

    pub fn div(&self, other: &Self) -> Self {
        self.mul(&other.reciprocal())
    }
}
