use core::fmt;

use super::{note_value::NoteValue, piano_key::PianoKey};

#[derive(Debug, Clone, Copy)]
pub struct SheetNote {
    pub note: PianoKey,
    pub value: NoteValue,
}

impl fmt::Display for SheetNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.note, self.value)
    }
}