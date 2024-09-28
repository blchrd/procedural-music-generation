use rand::{rngs::SmallRng, seq::IteratorRandom};
use crate::musictheory::{
    note_value::{NoteValue, NoteValueBase, NoteValueDotted}, 
    time_signature::TimeSignature
};

//TODO: Split the signature, and use the same note value in each (or just randomize it completely)

pub fn rhythm_pattern_generation_for_chord(time_signature: TimeSignature, mut seed: &mut SmallRng) -> Vec<NoteValue> {
    use NoteValueBase::{Whole, Half, Quarter};
    use NoteValueDotted::Dotted;
    let _whole_note = NoteValue{base: Whole, dotted: None};
    let _half_note = NoteValue{base: Half, dotted: None};
    let _half_note_dotted = NoteValue{base: Half, dotted: Some(Dotted)};
    let _quarter_note = NoteValue{base: Quarter, dotted: None};

    let patterns_4_4 = vec![
        vec![_whole_note],
        vec![_half_note, _half_note],
        vec![_whole_note, _half_note, _half_note],
    ];
    let patterns_3_4 = vec![
        vec![_half_note_dotted],
        vec![_half_note, _quarter_note],
    ];

    if f32::from(time_signature) == 1.0 {
        return patterns_4_4.iter().choose(&mut seed).unwrap().clone();
    } else if f32::from(time_signature) == 0.75 {
        return patterns_3_4.iter().choose(&mut seed).unwrap().clone();
    }
    
    vec![]
}

pub fn rhythm_pattern_rand_generation(time_signature: TimeSignature, mut seed: &mut SmallRng) -> Vec<NoteValue> {
    // Put the same value multiple time to weight the RNG
    // Find a way to randomize the weights
    let note_values: Vec<NoteValue> = vec![
        NoteValue{base: NoteValueBase::Whole, dotted: None},
        NoteValue{base: NoteValueBase::Half, dotted: None},
        NoteValue{base: NoteValueBase::Half, dotted: None},
        NoteValue{base: NoteValueBase::Quarter, dotted: None},
        NoteValue{base: NoteValueBase::Quarter, dotted: None},
        NoteValue{base: NoteValueBase::Quarter, dotted: None},
        NoteValue{base: NoteValueBase::Eighth, dotted: None},
        NoteValue{base: NoteValueBase::Eighth, dotted: None},
        NoteValue{base: NoteValueBase::Eighth, dotted: None},
        NoteValue{base: NoteValueBase::Eighth, dotted: None},
        NoteValue{base: NoteValueBase::Sixteenth, dotted: None},
        NoteValue{base: NoteValueBase::Sixteenth, dotted: None},
        NoteValue{base: NoteValueBase::Sixteenth, dotted: None},
        NoteValue{base: NoteValueBase::Sixteenth, dotted: None},
        NoteValue{base: NoteValueBase::Sixteenth, dotted: None},
        NoteValue{base: NoteValueBase::Sixteenth, dotted: None},
        // TODO: Add some dotted note (for uneven time signature)
        // NoteValue{base: NoteValueBase::Whole, dotted: Some(NoteValueDotted::Dotted)},
        // NoteValue{base: NoteValueBase::Half, dotted: Some(NoteValueDotted::Dotted)},
        // NoteValue{base: NoteValueBase::Quarter, dotted: Some(NoteValueDotted::Dotted)},
        // NoteValue{base: NoteValueBase::Eighth, dotted: Some(NoteValueDotted::Dotted)},
        // NoteValue{base: NoteValueBase::Sixteenth, dotted: Some(NoteValueDotted::Dotted)},
    ];

    
    let mut rhythm_pattern:Vec<NoteValue> = vec![];
    let mut rhythm_pattern_sum = 0.0;
    while rhythm_pattern_sum < f32::from(time_signature) {
        let mut picked_note_value = note_values.iter().choose(&mut seed).unwrap();
        while rhythm_pattern_sum + picked_note_value.get_relative_duration() > f32::from(time_signature) {
            picked_note_value = note_values.iter().choose(&mut seed).unwrap();
        }

        rhythm_pattern.push(picked_note_value.clone());
        rhythm_pattern_sum = rhythm_pattern.iter().fold(0.0, |sum, nv| sum + nv.get_relative_duration());
    }

    rhythm_pattern
}

pub fn rhythm_pattern_generation(time_signature: TimeSignature, mut seed: &mut SmallRng) -> Vec<NoteValue> {
    use NoteValueBase::{Quarter, Eighth};
    use NoteValueDotted::Dotted;
    let quarter_note = NoteValue{base: Quarter, dotted: None};
    let dotted_quarter_note = NoteValue{base: Quarter, dotted: Some(Dotted)};
    let eighth_note = NoteValue{base: Eighth, dotted: None};

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