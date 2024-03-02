use crate::musictheory::{scale::Scale, time_signature::TimeSignature};

pub fn chord_progression_generation(_scale: Scale, _time_signature: TimeSignature) -> String {
    // TODO: Implement different chord progression dependant of scale and time_signature
    "I-V-vi-IV".to_string()
}