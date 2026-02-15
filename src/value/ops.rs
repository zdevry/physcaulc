use super::{Dimension, Quantity, Value, mul_dims, recip_dims};

impl Quantity {
    pub fn add(&self, other: &Self) -> Result<Self, String> {
        if self.dim != other.dim {
            return Err("Dimensions don't match".to_string());
        }

        let mut result_derivatives = self.derivatives.clone();
        for (var, drv) in &other.derivatives {
            match result_derivatives.get_mut(var) {
                Some(rd) => *rd = rd.add(drv)?,
                None => {
                    result_derivatives.insert(var.clone(), drv.clone());
                }
            }
        }

        Ok(Quantity {
            value: self.value.add(&other.value)?,
            derivatives: result_derivatives,
            dim: self.dim,
        })
    }

    pub fn sub(&self, other: &Self) -> Result<Self, String> {
        if self.dim != other.dim {
            return Err("Dimensions don't match".to_string());
        }

        let mut result_derivatives = self.derivatives.clone();
        for (var, drv) in &other.derivatives {
            match result_derivatives.get_mut(var) {
                Some(rd) => *rd = rd.sub(drv)?,
                None => {
                    result_derivatives.insert(var.clone(), drv.negative());
                }
            }
        }

        Ok(Quantity {
            value: self.value.sub(&other.value)?,
            derivatives: result_derivatives,
            dim: self.dim,
        })
    }

    pub fn mul(&self, other: &Self) -> Result<Self, String> {
        let mut result_derivatives = self.derivatives.clone();

        for (_, rd) in result_derivatives.iter_mut() {
            *rd = rd.mul(&other.value)?;
        }

        for (var, drv) in &other.derivatives {
            match result_derivatives.get_mut(var) {
                Some(rd) => *rd = rd.add(&drv.mul(&self.value)?)?,
                None => {
                    result_derivatives.insert(var.clone(), drv.mul(&self.value)?);
                }
            }
        }

        Ok(Quantity {
            value: self.value.mul(&other.value)?,
            derivatives: result_derivatives,
            dim: mul_dims(self.dim, other.dim),
        })
    }

    pub fn div(&self, other: &Self) -> Result<Self, String> {
        let mut result_derivatives = self.derivatives.clone();

        for (_, rd) in result_derivatives.iter_mut() {
            *rd = rd.mul(&other.value)?;
        }

        for (var, drv) in &other.derivatives {
            match result_derivatives.get_mut(var) {
                Some(rd) => *rd = rd.sub(&drv.mul(&self.value)?)?,
                None => {
                    result_derivatives.insert(var.clone(), drv.mul(&self.value)?.negative());
                }
            }
        }

        for (_, rd) in result_derivatives.iter_mut() {
            *rd = rd.div(&other.value.square())?;
        }

        Ok(Quantity {
            value: self.value.div(&other.value)?,
            derivatives: result_derivatives,
            dim: mul_dims(self.dim, recip_dims(&other.dim)),
        })
    }
}
