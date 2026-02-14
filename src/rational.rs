use crate::utils::gcd;

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl Rational {
    pub const ZERO: Self = Rational {
        numerator: 0,
        denominator: 1,
    };

    pub fn new(numer: i32, denom: u32) -> Self {
        let common = gcd(numer.abs() as u32, denom);

        Self {
            numerator: numer / (common as i32),
            denominator: denom / common,
        }
    }

    pub fn negative(self) -> Self {
        Self {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }

    pub fn reciprocal(self) -> Self {
        let sign = self.numerator.signum();
        Self {
            numerator: sign * self.denominator as i32,
            denominator: self.numerator.abs() as u32,
        }
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

    pub fn sub(self, other: Self) -> Self {
        self.add(other.negative())
    }

    pub fn mul(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }

    pub fn div(self: Self, other: Self) -> Self {
        self.mul(other.reciprocal())
    }
}
