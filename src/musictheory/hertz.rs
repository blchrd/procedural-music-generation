// Author: Ben Lovy <ben@deciduously.com>
// License: MIT

use std::ops::{MulAssign, Sub};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Hertz(pub f64);

impl Hertz {
    #[must_use]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }
}

impl Sub for Hertz {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl From<Hertz> for f64 {
    fn from(h: Hertz) -> Self {
        h.0
    }
}

impl From<f64> for Hertz {
    fn from(f: f64) -> Self {
        Self(f)
    }
}

impl MulAssign<f64> for Hertz {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}