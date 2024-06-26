pub mod cent;
pub mod interval;
pub mod key;
pub mod mode;
pub mod note;
pub mod piano_key;
pub mod scale;
pub mod semitone;
pub mod hertz;
pub mod pitch;
pub mod chord;
pub mod tempo;
pub mod chord_progression;
pub mod note_value;
pub mod sheet_note;
pub mod measure;
pub mod pattern;
pub mod sheet;
pub mod time_signature;

pub fn char_strs<'a>(s: &'a str) -> Vec<&'a str> {
    s.split("")
        .skip(1)
        .take_while(|c| *c != "")
        .collect::<Vec<&str>>()
}