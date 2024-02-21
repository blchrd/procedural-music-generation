// Author: Ben Lovy <ben@deciduously.com>
// License: MIT

use std::ops::AddAssign;

use rodio::source::SineWave;

use super::{cent::Cent, hertz::Hertz, interval::Interval, piano_key::PianoKey, semitone::Semitone};

pub const STANDARD_PITCH: Hertz = Hertz(440.0);
pub const MIDDLE_C: Hertz = Hertz(261.626);
pub const C_ZERO: Hertz = Hertz(16.352);

#[derive(Debug, Clone, Copy, PartialOrd)]
pub struct Pitch(pub Hertz);

impl Pitch {
    pub fn new(frequency: Hertz) -> Self {
        Self(frequency)
    }
}

impl Default for Pitch {
    fn default() -> Self {
        Self(STANDARD_PITCH)
    }
}

impl From<Pitch> for f64 {
    fn from(p: Pitch) -> Self {
        p.0.into()
    }
}

impl From<Pitch> for SineWave {
    fn from(p: Pitch) -> Self {
        SineWave::new(f64::from(p) as f32)
    }
}

impl From<PianoKey> for Pitch {
    fn from(sp: PianoKey) -> Self {
        use Interval::*;
        let mut ret = Pitch::new(C_ZERO);

        for _ in 0..sp.octave {
            ret += Octave;
        }

        ret += sp.note.letter.interval_from_c();
        ret
    }
}

impl AddAssign<Cent> for Pitch {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn add_assign(&mut self, rhs: Cent) {
        self.0 *= 2.0f64.powf((rhs / Cent::from(Interval::Octave)).into())
    }
}

impl AddAssign<Semitone> for Pitch {
    fn add_assign(&mut self, rhs: Semitone) {
        *self += Cent::from(rhs)
    }
}

impl AddAssign<Interval> for Pitch {
    fn add_assign(&mut self, rhs: Interval) {
        *self += Cent::from(rhs)
    }
}

impl PartialEq for Pitch {
    fn eq(&self, other: &Self) -> bool {
        let tolerance = Hertz(0.1);
        let difference = (self.0 - other.0).abs();
        difference < tolerance
    }
}