use super::{Complex, Quantity, Value};

fn apply_real_func<F, G>(val: &Value, qfunc: F, cfunc: G) -> Result<Value, String>
where
    F: Fn(&Quantity) -> Result<Quantity, String>,
    G: Fn(&Complex) -> Result<Complex, String>,
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
    pub fn exp(&self) -> Result<Self, String> {
        apply_real_func(self, Quantity::exp, Complex::exp)
    }

    pub fn natlog(&self) -> Result<Self, String> {
        apply_real_func(self, Quantity::natlog, Complex::natlog)
    }

    pub fn cos(&self) -> Result<Self, String> {
        apply_real_func(self, Quantity::cos, Complex::cos)
    }

    pub fn sin(&self) -> Result<Self, String> {
        apply_real_func(self, Quantity::sin, Complex::sin)
    }

    pub fn tan(&self) -> Result<Self, String> {
        apply_real_func(self, Quantity::tan, Complex::tan)
    }
}
