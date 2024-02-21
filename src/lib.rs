pub mod chord_music_maker;
pub mod melody_music_maker;
pub mod musictheory;

#[cfg(test)]
mod test;

fn f64_to_f32(f: f64) -> f32 {
	#![allow(clippy::cast_possible_truncation)]
	f as f32
}