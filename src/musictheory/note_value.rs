use core::fmt;

use super::tempo::Tempo;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum NoteValueBase {
    Whole=1,
    Half=2,
    Quarter=4,
    Eighth=8,
    Sixteenth=16,
}

impl From<NoteValueBase> for i8 {
    fn from(value: NoteValueBase) -> Self {
        value as i8
    }
}

impl Default for NoteValueBase {
    fn default() -> Self {
        Self::Quarter
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum NoteValueDotted {
    Dotted=2,
    DoubleDotted=4,
    TripleDotted=8,
}

impl fmt::Display for NoteValueDotted {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use NoteValueDotted::*;
        let acc_str = match self {
            Dotted => "dotted",
            DoubleDotted => "double dotted",
            TripleDotted => "triple dotted",
        };
        write!(f, "{}", acc_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NoteValue {
    pub base: NoteValueBase,
    pub dotted: Option<NoteValueDotted>,
}

impl Default for NoteValue {
    fn default() -> Self {
        NoteValue {
            base: NoteValueBase::default(),
            dotted: None,
        }
    }
}

impl fmt::Display for NoteValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dotted_str = if let Some(d) = self.dotted {
            format!(" {}", d)
        } else {
            "".to_string()
        };
        
        write!(f, "{:?}{} note", self.base, dotted_str)
    }
}

impl NoteValue {
    pub fn get_relative_duration(&self) -> f32 {
        let base_duration = 1.0 / i8::from(self.base) as f32;
        let dotted_duration = if let Some(d) = self.dotted {
            match d {
                NoteValueDotted::Dotted => base_duration / 2.0,
                NoteValueDotted::DoubleDotted => base_duration / 2.0 + base_duration / 4.0,
                NoteValueDotted::TripleDotted => base_duration / 2.0 + base_duration / 4.0 + base_duration / 8.0,
            }
        } else {
            0.0
        };
        
        base_duration + dotted_duration
    }

    pub fn get_duration_for_tempo(&self, tempo: Tempo) -> f32 {
        (self.get_relative_duration() * 4.0) / tempo.get_bps()
    }
}