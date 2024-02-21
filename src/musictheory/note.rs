// Author: Ben Lovy <ben@deciduously.com>
// License: MIT

use core::fmt;
use std::io;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

use crate::musictheory::scale::Scale;

use super::char_strs;
use super::interval::Interval;
use super::semitone::Semitone;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NoteLetter {
    C = 0,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl Default for NoteLetter {
    fn default() -> Self {
        NoteLetter::C
    }
}

impl FromStr for NoteLetter {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Ok(NoteLetter::A),
            "B" => Ok(NoteLetter::B),
            "C" => Ok(NoteLetter::C),
            "D" => Ok(NoteLetter::D),
            "E" => Ok(NoteLetter::E),
            "F" => Ok(NoteLetter::F),
            "G" => Ok(NoteLetter::G),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} is not a valid note", s),
            )),
        }
    }
}

impl NoteLetter {
    pub fn inc(self) -> Self {
        use NoteLetter::*;
        match self {
            C => D,
            D => E,
            E => F,
            F => G,
            G => A,
            A => B,
            B => C,
        }
    }

    pub fn interval_from_c(self) -> Interval {
        use Interval::Unison;
        Scale::default()
            .get_intervals()
            .iter()
            .take(self as usize)
            .fold(Unison, |acc, i| acc + *i)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Accidental {
    Flat,
    Sharp,
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Accidental::*;
        let acc_str = match self {
            Flat => "b",
            Sharp => "#"
        };
        write!(f, "{}", acc_str)
    }
}

impl FromStr for Accidental {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "b" => Ok(Accidental::Flat),
            "#" => Ok(Accidental::Sharp),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} is not a valid accidental", s),
            ))
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Note {
    pub accidental: Option<Accidental>,
    pub letter: NoteLetter,
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let acc_str = if let Some(a) = self.accidental {
            format!("{}", a)
        } else {
            "".to_string()
        };
        write!(f, "{:?}{}", self.letter, acc_str)
    }
}

impl FromStr for Note {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char_strs = char_strs(s);
        let mut char_strs = char_strs.iter();
        // note will be first
        if let Some(letter) = char_strs.next() {
            let letter = NoteLetter::from_str(letter)?;
            if let Some(accidental) = char_strs.next() {
                //check if it's valid
                let accidental = Accidental::from_str(accidental)?;
                return Ok(Self {
                    letter,
                    accidental: Some(accidental),
                });
            } else {
                return Ok(Self {
                    letter,
                    accidental: None,
                });
            }
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} is not a valid note", s),
        ))
    }
}

impl From<Interval> for Note {
    fn from(i: Interval) -> Self {
        use Interval::*;
        let mut offset = Unison;

        let scale = Scale::Chromatic.get_intervals();
        scale.iter().take(i as usize).for_each(|i| offset += *i);
        Note::default() + offset
    }
}

impl Add<Interval> for Note {
    type Output = Self;

    fn add(self, rhs: Interval) -> Self {
        let semitones = Semitone::from(rhs);
        let mut ret = self;
        for _ in 0..i8::from(semitones) {
            ret.inc();
        }
        ret
    }
}

impl AddAssign<Interval> for Note {
    fn add_assign(&mut self, rhs: Interval) {
        *self = *self + rhs;
    }
}

impl Note {
    pub fn interval_from_c(self) -> Interval {
        use Accidental::*;
        let ret = self.letter.interval_from_c();
        if let Some(acc) = self.accidental {
            match acc {
                Flat => return Interval::from(Semitone::from(i8::from(Semitone::from(ret)) - 1)),
                Sharp => return ret + Interval::Min2
            }
        };
        ret
    }
    pub fn get_offset(self, other: Self) -> Interval {
        let self_interval_from_c = self.interval_from_c();
        let other_interval_from_c = other.interval_from_c();
        self_interval_from_c - other_interval_from_c
    }
    fn inc(&mut self) {
        use Accidental::*;
        use NoteLetter::*;

        if let Some(acc) = self.accidental {
            self.accidental = None;
            match acc {
                Sharp => {
                    self.letter = self.letter.inc();
                }
                Flat => {}
            }
        } else {
            if self.letter == B || self.letter == E {
                self.letter = self.letter.inc();
            } else {
                self.accidental = Some(Sharp);
            }
        }
    }
}