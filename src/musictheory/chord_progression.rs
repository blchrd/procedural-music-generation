use core::fmt;
use std::str::FromStr;

use crate::musictheory::chord::ChordInversion;

use super::{chord::{Chord, ChordType}, key::Key, piano_key::PianoKey, scale::Scale};

#[derive(Debug, Clone, PartialEq)]
pub struct ChordProgression {
    pub progression: String,
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
        use ChordInversion::Root;

        let notes = Key::new(scale, base_note, 1).all_keys();
        let mut ret = Self{progression: s.to_string(), chords: Vec::<Chord>::new()};

        s.split('-').into_iter().for_each(|chord_str| {
            //1. check chord type (upper or lowercase and last characters)
            let mut chord_type_str: String;
            let mut chord_degree: String = chord_str.to_string();
            let last_char = chord_str.chars().nth_back(0).unwrap();
            let second_last_char = chord_str.chars().nth_back(1);

            if chord_str.to_uppercase() == chord_str {
                chord_type_str = String::from("maj");
            } else {
                if chord_str == "vii" {
                    chord_type_str = String::from("min");
                } else {
                    chord_type_str = String::from("min");
                }
            }

            if last_char == '°' {
                chord_type_str = String::from("dim");
                chord_degree = chord_degree.replace('°', "");
            } else if last_char == 'ø' {
                chord_type_str = String::from("hdim7");
                chord_degree = chord_degree.replace('ø', "")
            } else if last_char == '↑' {
                chord_type_str = String::from("aug");
                chord_degree = chord_degree.replace('↑', "")
            } else if last_char == 'p' {
                chord_type_str = String::from("pow");
                chord_degree = chord_degree.replace('p', "")
            } else if last_char.is_digit(10) {
                if second_last_char.is_some() {
                    if second_last_char.unwrap() == '°' {
                        chord_type_str = String::from("dim");
                        chord_degree = chord_degree.replace('°', "");
                    } else if second_last_char.unwrap() == 'ø' {
                        chord_type_str = String::from("hdim7");
                        chord_degree = chord_degree.replace('ø', "")
                    } else if second_last_char.unwrap() == '↑' {
                        chord_type_str = String::from("aug");
                        chord_degree = chord_degree.replace('↑', "")
                    } else if second_last_char.unwrap() == 'p' {
                        chord_type_str = String::from("pow");
                        chord_degree = chord_degree.replace('p', "")
                    }
                }
                
                chord_type_str.push_str(&last_char.to_string());
                chord_degree = chord_degree.replace(last_char, "");
            }

            //2. convert roman number
            let degree = roman::from(&chord_degree.to_uppercase()).unwrap() as usize;

            //3. get the base note of the chord
            let base_note = notes[degree - 1];

            //4. construct the chord (for now, we just use major and minor triad (or diminished for the seventh degrees))
            ret.chords.push(Chord{base_note, chord_type: ChordType::from_str(&chord_type_str).unwrap(), inversion: Root, intervals: None});
        });
        ret
    }
}

impl fmt::Display for ChordProgression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret;
        ret = String::from("[ ");
        self.chords.iter().for_each(|c| ret.push_str(&format!("{} ", *c)));
        ret.push_str("]");
        
        ret = format!("{} ({})", self.progression.clone(), ret);

        write!(f, "{}", ret)
    }
}