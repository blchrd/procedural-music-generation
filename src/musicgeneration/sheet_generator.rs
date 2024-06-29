use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};

use crate::musictheory::{
    pattern::Pattern, 
    piano_key::PianoKey, 
    scale::Scale, 
    sheet::Sheet 
};

use super::pattern_generator::pattern_generation;

pub fn sheet_generation(base_note: PianoKey, scale: Scale, octaves: u8, nb_measures: i32, use_common_pattern: bool) -> Sheet {
    let mut sheet = Sheet::new();  
    let mut seed = SmallRng::from_entropy();
    let mut patterns = Vec::<Pattern>::new();
    let nb_pattern = (1..4).into_iter().choose(&mut seed).unwrap();

    for i in 0..nb_pattern {
        patterns.push(pattern_generation(
            String::from(format!("Pattern {}", i)), 
            base_note, 
            scale, 
            octaves, 
            nb_measures, 
            use_common_pattern
        ))
    }

    for _ in 0..nb_pattern*4 {
        sheet.add_pattern(patterns.iter().choose(&mut seed).unwrap().clone())
    }    

    sheet
}