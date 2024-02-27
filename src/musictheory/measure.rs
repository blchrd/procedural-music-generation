use super::{note_value::NoteValue, piano_key::PianoKey, sheet_note::SheetNote};

#[derive(Debug, Clone)]
pub struct Measure {
    pub notes: Vec<SheetNote>,
    total_note_value: f32 //this will be the time signature at the end, it should work nice, because 3/4 is literally 0.75, we'll see
}

impl Measure {
    pub fn new() -> Self {
        Measure {
            notes: Vec::<SheetNote>::new(),
            total_note_value: 1.0
        }
    }

    pub fn get_remaining_value(&self) -> f32 {
        let mut note_values_sum = 0.0;
        self.notes.iter().for_each(|n| note_values_sum += n.value.get_relative_duration());

        self.total_note_value - note_values_sum
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