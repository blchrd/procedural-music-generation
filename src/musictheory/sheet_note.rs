use super::{note_value::NoteValue, piano_key::PianoKey};

#[derive(Debug, Clone, Copy)]
pub struct SheetNote {
    pub note: PianoKey,
    pub value: NoteValue,
}