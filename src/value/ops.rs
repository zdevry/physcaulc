use super::{Complex, Quantity, Rational, Value};

fn apply_value_binary_op<F, G, H>(
    lhs: &Value,
    rhs: &Value,
    rational_op: F,
    quantity_op: G,
    complex_op: H,
) -> Result<Value, String>
where
    F: Fn(Rational, Rational) -> Option<Rational>,
    G: Fn(&Quantity, &Quantity) -> Result<Quantity, String>,
    H: Fn(&Complex, &Complex) -> Result<Complex, String>,
{
    match (lhs, rhs) {
        (Value::Rational(l), Value::Rational(r)) => match rational_op(*l, *r) {
            Some(result) => return Ok(Value::Rational(result)),
            _ => (),
        },
        _ => (),
    }

    match lhs.try_promote_quantity().zip(rhs.try_promote_quantity()) {
        Some((ql, qr)) => return Ok(Value::Quantity(quantity_op(&ql, &qr)?)),
        None => (),
    }

    let cl = lhs.promote_to_complex();
    let cr = rhs.promote_to_complex();

    Ok(Value::Complex(complex_op(&cl, &cr)?))
}

impl Value {
    pub fn add(&self, other: &Self) -> Result<Self, String> {
        apply_value_binary_op(
            self,
            other,
            Rational::checked_add,
            Quantity::add,
            Complex::add,
        )
    }

    pub fn sub(&self, other: &Self) -> Result<Self, String> {
        apply_value_binary_op(
            self,
            other,
            Rational::checked_sub,
            Quantity::sub,
            Complex::sub,
        )
    }

    pub fn mul(&self, other: &Self) -> Result<Self, String> {
        apply_value_binary_op(
            self,
            other,
            Rational::checked_mul,
            Quantity::mul,
            Complex::mul,
        )
    }

    pub fn div(&self, other: &Self) -> Result<Self, String> {
        apply_value_binary_op(
            self,
            other,
            Rational::checked_div,
            Quantity::div,
            Complex::div,
        )
    }
}
