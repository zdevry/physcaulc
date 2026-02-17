use super::{Quantity, Rational, SIDimension, Value};
use crate::f64plus::FloatPlus;
use std::collections::HashMap;

fn apply_binary_op<F, G, H>(
    lhs: &Quantity,
    rhs: &Quantity,
    checked_op: F,
    unchecked_dop: G,
    dim_op: H,
) -> Result<Quantity, String>
where
    F: Fn(&FloatPlus, &FloatPlus) -> Result<FloatPlus, String>,
    G: Fn(&FloatPlus, &FloatPlus, &FloatPlus, &FloatPlus) -> FloatPlus,
    H: Fn(SIDimension, SIDimension) -> Result<SIDimension, String>,
{
    let dim = dim_op(lhs.dim, rhs.dim)?;
    let value = checked_op(&lhs.value, &rhs.value)?;

    let mut derivatives = HashMap::<String, FloatPlus>::new();

    for (var, lhs_drv) in &lhs.derivatives {
        let rhs_drv = rhs.derivatives.get(var).unwrap_or(&FloatPlus::ZERO);
        derivatives.insert(
            var.clone(),
            unchecked_dop(&lhs.value, lhs_drv, &rhs.value, rhs_drv),
        );
    }

    for (var, rhs_drv) in &rhs.derivatives {
        if lhs.derivatives.contains_key(var) {
            continue;
        }

        derivatives.insert(
            var.clone(),
            unchecked_dop(&lhs.value, &FloatPlus::ZERO, &rhs.value, rhs_drv),
        );
    }

    Ok(Quantity {
        value,
        derivatives,
        dim,
    })
}

fn apply_value_binary_op<F, G>(
    lhs: &Value,
    rhs: &Value,
    rational_op: F,
    quantity_op: G,
) -> Result<Value, String>
where
    F: Fn(Rational, Rational) -> Option<Rational>,
    G: Fn(&Quantity, &Quantity) -> Result<Quantity, String>,
{
    match (lhs, rhs) {
        (Value::Rational(l), Value::Rational(r)) => match rational_op(*l, *r) {
            Some(result) => return Ok(Value::Rational(result)),
            _ => (),
        },
        _ => (),
    }

    let ql = lhs.as_quantity_cloned();
    let qr = rhs.as_quantity_cloned();

    let result = quantity_op(&ql, &qr)?;
    Ok(Value::Quantity(result))
}

impl Quantity {
    pub fn add(&self, other: &Self) -> Result<Self, String> {
        apply_binary_op(
            self,
            other,
            FloatPlus::add,
            |_, dl, _, dr| dl.unchecked_add(dr),
            |l, r| {
                if l != r {
                    Err("Dimensions don't match".to_string())
                } else {
                    Ok(l)
                }
            },
        )
    }

    pub fn sub(&self, other: &Self) -> Result<Self, String> {
        apply_binary_op(
            self,
            other,
            FloatPlus::sub,
            |_, dl, _, dr| dl.unchecked_sub(dr),
            |l, r| {
                if l != r {
                    Err("Dimensions don't match".to_string())
                } else {
                    Ok(l)
                }
            },
        )
    }

    pub fn mul(&self, other: &Self) -> Result<Self, String> {
        apply_binary_op(
            self,
            other,
            FloatPlus::mul,
            |l, dl, r, dr| dl.unchecked_mul(r).unchecked_add(&l.unchecked_mul(dr)),
            |l, r| Ok(super::mul_dims(l, r)),
        )
    }

    pub fn div(&self, other: &Self) -> Result<Self, String> {
        apply_binary_op(
            self,
            other,
            FloatPlus::div,
            |l, dl, r, dr| {
                dl.unchecked_mul(r)
                    .unchecked_sub(&l.unchecked_mul(dr))
                    .unchecked_div(&r.square())
            },
            |l, r| Ok(super::mul_dims(l, super::recip_dims(&r))),
        )
    }
}

impl Value {
    pub fn as_quantity_cloned(&self) -> Quantity {
        match self {
            Self::Quantity(q) => q.clone(),
            Self::Rational(r) => Quantity {
                value: FloatPlus::Scalar(r.numerator as f64 / r.denominator as f64),
                derivatives: HashMap::new(),
                dim: super::DIMLESS,
            },
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self, String> {
        apply_value_binary_op(self, other, Rational::checked_add, Quantity::add)
    }

    pub fn sub(&self, other: &Self) -> Result<Self, String> {
        apply_value_binary_op(self, other, Rational::checked_sub, Quantity::sub)
    }

    pub fn mul(&self, other: &Self) -> Result<Self, String> {
        apply_value_binary_op(self, other, Rational::checked_mul, Quantity::mul)
    }

    pub fn div(&self, other: &Self) -> Result<Self, String> {
        apply_value_binary_op(self, other, Rational::checked_div, Quantity::div)
    }
}
