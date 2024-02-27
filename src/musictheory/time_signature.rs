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
            let counting_value = splitted_signature[1].parse::<i32>().unwrap_or_default();
            let total_values = splitted_signature[0].parse::<i32>().unwrap_or_default();

            if (counting_value & (counting_value - 1)) == 0 { // check if counting_value is a power of 2
                if counting_value > 0 && total_values > 0 {
                    let relative_couting_value = 1.0 / counting_value as f32;
                    return Ok(TimeSignature::from(total_values as f32 * relative_couting_value));
                }
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
        let mut numerator = (f32::from(*self) * 1000.0) as i32;
        let mut denominator = 1000;

        let gcd = num::integer::gcd(numerator, denominator);
        numerator /= gcd;
        denominator /= gcd;

        if denominator == 1 {
            numerator *= 4;
            denominator *= 4
        }

        write!(
            f,
            "{}/{}",
            numerator,
            denominator
        )
    }
}