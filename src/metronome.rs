use rodio::{Decoder, Source};
use std::{collections::HashSet, fs::File};

use crate::utils;

#[derive(Debug, Clone)]
pub struct Metronome {
    pub click_locations: HashSet<u64>,
    pub samples_per_beat: u64,
    pub sample_count: u64,
    pub sample_rate: u32,
    pub source_samples: Vec<f32>,

    // time sig
    pub divisor: u8,

    // divisions
    pub division: u8,
    pub swing: i16,
}

impl Metronome {
    pub fn new(click_path: &str, tempo: u64, sr: u32, division: u8, swing: i16) -> Self {
        let file = File::open(click_path).expect("click path not found");
        let source = Decoder::try_from(file).expect("could not decode click");
        let source_samples: Vec<f32> = source.collect();

        let samples_per_beat = utils::tempo_to_samples(tempo, sr);

        let mut metronome = Metronome {
            click_locations: HashSet::new(),
            samples_per_beat: samples_per_beat,
            sample_count: 0,
            sample_rate: sr,
            source_samples: source_samples,
            divisor: 4,
            division: division,
            swing: swing,
        };

        metronome.click_locations = metronome.get_click_locations();
        return metronome;
    }

    fn get_click_locations(self: &Metronome) -> HashSet<u64> {
        let mut positions: HashSet<u64> = HashSet::new();
        positions.insert(0);

        let swing_pos = self.click_position_swing();
        if self.division % 2 == 0 {
            let last_pos = positions.iter().max().unwrap_or(&0);
            positions.insert(last_pos + swing_pos);
        }
        if self.division % 3 == 0 {
            let last_pos = positions.iter().max().unwrap_or(&0);
            positions.insert(last_pos + swing_pos);
        }

        positions
    }

    fn click_position(self: &Metronome) -> u64 {
        return 1000 * self.samples_per_beat * self.divisor as u64 / self.division as u64 / 1000;
    }

    // adjusted for swing
    fn click_position_swing(self: &Metronome) -> u64 {
        let position = self.click_position() as i64 + self.swing_offset();
        if position < 0 {
            return 0;
        }
        position as u64
    }

    fn swing_offset(self: &Metronome) -> i64 {
        let ratio = (self.swing * 2) - 100;
        let scaled_offset = self.click_position() as i64 * ratio as i64;
        scaled_offset / 100
    }

    fn should_click(self: &Metronome) -> bool {
        if self.sample_count == 0 {
            return true;
        }
        if self.click_locations.contains(&self.sample_count) {
            return true;
        }
        false
    }

    pub fn amplitude(self: &Metronome) -> f32 {
        if self.click_locations.len() < 2 {
            return 1.0;
        }
        let second_click = self
            .click_locations
            .iter()
            .filter(|&v| *v != 0)
            .min()
            .unwrap_or(&0);
        if self.sample_count < *second_click || self.division < 5 {
            return 1.0;
        }
        return 0.3;
    }

    fn should_reset(self: &Metronome) -> bool {
        let last_click = self
            .click_locations
            .iter()
            .max()
            .map(|&v| v)
            .unwrap_or(self.click_position());
        self.sample_count % self.click_position() == 0 && self.sample_count > last_click
    }

    pub fn next_sample(self: &mut Metronome) -> f32 {
        let should_click = self.should_click();
        if should_click {
            let amp = self.amplitude();
            if self.should_reset() {
                self.sample_count = 0;
            }
            return self.source_samples[0] * amp;
        }

        let position = self
            .click_locations
            .iter()
            .filter(|&v| *v <= self.sample_count)
            .map(|&v| (self.sample_count - v) as usize)
            .min()
            .unwrap_or(0);

        if self.should_reset() {
            self.sample_count = 0;
        }

        if position < self.source_samples.len() {
            let amp = self.amplitude();
            return self.source_samples[position] * amp;
        }

        return 0.0;
    }
}

impl Iterator for Metronome {
    type Item = f32;

    fn next(self: &mut Metronome) -> Option<f32> {
        self.sample_count += 1;
        Some(self.next_sample())
    }
}

impl Source for Metronome {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
