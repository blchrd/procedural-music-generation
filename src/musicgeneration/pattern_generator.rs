use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};

use crate::musictheory::{
    key::Key, 
    measure::Measure, 
    piano_key::PianoKey, 
    scale::Scale, 
    pattern::Pattern, 
    time_signature::TimeSignature
};

//TODO: If we're not using the common rhythm pattern, then we have to
//  narrow down or extend the max_distance between notes;
//  if we have an eighth, then narrow the distance
//  but if we have a whole note, we can extend it

use super::rhythm_pattern_generator::rhythm_pattern_generation;

pub fn pattern_generation(name: String, base_note: PianoKey, scale: Scale, octaves: u8, nb_measures: i32) -> Pattern {
    let mut pattern = Pattern::new(name);
    let max_distance = 5;
    let max_distance_between_measures = 14;

    let mut seed = SmallRng::from_entropy();
    let keys = Key::new(scale, base_note, octaves).all_keys();
    let mut measure_last_note: Option<PianoKey> = None;

    for _ in 0..nb_measures {
        let rhythm_pattern = rhythm_pattern_generation(TimeSignature::default());
        let mut measure = Measure::new(TimeSignature::default());
        let mut prev_note: Option<PianoKey> = None;
        rhythm_pattern.iter().for_each(|value| {
            let mut note = *keys.iter().choose(&mut seed).unwrap();
            while 
                measure_last_note.is_some() && (note.get_distance(measure_last_note.unwrap()) > max_distance_between_measures) ||
                prev_note.is_some() && (note == prev_note.unwrap() || prev_note.unwrap().get_distance(note) > max_distance) {
                note = *keys.iter().choose(&mut seed).unwrap();
            }
            measure_last_note = None;
            prev_note = Some(note.clone());
            measure.add_note(note, *value);
        });
        pattern.add_measure(measure);
        measure_last_note = prev_note.clone();
    }

    pattern
}