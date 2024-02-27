use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};

use crate::musictheory::{key::Key, measure::Measure, note_value::{NoteValue, NoteValueBase}, piano_key::PianoKey, scale::Scale, sheet::Sheet, time_signature::TimeSignature};

pub fn sheet_generation(base_note: PianoKey, scale: Scale, octaves: u8) -> Sheet {
    use NoteValueBase::{Half, Eighth};
    let mut sheet = Sheet::new();
    
    let mut seed = SmallRng::from_entropy();
    let eighth_note = NoteValue{base: Eighth, dotted: None};
    let half_note = NoteValue{base: Half, dotted: None};
    let keys = Key::new(scale, base_note, octaves).all_keys();
    let nb_measures = 4;

    // For now, we hard-code the rhythm, but we'll eventually generate it randomly
    let rhythm_pattern = vec![eighth_note, eighth_note, eighth_note, half_note, eighth_note];

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