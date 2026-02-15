use FloatPlus::{Scalar, Vector};

#[derive(Clone, Debug)]
pub enum FloatPlus {
    Scalar(f64),
    Vector(Vec<f64>),
}

fn applyUnaryOp<F>(val: &FloatPlus, op: F) -> FloatPlus
where
    F: Fn(f64) -> f64,
{
    match val {
        Scalar(x) => Scalar(op(*x)),
        Vector(vx) => Vector(vx.into_iter().map(|x| op(*x)).collect()),
    }
}

fn applyBinaryOp<F>(lhs: &FloatPlus, rhs: &FloatPlus, op: F) -> Result<FloatPlus, String>
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

impl FloatPlus {
    pub fn negative(&self) -> FloatPlus {
        applyUnaryOp(self, |x| -x)
    }

    pub fn reciprocal(&self) -> FloatPlus {
        applyUnaryOp(self, |x| 1. / x)
    }

    pub fn square(&self) -> FloatPlus {
        applyUnaryOp(self, |x| x * x)
    }

    pub fn add(&self, other: &Self) -> Result<FloatPlus, String> {
        applyBinaryOp(self, other, |a, b| a + b)
    }

    pub fn sub(&self, other: &Self) -> Result<FloatPlus, String> {
        applyBinaryOp(self, other, |a, b| a - b)
    }

    pub fn mul(&self, other: &Self) -> Result<FloatPlus, String> {
        applyBinaryOp(self, other, |a, b| a * b)
    }

    pub fn div(&self, other: &Self) -> Result<FloatPlus, String> {
        applyBinaryOp(self, other, |a, b| a / b)
    }
}
