use std::{
    fs::File,
    io::{Error, Read},
};

use crate::musictheory::{
    measure::Measure, note_value::NoteValue, pattern::Pattern, piano_key::PianoKey, sheet::Sheet, time_signature::TimeSignature
};

pub fn sheet_from_binary_file<EncodingError>(base_note: PianoKey, path: &str, half_byte_parsing: bool) -> Result<Sheet, Error> {
    let mut sheet = Sheet::new();
    let mut pattern = Pattern::new(String::new());
    let mut measure = Measure::new(TimeSignature::default());

    let bits = read_bytes_from_file(path, half_byte_parsing)?;

    bits.iter().for_each(|bit| {
        let mut note = base_note.clone();
        for _ in 0..*bit {
            note.inc();
        }

        measure.add_note(note, NoteValue::default());

        if measure.is_measure_complete() {
            pattern.add_measure(measure.clone());
            measure = Measure::new(TimeSignature::default());
        };
    });
    sheet.add_pattern(pattern);
    Ok(sheet)
}

fn read_bytes_from_file(path: &str, half_byte_parsing: bool) -> Result<Vec<u8>, Error> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    if !half_byte_parsing {
        return Ok(buffer)
    }

    let mut nibbles = Vec::with_capacity(buffer.len() * 2);
    for byte in buffer {
        let high_nibble = (byte >> 4) & 0x0F;
        let low_nibble = byte & 0x0F;
        nibbles.push(high_nibble);
        nibbles.push(low_nibble);
    }

    Ok(nibbles)
}