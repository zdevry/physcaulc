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

impl From<f64> for FloatPlus {
    fn from(value: f64) -> Self {
        Self::Scalar(value)
    }
}

impl FloatPlus {
    pub const ZERO: FloatPlus = FloatPlus::Scalar(0.);

    pub fn any<P>(&self, cond: P) -> bool
    where
        P: Fn(f64) -> bool,
    {
        match self {
            Self::Scalar(s) => cond(*s),
            Self::Vector(v) => v.iter().any(|s| cond(*s)),
        }
    }

    pub fn strictly_compatible(&self, other: &Self) -> Option<(usize, usize)> {
        match (self, other) {
            (Self::Vector(vl), Self::Vector(vr)) => {
                if vl.len() != vr.len() {
                    Some((vl.len(), vr.len()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn negative(&self) -> FloatPlus {
        apply_unary_op(self, std::ops::Neg::neg)
    }

    pub fn reciprocal(&self) -> FloatPlus {
        apply_unary_op(self, |x| 1. / x)
    }

    pub fn square(&self) -> FloatPlus {
        apply_unary_op(self, |x| x * x)
    }

    pub fn add(&self, other: &Self) -> FloatPlus {
        unchecked_apply_binary_op(self, other, std::ops::Add::add)
    }

    pub fn sub(&self, other: &Self) -> FloatPlus {
        unchecked_apply_binary_op(self, other, std::ops::Sub::sub)
    }
    pub fn mul(&self, other: &Self) -> FloatPlus {
        unchecked_apply_binary_op(self, other, std::ops::Mul::mul)
    }

    pub fn div(&self, other: &Self) -> FloatPlus {
        unchecked_apply_binary_op(self, other, std::ops::Div::div)
    }

    pub fn apply_func<F>(&self, f: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        apply_unary_op(self, &f)
    }

    pub fn apply_binary_func<F>(&self, other: &Self, f: F) -> Self
    where
        F: Fn(f64, f64) -> f64,
    {
        unchecked_apply_binary_op(self, other, f)
    }
}
