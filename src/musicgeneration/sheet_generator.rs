use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};

use crate::musictheory::{
    key::Key, 
    measure::Measure, 
    piano_key::PianoKey, 
    scale::Scale, 
    sheet::Sheet, 
    time_signature::TimeSignature
};

use super::rhythm_pattern_generator::rhythm_pattern_generation;

pub fn sheet_generation(base_note: PianoKey, scale: Scale, octaves: u8) -> Sheet {
    let mut sheet = Sheet::new();
    
    let mut seed = SmallRng::from_entropy();
    let keys = Key::new(scale, base_note, octaves).all_keys();
    let nb_measures = 4;

    let rhythm_pattern = rhythm_pattern_generation(TimeSignature::default());

    for _ in 0..nb_measures {
        let mut measure = Measure::new(TimeSignature::default());
        rhythm_pattern.iter().for_each(|value| {
            let note = *keys.iter().choose(&mut seed).unwrap();
            measure.add_note(note, *value);
        });
        sheet.add_measure(measure);
    }

    sheet
}