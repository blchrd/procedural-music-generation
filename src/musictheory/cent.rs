// Author: Ben Lovy <ben@deciduously.com>
// License: MIT

use std::ops::Div;
use super::{interval::Interval, semitone::Semitone};

const SEMITONE_CENTS: Cent = Cent(100.0);

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Cent(pub f64);

impl From<f64> for Cent {
    fn from(f: f64) -> Self {
        Cent(f)
    }
}

impl From<Cent> for f64 {
    fn from(c: Cent) -> Self {
        c.0
    }
}

impl From<Semitone> for Cent {
    fn from(s: Semitone) -> Self {
        Cent(i8::from(s) as f64 * f64::from(SEMITONE_CENTS))
    }
}

impl From<Interval> for Cent {
    fn from(i: Interval) -> Self {
        Semitone::from(i).into()
    }
}

impl Div for Cent {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Cent(f64::from(self) / f64::from(rhs))
    }
}