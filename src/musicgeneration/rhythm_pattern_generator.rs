use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};
use crate::musictheory::{
    note_value::{NoteValue, NoteValueBase, NoteValueDotted}, 
    time_signature::TimeSignature
};

pub fn rhythm_pattern_generation(time_signature: TimeSignature) -> Vec<NoteValue> {
    use NoteValueBase::{Quarter, Eighth};
    use NoteValueDotted::Dotted;
    let quarter_note = NoteValue{base: Quarter, dotted: None};
    let dotted_quarter_note = NoteValue{base: Quarter, dotted: Some(Dotted)};
    let eighth_note = NoteValue{base: Eighth, dotted: None};

    let mut seed = SmallRng::from_entropy();

    let common_pattern_4_4 = vec![
        vec![quarter_note, quarter_note, quarter_note, eighth_note, eighth_note],
        vec![eighth_note, eighth_note, eighth_note, quarter_note, eighth_note, quarter_note],
        vec![eighth_note, quarter_note, quarter_note, eighth_note, eighth_note, eighth_note],
        vec![quarter_note, eighth_note, quarter_note, eighth_note, eighth_note, eighth_note],
        vec![dotted_quarter_note, quarter_note, eighth_note, eighth_note, eighth_note],
    ];

    let common_pattern_3_4 = vec![
        vec![quarter_note, eighth_note, eighth_note, eighth_note, eighth_note],
        vec![eighth_note, eighth_note, quarter_note, eighth_note, eighth_note],
    ];

    if f32::from(time_signature) == 1.0 {
        return common_pattern_4_4.iter().choose(&mut seed).unwrap().clone();
    } else if f32::from(time_signature) == 0.75 {
        return common_pattern_3_4.iter().choose(&mut seed).unwrap().clone();
    }
    
    vec![]
}