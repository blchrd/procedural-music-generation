use core::fmt;
use std::str::FromStr;

use crate::musictheory::chord::ChordInversion;

use super::{chord::{Chord, ChordType}, key::Key, piano_key::PianoKey, scale::Scale};

#[derive(Debug, Clone, PartialEq)]
pub struct ChordProgression {
    pub chords: Vec<Chord>
}

impl Default for ChordProgression {
    fn default() -> Self {
        Self::from_scale_and_str(
            Scale::default(), 
            PianoKey::from_str("C4").unwrap(),
            "I-V-vi-IV"
        )
    }
}

impl ChordProgression {
    pub fn from_scale_and_str(scale: Scale, base_note: PianoKey, s: &str) -> Self {
        use ChordType::{MajorTriad, MinorTriad, DiminishedTriad};
        use ChordInversion::Root;

        let notes = Key::new(scale, base_note, 1).all_keys();
        let mut ret = Self{chords: Vec::<Chord>::new()};

        s.split('-').into_iter().for_each(|chord_str| {
            //1. check if chord degree is minor or major (with lowercase)
            let chord_type: ChordType;
            if chord_str.to_uppercase() == chord_str {
                chord_type = MajorTriad;
            } else {
                if chord_str == "vii" {
                    chord_type = DiminishedTriad;
                } else {
                    chord_type = MinorTriad;
                }
            }

            //2. convert roman number
            let degree = roman::from(&chord_str.to_uppercase()).unwrap() as usize;

            //3. get the base note of the chord
            let base_note = notes[degree - 1];

            //4. construct the chord (for now, we just use major and minor triad (or diminished for the seventh degrees))
            ret.chords.push(Chord{base_note, chord_type, inversion: Root});
        });
        ret
    }
}

impl fmt::Display for ChordProgression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::from("[ ");
        self.chords.iter().for_each(|c| ret.push_str(&format!("{} ", *c)));
        ret.push_str("]");

        write!(f, "{}", ret)
    }
}