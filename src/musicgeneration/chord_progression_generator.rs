use crate::musictheory::{mode::{Mode, PentatonicMode}, scale::Scale, time_signature::TimeSignature};
use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};

pub fn chord_progression_generation(scale: Scale, _time_signature: TimeSignature, full_random: bool) -> String {
    let mut seed = SmallRng::from_entropy();

    let chords = vec![
        "I","I","i","i","i°","I↑","Ip3",
        "II","II","ii","ii","ii°","II↑","IIp3",
        "III","III","iii","iii","iii°","III↑","IIIp3",
        "IV","IV","iv","iv","IV°","IV↑","IVp3",
        "V","V","v","v","v°","V↑","Vp3",
        "VI","VI","vi","vi","vi°","VI↑","VIp3",
        "VII","VII","vii","vii","vii°","VII↑","VIIp3",
    ];
    let chord_progressions = match scale {
        Scale::Chromatic => vec!["I"],
        Scale::Tetratonic => vec!["I"],
        Scale::Diatonic(mode) => {
            use Mode::*;
            match mode {
                Ionian => vec![
                    "I-IV-V-I",
                    "I-V-vi-IV",
                    "I-vi-IV-V",
                    "vi-VI-I-V",
                    "i-v-iv-i",
                    "I-vi-ii-V",
                    "I-V-vi-iii-IV-I-IV-V",
                    "I-ii-iii-IV-V",
                    "V-IV-I",
                    "ii-V-I",
                ],
                Dorian => vec![
                    "i-IV",
                    "i-IV-V",
                    "i-IV-I",
                    "i-IV-I-i",
                    "i-IV-I-I-IV-i"
                ],
                Phrygian => vec![
                    "i-II",
                    "II-i",
                    "i-II-i",
                    "i-vii",
                    "i-v°",
                    "i-v°-II-i",
                    "i-iv-II-i",
                    "i-VI-II",
                    "i-vii-II-i",
                    "ii-II-III-II",
                ],
                Lydian => vec![
                    "iii-IV°-V",
                    "I-II",
                    "I-II-vii-vi",
                    "I-IV°-ii-V",
                    "I-V-II",
                    "vi-V-I-II",
                ],
                Mixolydian => vec![
                    "I-IV-v-I",
                    "I-ii-IV-I-IV-v-I",
                    "I-VII-IV-iv-v-IV-I",
                    "I-v-I-VII-IV-v-I",
                    "I-I-IV-I-VII-IV-I",
                ],
                Aeolian => vec![
                    "VI-VII-i-VII",
                    "VI-VII-i-III",
                    "VI-III-VII",
                    "III-VII-i",
                ],
                Locrian => vec![
                    "i-iii-i-V",
                    "i-vii-iv-iii",
                ],
            }
        },
        Scale::Pentatonic(mode) => {
            use PentatonicMode::*;
            match mode {
                Major => vec![
                    "I-III-IV", // equivalent to I-IV-V with proper notation
                    "ii-IV-I", // equivalent to ii-V-I with proper notation
                ],
                Suspended => vec![
                    // Following are rewritten base on dorian chord progression
                    "i-III",
                    "i-III-IV",
                    "i-III-I",
                    "i-III-I-i",
                    "i-III-I-I-III-i"
                ],
                BluesMinor => vec![
                    // Following are rewritten base on dorian chord progression
                    "i-III",
                    "i-III-IV",
                    "i-III-I",
                    "i-III-I-i",
                    "i-III-I-I-III-i"
                ],
                BluesMajor => vec![
                    "I-III-IV", // equivalent to I-IV-V with proper notation
                    "ii-IV-I", // equivalent to ii-V-I with proper notation
                ],
                Minor => vec![
                    "I-III-IV", // equivalent to I-IV-V with proper notation
                    "ii-IV-I", // equivalent to ii-V-I with proper notation
                    // Following are rewritten base on dorian chord progression
                    "i-III",
                    "i-III-IV",
                    "i-III-I",
                    "i-III-I-i",
                    "i-III-I-I-III-i"
                ],
            }
        },
    };
    
    if !full_random {
        chord_progressions.iter().choose(&mut seed).unwrap().to_string()
    } else {
        let mut chord_progression: Vec<&str> = vec![];
        let nb_chord = (2..8).into_iter().choose(&mut seed).unwrap();
        for _i in 0..nb_chord {
            chord_progression.push(chords.iter().choose(&mut seed).unwrap());
        }

        chord_progression.join("-")
    }
}