use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};

use crate::musictheory::{
    key::Key, 
    measure::Measure, 
    piano_key::PianoKey, 
    scale::Scale, 
    pattern::Pattern, 
    time_signature::TimeSignature
};

use super::rhythm_pattern_generator::rhythm_pattern_generation;

pub fn pattern_generation(name: String, base_note: PianoKey, scale: Scale, octaves: u8, nb_measures: i32) -> Pattern {
    let mut pattern = Pattern::new(name);
    
    let mut seed = SmallRng::from_entropy();
    let keys = Key::new(scale, base_note, octaves).all_keys();

    for _ in 0..nb_measures {
        let rhythm_pattern = rhythm_pattern_generation(TimeSignature::default());
        let mut measure = Measure::new(TimeSignature::default());
        let mut prev_note: Option<PianoKey> = None;
        rhythm_pattern.iter().for_each(|value| {
            let mut note = *keys.iter().choose(&mut seed).unwrap();
            while prev_note.is_some() && note == prev_note.unwrap() {
                note = *keys.iter().choose(&mut seed).unwrap();
            }
            prev_note = Some(note.clone());
            measure.add_note(note, *value);
        });
        pattern.add_measure(measure);
    }

    pattern
}