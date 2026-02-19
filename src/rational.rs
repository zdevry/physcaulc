use crate::utils::gcd;

#[derive(Clone, Copy, PartialEq)]
pub struct Rational {
    pub numerator: i32,
    pub denominator: u32,
}

impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.numerator == 0 {
            write!(f, "0")
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl std::fmt::Debug for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl Into<f64> for Rational {
    fn into(self) -> f64 {
        self.to_float()
    }
}

impl Rational {
    pub const ZERO: Self = Rational {
        numerator: 0,
        denominator: 1,
    };

    pub const ONE: Self = Rational {
        numerator: 1,
        denominator: 1,
    };

    pub fn new(numer: i32, denom: u32) -> Self {
        let common = gcd(numer.unsigned_abs(), denom);

        Self {
            numerator: numer / (common as i32),
            denominator: denom / common,
        }
    }

    pub fn is_zero(self) -> bool {
        self.numerator == 0
    }

    pub fn is_integral(self) -> bool {
        self.denominator == 1
    }

    pub fn to_float(self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    pub fn negative(self) -> Self {
        Self {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }

    pub fn checked_negative(self) -> Option<Self> {
        Some(Self {
            numerator: self.numerator.checked_neg()?,
            denominator: self.denominator,
        })
    }

    pub fn reciprocal(self) -> Self {
        let sign = self.numerator.signum();
        Self {
            numerator: sign * self.denominator as i32,
            denominator: self.numerator.abs() as u32,
        }
    }

    pub fn checked_reciprocal(self) -> Option<Self> {
        if self.denominator > i32::MAX as u32 {
            return None;
        }

        let sign = self.numerator.signum();
        Some(Self {
            numerator: sign * self.denominator as i32,
            denominator: self.numerator.unsigned_abs(),
        })
    }

    pub fn add(self, other: Self) -> Self {
        let common = gcd(self.denominator, other.denominator);

        let left_factor = self.denominator / common;
        let right_factor = other.denominator / common;

        Self::new(
            right_factor as i32 * self.numerator + left_factor as i32 * other.numerator,
            left_factor * right_factor * common,
        )
    }

    pub fn checked_add(self, other: Self) -> Option<Self> {
        let common = gcd(self.denominator, other.denominator);

        let left_factor = self.denominator / common;
        let right_factor = other.denominator / common;

        Some(Self::new(
            self.numerator
                .checked_mul(right_factor as i32)?
                .checked_add(other.numerator.checked_mul(left_factor as i32)?)?,
            left_factor.checked_mul(right_factor)?.checked_mul(common)?,
        ))
    }

    pub fn sub(self, other: Self) -> Self {
        self.add(other.negative())
    }

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.checked_add(other.checked_negative()?)
    }

    pub fn mul(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }

    pub fn checked_mul(self, other: Self) -> Option<Self> {
        Some(Self::new(
            self.numerator.checked_mul(other.numerator)?,
            self.denominator.checked_mul(other.denominator)?,
        ))
    }

    pub fn div(self, other: Self) -> Self {
        self.mul(other.reciprocal())
    }

    pub fn checked_div(self, other: Self) -> Option<Self> {
        self.checked_mul(other.checked_reciprocal()?)
    }
}
