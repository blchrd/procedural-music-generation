//TODO: Change the sustain behavior, we just want it to sustain until the note is released
//  It's not a fixed duration like the way it's implemented here.
//  Same thing with the "attack" formula, we want it to start from any volume, it's not always start at 0
//  These adjustements will make the global sound more organic.

use core::fmt;
use crate::{f64_to_f32, musictheory::hertz::Hertz};

#[derive(Debug, Clone, Copy)]
pub struct AdsrEnvelop {
    attack: f32, // from 0.0 to max volume
    decay: f32, // from max volume to sustain volume 
    sustain: f32, // sustain volume all along
    release: f32, // from sustain volume to 0.0
}

impl Default for AdsrEnvelop {
    fn default() -> Self {
        AdsrEnvelop {
            attack: 0.0,
            decay: 0.0,
            sustain: 100.0,
            release: 0.0,
        }
    }
}

impl AdsrEnvelop {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        AdsrEnvelop::default()
            .set_attack(attack)
            .set_decay(decay)
            .set_sustain(sustain)
            .set_release(release)
    }

    pub fn set_attack(mut self, attack: f32) -> Self {
        self.attack = attack;
        self
    }

    pub fn set_decay(mut self, decay: f32) -> Self {
        self.decay = decay;
        self
    }

    pub fn set_sustain(mut self, sustain: f32) -> Self {
        self.sustain = sustain;
        self
    }

    pub fn set_release(mut self, release: f32) -> Self {
        self.release = release;
        self
    }

    pub fn get_decay_start(&self) -> f32 {
        self.attack
    }

    pub fn get_sustain_start(&self) -> f32 {
        self.attack + self.decay
    }

    pub fn get_release_start(&self) -> f32 {
        self.attack + self.decay + self.sustain
    }

    pub fn get_note_duration(&self) -> f32 {
        self.attack + self.decay + self.sustain + self.release
    }

    pub fn get_amplitude_for_sample(&self, sample: f64, sample_rate: Hertz) -> f32 {
        let base_amplitude: f32 = 1.0;
        let attack_max_level = 0.5;
        let amplitude: f32;
        let current_note_duration = sample as f32 / f64_to_f32(f64::from(sample_rate));
        if (sample) < (f64::from(sample_rate) / (1.0 / self.get_decay_start() as f64)) {
            // Attack
            let slope = (base_amplitude + attack_max_level) / self.get_decay_start();
            amplitude = current_note_duration * slope;
        } else if (sample) < (f64::from(sample_rate) / (1.0 / self.get_sustain_start() as f64)) {
            // Decay
            let slope = -attack_max_level / (self.get_sustain_start() - self.get_decay_start());
            amplitude = (current_note_duration - self.get_decay_start()) * slope + (base_amplitude + attack_max_level);
        } else if (sample) < (f64::from(sample_rate) / (1.0 / self.get_release_start() as f64)) {
            // Sustain
            amplitude = base_amplitude;
        } else if (sample) < (f64::from(sample_rate) / (1.0 / self.get_note_duration() as f64)) {
            // Release
            let slope = -base_amplitude / (self.get_note_duration() - self.get_release_start());
            amplitude = ((current_note_duration - self.get_release_start())) * slope + base_amplitude;
        } else {
            amplitude = 0.0;
        }

        amplitude
    }
}

impl fmt::Display for AdsrEnvelop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Attack: {} s / Decay: {} s / Sustain: {} s / Release {} s",
            self.attack, self.decay, self.sustain, self.release
        )
    }
}