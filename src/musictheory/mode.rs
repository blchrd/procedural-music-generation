// Author: Ben Lovy <ben@deciduously.com>
// License: MIT

use super::{interval::Interval, scale::ScaleLength};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Mode {
    Ionian = 0,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum PentatonicMode {
    Major = 0,
    Suspended,
    BluesMinor,
    BluesMajor,
    Minor,
}

impl Mode {
    pub fn base_interval(sl: ScaleLength) -> Vec<Interval> {
        use ScaleLength::*;
        use Interval::*;
        match sl {
            Pentatonic => vec![Maj2, Maj2, Min3, Maj2, Min3],
            Heptatonic | _ => vec![Maj2, Maj2, Min2, Maj2, Maj2, Maj2, Min2]
        }
    }
}