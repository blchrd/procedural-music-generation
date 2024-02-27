use super::{note_value::NoteValue, piano_key::PianoKey, sheet_note::SheetNote, time_signature::TimeSignature};

#[derive(Debug, Clone)]
pub struct Measure {
    pub notes: Vec<SheetNote>,
    pub time_signature: TimeSignature //this will be the time signature at the end, it should work nice, because 3/4 is literally 0.75, we'll see
}

impl Default for Measure {
    fn default() -> Self {
        Measure {
            notes: Vec::<SheetNote>::new(),
            time_signature: TimeSignature::default()
        }
    }
}

impl Measure {
    pub fn new(time_signature: TimeSignature) -> Self {
        Measure {
            notes: Vec::<SheetNote>::new(),
            time_signature: time_signature,
        }
    }

    pub fn get_remaining_value(&self) -> f32 {
        let mut note_values_sum = 0.0;
        self.notes.iter().for_each(|n| note_values_sum += n.value.get_relative_duration());

        f32::from(self.time_signature) - note_values_sum
    }

    pub fn is_measure_complete(&self) -> bool {
        self.get_remaining_value() == 0.0
    }

    pub fn add_note(&mut self, note: PianoKey, value: NoteValue) {
        if value.get_relative_duration() > self.get_remaining_value() {
            panic!("Measure overflow")
        }
        self.notes.push(SheetNote {note, value});
    }
}