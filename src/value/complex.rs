use super::Complex;
use crate::utils;

impl Complex {
    pub fn negative(&self) -> Self {
        Self {
            real: self.real.negative(),
            imag: self.imag.negative(),
            dim: self.dim,
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self, String> {
        if self.dim != other.dim {
            return Err(utils::format_units_unequal_msg(self.dim, other.dim));
        }
        match self.real.strictly_compatible(&other.real) {
            Some((m, n)) => return Err(utils::format_lengths_unequal_msg(m, n)),
            None => (),
        }

        Ok(Self {
            real: self.real.add(&other.real),
            imag: self.imag.add(&other.imag),
            dim: self.dim,
        })
    }

    pub fn sub(&self, other: &Self) -> Result<Self, String> {
        if self.dim != other.dim {
            return Err(utils::format_units_unequal_msg(self.dim, other.dim));
        }
        match self.real.strictly_compatible(&other.real) {
            Some((m, n)) => return Err(utils::format_lengths_unequal_msg(m, n)),
            None => (),
        }

        Ok(Self {
            real: self.real.sub(&other.real),
            imag: self.imag.sub(&other.imag),
            dim: self.dim,
        })
    }

    pub fn mul(&self, other: &Self) -> Result<Self, String> {
        match self.real.strictly_compatible(&other.real) {
            Some((m, n)) => return Err(utils::format_lengths_unequal_msg(m, n)),
            None => (),
        }

        Ok(Self {
            real: self.real.mul(&other.real).sub(&self.imag.mul(&other.imag)),
            imag: self.imag.mul(&other.real).add(&self.real.mul(&other.imag)),
            dim: super::mul_dims(self.dim, other.dim),
        })
    }

    pub fn div(&self, other: &Self) -> Result<Self, String> {
        match self.real.strictly_compatible(&other.real) {
            Some((m, n)) => return Err(utils::format_lengths_unequal_msg(m, n)),
            None => (),
        }

        let denom_factors = other.real.square().add(&other.imag.square());

        Ok(Self {
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
            dim: super::mul_dims(self.dim, super::recip_dims(&other.dim)),
        })
    }
}
