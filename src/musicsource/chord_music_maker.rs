// Make controller with multiple source for chord progression

use std::{f32::consts::PI, time::Duration};

use rodio::Source;

use crate::{f64_to_f32, musictheory::{chord_progression::ChordProgression, hertz::Hertz, pitch::Pitch, tempo::Tempo}};

pub const SAMPLE_RATE: Hertz = Hertz(48_000.0);
pub type Sample = f32;

#[derive(Clone)]
pub struct ChordMusicMaker {
    chord_progression: ChordProgression,
    current_chord: usize,
    current_sample: usize,
    sample_rate: Hertz,
    tempo: Tempo,
    volume: f32,
}

impl Default for ChordMusicMaker {
    fn default() -> Self {
        Self {
            chord_progression: ChordProgression::default(),
            current_chord: 0,
            current_sample: usize::default(),
            sample_rate: SAMPLE_RATE,
            tempo: Tempo::from(60),
            volume: 2.0,
        }
    }
}

impl ChordMusicMaker {
    pub fn new(chord_progression: ChordProgression, tempo: u16) -> Self {
        Self::default().set_chord_progression(chord_progression).set_tempo(Tempo::from(tempo))
    }

    fn next_chord(&mut self) {
        self.current_chord += 1;
        if self.current_chord >= self.chord_progression.chords.len() {
            self.current_chord = 0;
        }
    }

    fn set_chord_progression(mut self, chord_progression: ChordProgression) -> Self {
        self.chord_progression = chord_progression;
        self
    }

    fn set_tempo(mut self, tempo: Tempo) -> Self {
        self.tempo = tempo;
        self
    }
}

impl Iterator for ChordMusicMaker {
    type Item = Sample; //Sampled amplitude
    fn next(&mut self) -> Option<Self::Item> {
        self.current_sample = self.current_sample.wrapping_add(1);

        // Add frequencies from the different notes of the chord
        let mut value = 0.0_f32;
        let mut sin = 0.0_f32;
        
        self.chord_progression.chords[self.current_chord].get_keys().iter().for_each(|n| {
            let frequency = f64_to_f32(Pitch::from(*n).into());

            value += self.volume
                * PI
                * frequency
                * self.current_sample as Sample
                / f64::from(self.sample_rate) as Sample;

            sin += value.sin();
        });

        if self.current_sample as f64 >= (f64::from(self.sample_rate) / self.tempo.get_bps()  as f64) {
            self.current_sample = 0;
            self.next_chord();
        }
        Some(sin)
    }
}

impl Source for ChordMusicMaker {
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
