use std::f32::consts::PI;
use core::{fmt, time::Duration};

use rodio::Source;

use crate::{f64_to_f32, musictheory::{hertz::Hertz, pitch::Pitch, sheet::Sheet, tempo::Tempo}};

pub const SAMPLE_RATE: Hertz = Hertz(48_000.0);
pub type Sample = f32;

#[derive(Clone)]
pub struct SheetMusicMaker {
    sheet: Sheet,
    current_note: usize,
    current_measure: usize,
    current_sample: usize,
    sample_rate: Hertz,
    tempo: Tempo,
    volume: f32,
}

impl Default for SheetMusicMaker {
    fn default() -> Self {
        Self {
            sheet: Sheet::new(),
            current_note: 0,
            current_measure: 0,
            current_sample: usize::default(),
            sample_rate: SAMPLE_RATE,
            tempo: Tempo::from(60),
            volume: 2.0,
        }
    }
}

impl SheetMusicMaker {
    pub fn new(sheet: Sheet, tempo: u16) -> Self {
        Self::default().set_sheet(sheet).set_tempo(Tempo::from(tempo))
    }
    fn get_frequency(&mut self) -> Sample {
        let current_sheet_note = self.sheet.measures[self.current_measure].notes[self.current_note];
        let pitch = Pitch::from(current_sheet_note.note);
        f64_to_f32(pitch.into())
    }
    fn next_note(&mut self) {
        self.current_note += 1;
        if self.current_note >= self.sheet.measures[self.current_measure].notes.len() {
            self.current_measure += 1;
            self.current_note = 0;
            if self.current_measure >= self.sheet.measures.len() {
                self.current_measure = 0
            }
        }
    }
    fn set_sheet(mut self, sheet: Sheet) -> Self {
        self.sheet = sheet;
        self
    }
    fn set_tempo(mut self, tempo: Tempo) -> Self {
        self.tempo = tempo;
        self
    }
}

impl Iterator for SheetMusicMaker {
    type Item = Sample; // Sampled amplitude
    fn next(&mut self) -> Option<Self::Item> {
        self.current_sample = self.current_sample.wrapping_add(1); // will cycle
        let current_sheet_note = self.sheet.measures[self.current_measure].notes[self.current_note];

        let value = self.volume
            * PI
            * self.get_frequency()
            * self.current_sample as Sample
            / f64::from(self.sample_rate) as Sample;

        if self.current_sample as f64 >= (f64::from(self.sample_rate) / (1.0 / current_sheet_note.value.get_duration_for_tempo(self.tempo)) as f64) {
            self.current_sample = 0;
            self.next_note(); 
        }

        // SawWave
        // Some(value.tan().recip().atan())
        // SquareWave
        // Some(value.sin().signum())
        // SineWave
        Some(value.sin())
    }
}

impl Source for SheetMusicMaker {
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

impl fmt::Display for SheetMusicMaker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Generating from sheet {}",
            self.sheet
        )
    }
}