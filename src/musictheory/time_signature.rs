use core::fmt;
use std::{io, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TimeSignature(pub f32);

impl Default for TimeSignature {
    fn default() -> Self {
        TimeSignature(1.0)
    }
}

impl From<f32> for TimeSignature {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<TimeSignature> for f32 {
    fn from(value: TimeSignature) -> Self {
        value.0
    }
}

impl FromStr for TimeSignature {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted_signature = s.split('/').collect::<Vec<&str>>();
        if splitted_signature.len() == 2 {
            let counting_value = splitted_signature[1].parse::<f32>().unwrap_or_default();
            let total_values = splitted_signature[0].parse::<f32>().unwrap_or_default();

            if counting_value > 0.0 && total_values > 0.0 {
                let relative_couting_value = 1.0 / counting_value;
                return Ok(TimeSignature::from(total_values * relative_couting_value));
            }
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} is not a valid signature", s),
        ))
    }
}

impl fmt::Display for TimeSignature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(to_str not implemented yet)"
        )
    }
}