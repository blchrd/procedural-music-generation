pub mod musicsource;
pub mod musictheory;
pub mod musicgeneration;
pub mod signal;

#[cfg(test)]
mod test;

fn f64_to_f32(f: f64) -> f32 {
	#![allow(clippy::cast_possible_truncation)]
	f as f32
}