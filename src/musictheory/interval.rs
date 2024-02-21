// Author: Ben Lovy <ben@deciduously.com>
// License: MIT

use std::ops::{Add, AddAssign, Sub};

use super::semitone::Semitone;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Interval {
    Unison = 0,
    Min2,
    Maj2,
    Min3,
    Maj3,
    Perfect4,
    Tritone,
    Perfect5,
    Min6,
    Maj6,
    Min7,
    Maj7,
    Octave,
}

impl From<Interval> for i8 {
    fn from(i: Interval) -> Self {
        Semitone::from(i).into()
    }
}

impl From<Semitone> for Interval {
    fn from(s: Semitone) -> Self {
        use Interval::*;
        let int_semitones = i8::from(s);
        match int_semitones {
            0 => Unison,
            1 => Min2,
            2 => Maj2,
            3 => Min3,
            4 => Maj3,
            5 => Perfect4,
            6 => Tritone,
            7 => Perfect5,
            8 => Min6,
            9 => Maj6,
            10 => Min7,
            11 => Maj7,
            12 | _ => Interval::from(Semitone(int_semitones % Octave as i8)),
        }
    }
}

impl From<Interval> for Semitone {
    fn from(i: Interval) -> Self {
        Semitone(i as i8)
    }
}

impl Add for Interval {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Interval::from(Semitone(
            i8::from(self) + i8::from(rhs) % Interval::Octave as i8,
        ))
    }
}

impl Sub for Interval {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut delta = i8::from(self) - i8::from(rhs);
        if delta < 0 {
            delta += Interval::Octave as i8;
        };
        Interval::from(Semitone(delta))
    }
}

impl AddAssign for Interval {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}