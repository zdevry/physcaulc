use super::{Complex, Quantity, Value, ValueError};

fn apply_real_func<F, G>(val: &Value, qfunc: F, cfunc: G) -> Result<Value, ValueError>
where
    F: Fn(&Quantity) -> Result<Quantity, ValueError>,
    G: Fn(&Complex) -> Result<Complex, ValueError>,
{
    match val.try_promote_quantity() {
        Some(q) => Ok(qfunc(&q)?.into()),
        None => {
            let c = val.promote_to_complex();
            Ok(cfunc(&c)?.into())
        }
    }
}
impl Value {
    pub fn exp(&self) -> Result<Self, ValueError> {
        apply_real_func(self, Quantity::exp, Complex::exp)
    }

    pub fn natlog(&self) -> Result<Self, ValueError> {
        apply_real_func(self, Quantity::natlog, Complex::natlog)
    }

    pub fn cos(&self) -> Result<Self, ValueError> {
        apply_real_func(self, Quantity::cos, Complex::cos)
    }

    pub fn sin(&self) -> Result<Self, ValueError> {
        apply_real_func(self, Quantity::sin, Complex::sin)
    }

    pub fn tan(&self) -> Result<Self, ValueError> {
        apply_real_func(self, Quantity::tan, Complex::tan)
    }
}
