use core::fmt;
use std::{io, str::FromStr};

use super::{interval::Interval, piano_key::PianoKey, pitch::Pitch};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChordType {
    MajorTriad,
    MajorSixth,
    DominantSeventh,
    AugmentedTriad,
    AugmentedSeventh,
    MinorTriad,
    MinorSixth,
    MinorSeventh,
    MinorMajorSeventh,
    DiminishedTriad,
    DiminishedSeventh,
    HalfDiminishedSeventh,
    PowerDiad,
    PowerTriad,
}

impl ChordType {
    pub fn get_intervals(self) -> Vec<Interval> {
        use Interval::*;
        match self {
            Self::MajorTriad => vec![Maj3, Perfect5],
            Self::MajorSixth => vec![Maj3, Perfect5, Maj6],
            Self::DominantSeventh => vec![Maj3, Perfect5, Min7],
            Self::AugmentedTriad => vec![Maj3, Min6],
            Self::AugmentedSeventh => vec![Maj3, Min6, Min7],
            Self::MinorTriad => vec![Min3, Perfect5],
            Self::MinorSixth => vec![Min3, Perfect5, Maj6],
            Self::MinorSeventh => vec![Min3, Perfect5, Min7],
            Self::MinorMajorSeventh => vec![Min3, Perfect5, Maj7],
            Self::DiminishedTriad => vec![Min3, Tritone],
            Self::DiminishedSeventh => vec![Min3, Tritone, Maj6],
            Self::HalfDiminishedSeventh => vec![Min3, Tritone, Min7],
            Self::PowerDiad => vec![Perfect5],
            Self::PowerTriad => vec![Perfect5, Octave],
        }
    }
}

impl Default for ChordType {
    fn default() -> Self {
        Self::MajorTriad
    }
}

impl FromStr for ChordType {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ChordType::*;
        match s.to_uppercase().as_str() {
            "MAJ" => Ok(MajorTriad),
            "MAJ6" => Ok(MajorSixth),
            "DOM7" => Ok(DominantSeventh),
            "AUG" => Ok(AugmentedTriad),
            "AUG7" => Ok(AugmentedSeventh),
            "MIN" => Ok(MinorTriad),
            "MIN6" => Ok(MinorSixth),
            "MIN7" => Ok(MinorSeventh),
            "MINMAJ7" => Ok(MinorMajorSeventh),
            "DIM" => Ok(DiminishedTriad),
            "DIM7" => Ok(DiminishedSeventh),
            "HDIM7" => Ok(HalfDiminishedSeventh),
            "POW2" => Ok(PowerDiad),
            "POW3" => Ok(PowerTriad),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown chord")),
        }
    }
}

impl fmt::Display for ChordType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: &str;
        match self {
            Self::MajorTriad => s = "maj",
            Self::MajorSixth => s = "maj6",
            Self::DominantSeventh => s = "dom7",
            Self::AugmentedTriad => s = "aug",
            Self::AugmentedSeventh => s = "aug7",
            Self::MinorTriad => s = "min",
            Self::MinorSixth => s = "min6",
            Self::MinorSeventh => s = "min7",
            Self::MinorMajorSeventh => s = "minmaj7",
            Self::DiminishedTriad => s = "dim",
            Self::DiminishedSeventh => s = "dim7",
            Self::HalfDiminishedSeventh => s = "hdim7",
            Self::PowerDiad => s = "pow2",
            Self::PowerTriad =>s = "pow3" ,
        }
        
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChordInversion {
    Root = 0,
    First,
    Second,
    Third
}

impl Default for ChordInversion {
    fn default() -> Self {
        Self::Root
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Chord {
    pub base_note: PianoKey,
    pub chord_type: ChordType,
    pub inversion: ChordInversion,
}

impl Default for Chord {
    fn default() -> Self {
        Self {
            base_note: PianoKey::default(),
            chord_type: ChordType::default(),
            inversion: ChordInversion::default(),
        }
    }
}

impl Chord {
    pub fn new(chord_type: ChordType, base_note: PianoKey, inversion: ChordInversion) -> Self {
        Self {
            base_note,
            chord_type,
            inversion,
        }
    }

    pub fn get_keys_string(self) -> String {
        let notes = self.get_keys();
        let mut ret = String::from("| ");

        notes.iter().for_each(|n| ret.push_str(&format!("{} ", *n)));
        ret.push_str("|");
        
        format!("{}", ret)
    }

    pub fn get_keys(self) -> Vec<PianoKey> {
        let mut ret = Vec::new();
        ret.push(self.base_note);
        self.chord_type.get_intervals().iter().for_each(|i| {
            ret.push(self.base_note + *i);
        });

        ret.rotate_left(self.inversion as usize);
        // Check if the notes' octave is correct
        if ret[0].octave > self.base_note.octave {
            let diff = ret[0].octave - self.base_note.octave;
            ret.iter_mut().for_each(|sp| sp.octave -= diff);
        }
        for i in 1..ret.len() {
            if ret[i].octave < ret[i - 1].octave {
                ret[i].octave = ret[i - 1].octave;
            } else if Pitch::from(ret[i]) < Pitch::from(ret[i - 1]) {
                ret[i].octave = ret[i - 1].octave + 1;
            }
        }

        ret
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.base_note, self.chord_type)
    }
}