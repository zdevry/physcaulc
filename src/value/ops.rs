use super::{Complex, Quantity, Rational, SIDimension, Value, ValueError};
use crate::f64plus::FloatPlus;
use std::collections::HashMap;

fn apply_value_binary_op<F, G, H>(
    lhs: &Value,
    rhs: &Value,
    rational_op: F,
    quantity_op: G,
    complex_op: H,
) -> Result<Value, ValueError>
where
    F: Fn(Rational, Rational) -> Option<Rational>,
    G: Fn(&Quantity, &Quantity) -> Result<Quantity, ValueError>,
    H: Fn(&Complex, &Complex) -> Result<Complex, ValueError>,
{
    match (lhs, rhs) {
        (&Value::Rational(l), &Value::Rational(r)) => match rational_op(l, r) {
            Some(result) => return Ok(result.into()),
            _ => (),
        },
        _ => (),
    }

    match lhs.try_promote_quantity().zip(rhs.try_promote_quantity()) {
        Some((ql, qr)) => return Ok(quantity_op(&ql, &qr)?.into()),
        None => (),
    }

    let cl = lhs.promote_to_complex();
    let cr = rhs.promote_to_complex();

    Ok(complex_op(&cl, &cr)?.into())
}

impl Value {
    pub fn add(&self, other: &Self) -> Result<Self, ValueError> {
        apply_value_binary_op(
            self,
            other,
            Rational::checked_add,
            Quantity::add,
            Complex::add,
        )
    }

    pub fn sub(&self, other: &Self) -> Result<Self, ValueError> {
        apply_value_binary_op(
            self,
            other,
            Rational::checked_sub,
            Quantity::sub,
            Complex::sub,
        )
    }

    pub fn mul(&self, other: &Self) -> Result<Self, ValueError> {
        apply_value_binary_op(
            self,
            other,
            Rational::checked_mul,
            Quantity::mul,
            Complex::mul,
        )
    }

    pub fn div(&self, other: &Self) -> Result<Self, ValueError> {
        apply_value_binary_op(
            self,
            other,
            Rational::checked_div,
            Quantity::div,
            Complex::div,
        )
    }

    // cases of pow function
    // 1. Rational^Rational -> Rational if index integer, =>4 otherwise
    // 2. Rational^Quantity -> =>5
    // 3. Rational^Complex -> =>9
    // 4. Quantity^Rational -> Quantity if base>0 OR base<0 and denominator odd, Complex otherwise; units
    // 5. Quantity^Quantity -> Quantity if base>0, Complex otherwise; unitless
    // 6. Quantity^Complex -> =>9
    // 7. Complex^Rational -> Complex; units
    // 8. Complex^Quantity -> =>9
    // 9. Complex^Complex -> Complex; unitless
    pub fn pow(&self, other: &Self) -> Result<Self, ValueError> {
        match other {
            &Self::Rational(e) => match self {
                &Self::Rational(b) => {
                    if e.is_integral() {
                        Ok(pow_ri(b, e.numerator)?.into())
                    } else {
                        Ok(pow_qr(&Quantity::from_rational(b), e))
                    }
                }
                Self::Quantity(b) => Ok(pow_qr(b, e)),
                Self::Complex(b) => Ok(pow_cr(b, e).into()),
            },
            Self::Quantity(e) => match self.try_promote_quantity() {
                Some(b) => pow_qq(&b, e),
                None => Ok(pow_cc(&self.promote_to_complex(), &Complex::from_quantity(e))?.into()),
            },
            Self::Complex(e) => Ok(pow_cc(&self.promote_to_complex(), e)?.into()),
        }
    }
}

fn pow_ri(base: Rational, index: i32) -> Result<Rational, ValueError> {
    let abs_idx = index.unsigned_abs();
    let result = Rational::new(base.numerator.pow(abs_idx), base.denominator.pow(abs_idx));

    if index < 0 {
        if base.is_zero() {
            Err(ValueError::DivisionByZero)
        } else {
            Ok(result.reciprocal())
        }
    } else {
        Ok(result)
    }
}

