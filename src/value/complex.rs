use super::{Complex, Quantity, Rational, SIDimension, ValueError};
use crate::f64plus::FloatPlus;

impl Complex {
    pub fn from_quantity(q: &Quantity) -> Self {
        Complex {
            real: q.value.clone(),
            imag: FloatPlus::Scalar(0.),
            dim: q.dim,
        }
    }

    pub fn from_rational(r: Rational) -> Self {
        Complex {
            real: FloatPlus::Scalar(r.to_float()),
            imag: FloatPlus::Scalar(0.),
            dim: SIDimension::DIMLESS,
        }
    }

    pub fn strictly_compatible(&self, other: &Self) -> Option<(usize, usize)> {
        self.real
            .strictly_compatible(&other.real)
            .or(self.imag.strictly_compatible(&other.imag))
    }

    pub fn negative(&self) -> Self {
        Self {
            real: self.real.negative(),
            imag: self.imag.negative(),
            dim: self.dim,
        }
    }

    pub fn mag_si_units(&self) -> FloatPlus {
        self.real
            .square()
            .add(&self.imag.square())
            .apply_func(f64::sqrt)
    }

    pub fn arg(&self) -> FloatPlus {
        self.imag.apply_binary_func(&self.real, f64::atan2)
    }

    pub fn add(&self, other: &Self) -> Result<Self, ValueError> {
        if self.dim != other.dim {
            return Err(ValueError::UnequalDimensions(self.dim, other.dim));
        }
        match self.strictly_compatible(other) {
            Some((m, n)) => return Err(ValueError::UnequalVectorLength(m, n)),
            None => (),
        }

        Ok(self.unchecked_add(other))
    }

    pub fn unchecked_add(&self, other: &Self) -> Self {
        Self {
            real: self.real.add(&other.real),
            imag: self.imag.add(&other.imag),
            dim: self.dim,
        }
    }

    pub fn sub(&self, other: &Self) -> Result<Self, ValueError> {
        if self.dim != other.dim {
            return Err(ValueError::UnequalDimensions(self.dim, other.dim));
        }
        match self.strictly_compatible(other) {
            Some((m, n)) => return Err(ValueError::UnequalVectorLength(m, n)),
            None => (),
        }

        Ok(self.unchecked_sub(other))
    }

    pub fn unchecked_sub(&self, other: &Self) -> Self {
        Self {
            real: self.real.sub(&other.real),
            imag: self.imag.sub(&other.imag),
            dim: self.dim,
        }
    }

    pub fn mul(&self, other: &Self) -> Result<Self, ValueError> {
        match self.strictly_compatible(other) {
            Some((m, n)) => return Err(ValueError::UnequalVectorLength(m, n)),
            None => (),
        }

        Ok(self.unchecked_mul(other))
    }

    pub fn unchecked_mul(&self, other: &Self) -> Self {
        Self {
            real: self.real.mul(&other.real).sub(&self.imag.mul(&other.imag)),
            imag: self.imag.mul(&other.real).add(&self.real.mul(&other.imag)),
            dim: self.dim.mul(&other.dim),
        }
    }

    pub fn div(&self, other: &Self) -> Result<Self, ValueError> {
        match self.strictly_compatible(other) {
            Some((m, n)) => return Err(ValueError::UnequalVectorLength(m, n)),
            None => (),
        }

        Ok(self.unchecked_div(other))
    }

    pub fn unchecked_div(&self, other: &Self) -> Self {
        let denom_factors = other.mag_si_units().square();

        Self {
            real: self
                .real
                .mul(&other.real)
                .add(&self.imag.mul(&other.imag))
                .div(&denom_factors),
            imag: self
                .imag
                .mul(&other.real)
                .sub(&self.real.mul(&other.imag))
                .div(&denom_factors),
            dim: self.dim.mul(&other.dim.reciprocal()),
        }
    }

    pub fn exp(&self) -> Result<Self, ValueError> {
        if self.dim != SIDimension::DIMLESS {
            return Err(ValueError::NotDimensionlessOperand(self.dim));
        }

        let magnitude = self.real.apply_func(f64::exp);
        let phase_real = self.imag.apply_func(f64::cos);
        let phase_imag = self.imag.apply_func(f64::sin);

        Ok(Self {
            real: magnitude.mul(&phase_real),
            imag: magnitude.mul(&phase_imag),
            dim: SIDimension::DIMLESS,
        })
    }

    pub fn natlog(&self) -> Result<Self, ValueError> {
        if self.dim != SIDimension::DIMLESS {
            return Err(ValueError::NotDimensionlessOperand(self.dim));
        }

        Ok(Self {
            real: self.mag_si_units().apply_func(f64::ln),
            imag: self.arg(),
            dim: SIDimension::DIMLESS,
        })
    }

    pub fn cos(&self) -> Result<Self, ValueError> {
        if self.dim != SIDimension::DIMLESS {
            return Err(ValueError::NotDimensionlessOperand(self.dim));
        }

        Ok(Self {
            real: self
                .real
                .apply_func(f64::cos)
                .mul(&self.imag.apply_func(f64::cosh)),
            imag: self
                .real
                .apply_func(f64::sin)
                .mul(&self.imag.apply_func(f64::sinh))
                .negative(),
            dim: SIDimension::DIMLESS,
        })
    }

    pub fn sin(&self) -> Result<Self, ValueError> {
        if self.dim != SIDimension::DIMLESS {
            return Err(ValueError::NotDimensionlessOperand(self.dim));
        }

        Ok(Self {
            real: self
                .real
                .apply_func(f64::sin)
                .mul(&self.imag.apply_func(f64::cosh)),
            imag: self
                .real
                .apply_func(f64::cos)
                .mul(&self.imag.apply_func(f64::sinh)),
            dim: SIDimension::DIMLESS,
        })
    }

    pub fn tan(&self) -> Result<Self, ValueError> {
        self.sin()?.div(&self.cos()?)
    }
}
