use std::str::FromStr;

use rand::{rngs::SmallRng, seq::IteratorRandom};

use crate::musictheory::{piano_key::PianoKey, scale::Scale};

pub fn get_random_scale(mut seed: &mut SmallRng) -> Scale {  
    let scales = vec![
        Scale::from_str("IONIAN").unwrap(),
        Scale::from_str("DORIAN").unwrap(),
        Scale::from_str("PHRYGIAN").unwrap(),
        Scale::from_str("LYDIAN").unwrap(),
        Scale::from_str("MIXOLYDIAN").unwrap(),
        Scale::from_str("AEOLIAN").unwrap(),
        Scale::from_str("LOCRIAN").unwrap(),
        Scale::from_str("PENTATONIC").unwrap(),
        Scale::from_str("PENTASUSPENDED").unwrap(),
        Scale::from_str("PENTABLUESMAJOR").unwrap(),
        Scale::from_str("PENTABLUESMINOR").unwrap(),
        Scale::from_str("PENTAMINOR").unwrap(),
        Scale::from_str("CHROMATIC").unwrap(),
        Scale::from_str("TETRATONIC").unwrap(),
    ];

    scales.iter().choose(&mut seed).unwrap().clone()
}

pub fn get_random_base_note(mut seed: &mut SmallRng) -> PianoKey {
    let notes = vec![
        PianoKey::from_str("A4").unwrap(),
        PianoKey::from_str("A#4").unwrap(),
        PianoKey::from_str("B4").unwrap(),
        PianoKey::from_str("C4").unwrap(),
        PianoKey::from_str("C#4").unwrap(),
        PianoKey::from_str("D4").unwrap(),
        PianoKey::from_str("D#4").unwrap(),
        PianoKey::from_str("E4").unwrap(),
        PianoKey::from_str("F4").unwrap(),
        PianoKey::from_str("F#4").unwrap(),
        PianoKey::from_str("G4").unwrap(),
        PianoKey::from_str("G#4").unwrap(),
    ];

    notes.iter().choose(&mut seed).unwrap().clone()
}