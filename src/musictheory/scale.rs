// Author: Ben Lovy <ben@deciduously.com>
// License: MIT
// Modified by Thomas Blanchard (blchrd)

use core::fmt;
use std::{io, str::FromStr};

use super::{interval::Interval, mode::{Mode, PentatonicMode}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scale {
    Chromatic,
    Diatonic(Mode),
    Pentatonic(PentatonicMode),
    Tetratonic,
}

impl Default for Scale {
    fn default() -> Self {
        Scale::Diatonic(Mode::Ionian)
    }
}

impl Scale {
    pub fn get_intervals(self) -> Vec<Interval> {
        use Interval::*;
        use Scale::*;
        match self {
            Chromatic => [Min2]
                .iter()
                .cycle()
                .take(ScaleLength::Dodecatonic as usize)
                .copied()
                .collect::<Vec<Interval>>(),
            Pentatonic(mode) => Mode::base_interval(ScaleLength::Pentatonic)
                .iter()
                .cycle()
                .skip(mode as usize)
                .take(ScaleLength::Pentatonic as usize)
                .copied()
                .collect::<Vec<Interval>>(),
            Diatonic(mode) => Mode::base_interval(ScaleLength::Heptatonic)
                .iter()
                .cycle()
                .skip(mode as usize)
                .take(ScaleLength::Heptatonic as usize)
                .copied()
                .collect::<Vec<Interval>>(),
            Tetratonic => vec![Min2, Maj2, Maj3]
        }
    }
}

impl FromStr for Scale {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Mode::*;
        use PentatonicMode::*;
        use Scale::*;
        match s.to_uppercase().as_str() {
            "IONIAN" | "MAJOR" => Ok(Diatonic(Ionian)),
            "DORIAN" => Ok(Diatonic(Dorian)),
            "PHRYGIAN" => Ok(Diatonic(Phrygian)),
            "LYDIAN" => Ok(Diatonic(Lydian)),
            "MIXOLYDIAN" => Ok(Diatonic(Mixolydian)),
            "AEOLIAN" | "MINOR" => Ok(Diatonic(Aeolian)),
            "LOCRIAN" => Ok(Diatonic(Locrian)),
            "PENTATONIC" | "PENTAMAJOR" => Ok(Pentatonic(Major)),
            "PENTASUSPENDED" => Ok(Pentatonic(Suspended)),
            "PENTABLUESMAJOR" => Ok(Pentatonic(BluesMajor)),
            "PENTABLUESMINOR" => Ok(Pentatonic(BluesMinor)),
            "PENTAMINOR" => Ok(Pentatonic(Minor)),
            "CHROMATIC" => Ok(Chromatic),
            "TETRATONIC" => Ok(Tetratonic),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown scale")),
        }
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Scale::*;
        let s = match self {
            Chromatic | Tetratonic => format!("{:?} scale", self).to_lowercase(),
            Pentatonic(mode) => format!("pentatonic {:?} mode", mode),
            Diatonic(mode) => {
                use Mode::*;
                match mode {
                    Aeolian => "minor scale".into(),
                    Ionian => "major scale".into(),
                    _ => format!("{:?} mode", mode)
                }
            }
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScaleLength {
    Tetratonic = 4,
    Pentatonic = 5,
    Heptatonic = 7,
    Dodecatonic = 12,
}