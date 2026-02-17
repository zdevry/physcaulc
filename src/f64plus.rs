use FloatPlus::{Scalar, Vector};

#[derive(Clone, Debug)]
pub enum FloatPlus {
    Scalar(f64),
    Vector(Vec<f64>),
}

fn apply_unary_op<F>(val: &FloatPlus, op: F) -> FloatPlus
where
    F: Fn(f64) -> f64,
{
    match val {
        Scalar(x) => Scalar(op(*x)),
        Vector(vx) => Vector(vx.into_iter().map(|x| op(*x)).collect()),
    }
}

fn apply_binary_op<F>(lhs: &FloatPlus, rhs: &FloatPlus, op: F) -> Result<FloatPlus, String>
where
    F: Fn(f64, f64) -> f64,
{
    match (lhs, rhs) {
        (Scalar(l), Scalar(r)) => Ok(Scalar(op(*l, *r))),
        (Vector(vl), Scalar(r)) => Ok(Vector(vl.into_iter().map(|l| op(*l, *r)).collect())),
        (Scalar(l), Vector(vr)) => Ok(Vector(vr.into_iter().map(|r| op(*l, *r)).collect())),
        (Vector(vl), Vector(vr)) => {
            if vl.len() != vr.len() {
                return Err("Lengths don't match".to_string());
            }

            Ok(Vector(
                vl.into_iter().zip(vr).map(|(l, r)| op(*l, *r)).collect(),
            ))
        }
    }
}

fn unchecked_apply_binary_op<F>(lhs: &FloatPlus, rhs: &FloatPlus, op: F) -> FloatPlus
where
    F: Fn(f64, f64) -> f64,
{
    match (lhs, rhs) {
        (Scalar(l), Scalar(r)) => Scalar(op(*l, *r)),
        (Vector(vl), Scalar(r)) => Vector(vl.into_iter().map(|l| op(*l, *r)).collect()),
        (Scalar(l), Vector(vr)) => Vector(vr.into_iter().map(|r| op(*l, *r)).collect()),
        (Vector(vl), Vector(vr)) => {
            Vector(vl.into_iter().zip(vr).map(|(l, r)| op(*l, *r)).collect())
        }
    }
}

impl FloatPlus {
    pub const ZERO: FloatPlus = FloatPlus::Scalar(0.);

    pub fn negative(&self) -> FloatPlus {
        apply_unary_op(self, std::ops::Neg::neg)
    }

    pub fn reciprocal(&self) -> FloatPlus {
        apply_unary_op(self, |x| 1. / x)
    }

    pub fn square(&self) -> FloatPlus {
        apply_unary_op(self, |x| x * x)
    }

    pub fn unchecked_add(&self, other: &Self) -> FloatPlus {
        unchecked_apply_binary_op(self, other, std::ops::Add::add)
    }

    pub fn unchecked_sub(&self, other: &Self) -> FloatPlus {
        unchecked_apply_binary_op(self, other, std::ops::Sub::sub)
    }
    pub fn unchecked_mul(&self, other: &Self) -> FloatPlus {
        unchecked_apply_binary_op(self, other, std::ops::Mul::mul)
    }

    pub fn unchecked_div(&self, other: &Self) -> FloatPlus {
        unchecked_apply_binary_op(self, other, std::ops::Div::div)
    }

    pub fn add(&self, other: &Self) -> Result<FloatPlus, String> {
        apply_binary_op(self, other, std::ops::Add::add)
    }

    pub fn sub(&self, other: &Self) -> Result<FloatPlus, String> {
        apply_binary_op(self, other, std::ops::Sub::sub)
    }

    pub fn mul(&self, other: &Self) -> Result<FloatPlus, String> {
        apply_binary_op(self, other, std::ops::Mul::mul)
    }

    pub fn div(&self, other: &Self) -> Result<FloatPlus, String> {
        apply_binary_op(self, other, std::ops::Div::div)
    }
}
