// Author: Ben Lovy <ben@deciduously.com>
// License: MIT
// Modified by Thomas Blanchard

use std::f32::consts::PI;
use std::str::FromStr;
use core::{fmt, time::Duration};
use rand::{rngs::SmallRng, seq::IteratorRandom, SeedableRng};

use rodio::Source;

use crate::{f64_to_f32, musictheory::{hertz::Hertz, key::Key, piano_key::PianoKey, pitch::Pitch, scale::Scale, tempo::Tempo}};

pub const SAMPLE_RATE: Hertz = Hertz(44_100.0);
pub type Sample = f32;

#[derive(Clone)]
pub struct MelodyMusicMaker {
    key: Key,
    seed: SmallRng,
    current_note: PianoKey,
    current_sample: usize,
    sample_rate: Hertz,
    tempo: Tempo,
    volume: f32,
}

impl Default for MelodyMusicMaker {
    fn default() -> Self {
        Self {
            key: Key::default(),
            seed: SmallRng::from_entropy(),
            current_note: PianoKey::from_str("C4").unwrap(),
            current_sample: usize::default(),
            sample_rate: SAMPLE_RATE,
            tempo: Tempo::from(60),
            volume: 2.0,
        }
    }
}

impl MelodyMusicMaker {
    pub fn new(base_note: PianoKey, scale: Scale, octaves: u8, tempo: u16) -> Self {
        Self::default().set_key(base_note, scale, octaves).set_tempo(Tempo::from(tempo))
    }
    fn get_frequency(&mut self) -> Sample {
        let pitch = Pitch::from(self.current_note);
        f64_to_f32(pitch.into())
    }
    fn new_note(&mut self) {
        let keys = self.key.all_keys();
        self.current_note = *keys.iter().choose(&mut self.seed).unwrap();
    }
    fn set_key(mut self, base_note: PianoKey, scale: Scale, octaves: u8) -> Self {
        self.key = Key::new(scale, base_note, octaves);
        self
    }
    fn set_tempo(mut self, tempo: Tempo) -> Self {
        self.tempo = tempo;
        self
    }
}

impl Iterator for MelodyMusicMaker {
    type Item = Sample; // Sampled amplitude
    fn next(&mut self) -> Option<Self::Item> {
        self.current_sample = self.current_sample.wrapping_add(1); // will cycle

        let value = self.volume
            * PI
            * self.get_frequency()
            * self.current_sample as Sample
            / f64::from(self.sample_rate) as Sample;

        if self.current_sample as f64 >= (f64::from(self.sample_rate) / self.tempo.get_bps() as f64) {
            self.current_sample = 0;
            self.new_note(); 
        }

        // SawWave
        // Some(value.tan().recip().atan())
        // SquareWave
        // Some(value.sin().signum())
        // SineWave
        Some(value.sin())
    }
}

impl Source for MelodyMusicMaker {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        f64::from(self.sample_rate) as u32
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl fmt::Display for MelodyMusicMaker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let key = self.key;
        write!(
            f,
            "Generating music from the {} {}\nOctaves: {} - {}\n{}",
            key.base_note.note,
            key.scale,
            key.base_note.octave,
            key.base_note.octave + key.octaves,
            key
        )
    }
}