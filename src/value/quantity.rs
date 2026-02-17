use super::{DIMLESS, Quantity, SIDimension};
use crate::{f64plus::FloatPlus, utils};
use std::collections::HashMap;

fn apply_dimless_func<F, D>(q: &Quantity, f: F, df: F, name: &str) -> Result<Quantity, String>
where
    F: Fn(f64) -> f64,
{
    if q.dim != DIMLESS {
        return Err(format!("{name} function cannot accept values with units"));
    }

    let value = q.value.apply_func(&f);
    let mut derivatives = HashMap::new();

    for (var, drv) in &q.derivatives {
        derivatives.insert(var.clone(), q.value.apply_func(&df).mul(&drv));
    }

    Ok(Quantity {
        value,
        derivatives,
        dim: DIMLESS,
    })
}

fn apply_binary_op<F, G, H>(
    lhs: &Quantity,
    rhs: &Quantity,
    op: F,
    dop: G,
    dim_op: H,
) -> Result<Quantity, String>
where
    F: Fn(&FloatPlus, &FloatPlus) -> FloatPlus,
    G: Fn(&FloatPlus, &FloatPlus, &FloatPlus, &FloatPlus) -> FloatPlus,
    H: Fn(SIDimension, SIDimension) -> Result<SIDimension, String>,
{
    let dim = dim_op(lhs.dim, rhs.dim)?;
    match lhs.value.strictly_compatible(&rhs.value) {
        Some((m, n)) => return Err(utils::format_lengths_unequal_msg(m, n)),
        None => (),
    }
    let value = op(&lhs.value, &rhs.value);

    let mut derivatives = HashMap::<String, FloatPlus>::new();

    for (var, lhs_drv) in &lhs.derivatives {
        let rhs_drv = rhs.derivatives.get(var).unwrap_or(&FloatPlus::ZERO);
        derivatives.insert(var.clone(), dop(&lhs.value, lhs_drv, &rhs.value, rhs_drv));
    }

    for (var, rhs_drv) in &rhs.derivatives {
        if lhs.derivatives.contains_key(var) {
            continue;
        }

        derivatives.insert(
            var.clone(),
            dop(&lhs.value, &FloatPlus::ZERO, &rhs.value, rhs_drv),
        );
    }

    Ok(Quantity {
        value,
        derivatives,
        dim,
    })
}

impl Quantity {
    pub fn negative(&self) -> Self {
        let mut derivatives = HashMap::new();

        for (var, drv) in &self.derivatives {
            derivatives.insert(var.clone(), drv.negative());
        }

        Self {
            value: self.value.negative(),
            derivatives,
            dim: self.dim,
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self, String> {
        apply_binary_op(
            self,
            other,
            FloatPlus::add,
            |_, dl, _, dr| dl.add(dr),
            |l, r| {
                if l != r {
                    Err(utils::format_units_unequal_msg(l, r))
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
            |_, dl, _, dr| dl.sub(dr),
            |l, r| {
                if l != r {
                    Err(utils::format_units_unequal_msg(l, r))
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
            |l, dl, r, dr| dl.mul(r).add(&l.mul(dr)),
            |l, r| Ok(super::mul_dims(l, r)),
        )
    }

    pub fn div(&self, other: &Self) -> Result<Self, String> {
        apply_binary_op(
            self,
            other,
            FloatPlus::div,
            |l, dl, r, dr| dl.mul(r).sub(&l.mul(dr)).div(&r.square()),
            |l, r| Ok(super::mul_dims(l, super::recip_dims(&r))),
        )
    }
}
