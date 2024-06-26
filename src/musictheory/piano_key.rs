// Author: Ben Lovy <ben@deciduously.com>
// License: MIT

use core::fmt;
use std::{io, ops::Add, str::FromStr};

use crate::musictheory::note::{Accidental, NoteLetter};

use super::{char_strs, interval::Interval, note::Note, semitone::Semitone};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct PianoKey {
    pub note: Note,
    pub octave: u8,
}

impl fmt::Display for PianoKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.note, self.octave)
    }
}

impl Add<Interval> for PianoKey {
    type Output = Self;

    fn add(self, rhs: Interval) -> Self::Output {
        let semitones = Semitone::from(rhs);
        let mut ret = self;
        for _ in 0..i8::from(semitones) {
            ret.inc()
        }
        ret
    }
}

impl FromStr for PianoKey {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(octave) = char_strs(s).last() {
            if let Ok(octave) = octave.parse::<u8>() {
                let note = Note::from_str(&s[0..s.len() -1])?;
                if octave <= Self::max_octave() {
                    Ok(Self { note, octave })
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} is too high!", octave),
                    ))
                }
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} is too high for this keyboard", octave),
                ))
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} is not a valid note", s),
            ))
        }
    }
}

impl PianoKey {
    pub fn new(s: &str) -> Result<Self, io::Error> {
        Self::from_str(s)
    }

    pub fn max_octave() -> u8 {
        8
    }

    pub fn inc(&mut self) {
        use Accidental::*;
        use NoteLetter::*;

        if let Some(acc) = self.note.accidental {
            self.note.accidental = None;
            match acc {
                Sharp => {
                    self.note.letter = self.note.letter.inc();
                }
                Flat => {}
            }
        } else {
            if self.note.letter == B || self.note.letter == E {
                if self.note.letter == B {
                    self.octave += 1;
                }
                self.note.letter = self.note.letter.inc(); 
            } else {
                self.note.accidental = Some(Sharp);
            }
        }
    }

    pub fn get_distance(&self, piano_key: PianoKey) -> i32 {
        let from_note: PianoKey;
        let to_note: PianoKey;

        if self.octave != piano_key.octave {
            if self.octave > piano_key.octave {
                from_note = piano_key;
                to_note = self.clone();
            } else {
                from_note = self.clone();
                to_note = piano_key;
            }
        } else {
            let self_distance_from_c = self.note.interval_from_c() as i32;
            let other_distance_from_c = piano_key.note.interval_from_c() as i32;
            let distance_between_note = self_distance_from_c - other_distance_from_c;
    
            if distance_between_note > 0 {
                from_note = piano_key;
                to_note = self.clone();
            } else {
                from_note = self.clone();
                to_note = piano_key;
            }
        }

        let mut distance = 0;
        let mut temp_note = from_note.clone();
        while temp_note != to_note {
            temp_note.inc();
            distance += 1;
        }

        return distance;
    }
}