fn pow_qr(base: &Quantity, index: Rational) -> Value {
    // Negative raised to odd denominators are treated differently
    if base.value.any(|x| x < 0.) && index.denominator % 2 == 0 {
        return pow_cr(&Complex::from_quantity(base), index).into();
    }

    let result_value = pow_fpr(&base.value, index);

    let index_minus_one = index.sub(Rational::ONE);
    let mut result_derivatives = HashMap::<String, FloatPlus>::new();
    for (var, drv) in &base.derivatives {
        result_derivatives.insert(
            var.clone(),
            FloatPlus::Scalar(index.into())
                .mul(&pow_fpr(&base.value, index_minus_one))
                .mul(&drv),
        );
    }

    Quantity {
        value: result_value,
        derivatives: result_derivatives,
        dim: base.dim.pow(index),
    }
    .into()
}

fn pow_fpr(base: &FloatPlus, index: Rational) -> FloatPlus {
    let sign = base.apply_func(f64::signum);
    let abs_result_value = base.apply_func(|x| x.abs().powf(index.into()));
    if index.numerator % 2 == 0 {
        abs_result_value
    } else {
        abs_result_value.mul(&sign)
    }
}

fn pow_cr(base: &Complex, index: Rational) -> Complex {
    let mag = base.mag_si_units();
    let arg = base.arg();

    let result_mag = mag.apply_func(|x| x.powf(index.into()));
    let result_arg = arg.apply_func(|x| x * index.to_float());
    let phase_real = result_arg.apply_func(f64::cos);
    let phase_imag = result_arg.apply_func(f64::sin);

    Complex {
        real: result_mag.mul(&phase_real),
        imag: result_mag.mul(&phase_imag),
        dim: base.dim.pow(index),
    }
}

fn pow_cc(base: &Complex, index: &Complex) -> Result<Complex, ValueError> {
    match base.strictly_compatible(index) {
        Some((m, n)) => return Err(ValueError::UnequalVectorLength(m, n)),
        None => (),
    }

    if index.dim != SIDimension::DIMLESS {
        return Err(ValueError::NotDimensionlessOperand(index.dim));
    }
    if base.dim != SIDimension::DIMLESS {
        return Err(ValueError::UnsupportedBaseDimension(base.dim));
    }

    // z^w = exp(w ln z)
    Ok(base.natlog().unwrap().unchecked_mul(&index).exp().unwrap())
}

fn pow_qq(base: &Quantity, index: &Quantity) -> Result<Value, ValueError> {
    if base.value.any(|x| x < 0.) {
        return Ok(pow_cc(
            &Complex::from_quantity(base),
            &Complex::from_quantity(index),
        )?
        .into());
    }

    match base.value.strictly_compatible(&index.value) {
        Some((m, n)) => return Err(ValueError::UnequalVectorLength(m, n)),
        None => (),
    }

    if index.dim != SIDimension::DIMLESS {
        return Err(ValueError::NotDimensionlessOperand(index.dim));
    }
    if base.dim != SIDimension::DIMLESS {
        return Err(ValueError::UnsupportedBaseDimension(base.dim));
    }

    let result_value = base.value.apply_binary_func(&index.value, f64::powf);
    let mut result_derivatives = HashMap::<String, FloatPlus>::new();

    for (var, base_drv) in &base.derivatives {
        let index_drv = index.derivatives.get(var).unwrap_or(&FloatPlus::ZERO);
        result_derivatives.insert(
            var.clone(),
            derivative_pow_qq(
                &base.value,
                base_drv,
                &index.value,
                index_drv,
                &result_value,
            ),
        );
    }

    for (var, index_drv) in &index.derivatives {
        if base.derivatives.contains_key(var) {
            continue;
        }

        result_derivatives.insert(
            var.clone(),
            derivative_pow_qq(
                &base.value,
                &FloatPlus::ZERO,
                &index.value,
                index_drv,
                &result_value,
            ),
        );
    }

    Ok(Quantity {
        value: result_value,
        derivatives: result_derivatives,
        dim: SIDimension::DIMLESS,
    }
    .into())
}

fn derivative_pow_qq(
    b: &FloatPlus,
    db: &FloatPlus,
    e: &FloatPlus,
    de: &FloatPlus,
    pow: &FloatPlus,
) -> FloatPlus {
    de.mul(&b.apply_func(f64::ln))
        .add(&e.mul(db).div(b))
        .mul(&pow)
}
