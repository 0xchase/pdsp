use std::ops::{Deref, DerefMut, Mul, Sub, Add};
use std::sync::Arc;

use rand::{rngs::ThreadRng, Rng};

use crate::buffers::*;
use crate::float::*;

use crate::Generator;
use crate::Pitched;

#[derive(Clone)]
pub struct SampleFile<S: Sample> {
    buffer: Arc<Buffer<S>>,
    pub path: String,
    pub start: usize,
    pub end: usize,
    pub pitch: Option<f32>
}

impl<S: Sample> SampleFile<S> {
    pub fn from(buffer: Arc<Buffer<S>>, path: String) -> Self {
        let start = 0;
        let end = buffer.len();

        return Self {
            buffer,
            start,
            end,
            path,
            pitch: None,
        };
    }

    pub fn set_pitch(&mut self, hz: f32) {
        self.pitch = Some(hz);
    }

    pub fn set_unpitched(&mut self) {
        self.pitch = None;
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn as_slice(&self) -> &[S] {
        self.buffer.as_slice()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}

pub struct Converter<F: Float, S: Sample, G: Generator<Output = S>, I: Interpolator<Item = S>> {
    src: G,
    interpolator: I,
    interpolation_value: F,
    ratio: f32
}

impl<F: Float, S: Sample, G: Generator<Output = S>, I: Interpolator<Item = S>> Converter<F, S, G, I> {
    pub fn from(src: G) -> Self {
        Self {
            src,
            interpolator: I::new(),
            interpolation_value: F::ZERO,
            ratio: 1.0
        }
    }

    pub fn set_ratio(&mut self, ratio: f32) {
        self.ratio = ratio;
    }
}

impl<F: Float, S: Sample<Float = F>, G: Generator<Output = S>, I: Interpolator<Item = S>> Generator for Converter<F, S, G, I> {
    type Output = G::Output;

    fn reset(&mut self) {
        self.interpolator.reset();
        self.src.reset();
    }

    fn prepare(&mut self, sample_rate: u32, block_size: usize) {
        self.src.prepare(sample_rate, block_size);
    }

    fn generate(&mut self) -> Self::Output {
        while self.interpolation_value >= F::MAX {
            self.interpolator.next_sample(self.src.generate());
            self.interpolation_value -= F::MAX;
        }

        let s = S::from(self.interpolation_value);
        let out = self.interpolator.interpolate(s);
        self.interpolation_value += F::from(self.ratio);
        return out;
    }
}

impl<F: Float, S: Sample, G: Generator<Output = S>, I: Interpolator<Item = S>> std::ops::Deref for Converter<F, S, G, I> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        &self.src
    }
}

impl<F: Float, S: Sample, G: Generator<Output = S>, I: Interpolator<Item = S>> std::ops::DerefMut for Converter<F, S, G, I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.src
    }
}

pub trait Interpolator {
    type Item: Sample;

    fn new() -> Self;
    fn reset(&mut self);
    fn next_sample(&mut self, input: Self::Item);
    fn interpolate(&self, x: Self::Item) -> Self::Item;
}

pub struct Linear<S: Add + Sub + Mul> {
    last: S,
    prev: S
}

impl<F: Sample> Interpolator for Linear<F> {
    type Item = F;

    fn new() -> Self {
        Self {
            last: F::EQUILIBRIUM,
            prev: F::EQUILIBRIUM
        }
    }

    fn reset(&mut self) {
        self.last = F::EQUILIBRIUM;
        self.prev = F::EQUILIBRIUM;
    }

    fn next_sample(&mut self, input: Self::Item) {
        self.last = self.prev;
        self.prev = input;
    }

    fn interpolate(&self, x: F) -> Self::Item {
        ((self.prev - self.last) * x) + self.last
    }
}

pub struct SamplePlayer<T: Sample> {
    sample: Option<SampleFile<T>>,
    playing: bool,
    index: usize,
    start: usize,
    end: usize,
    should_loop: bool
}

impl<T: Sample> SamplePlayer<T> {
    pub fn new() -> Self {
        Self {
            sample: None,
            playing: false,
            index: 0,
            start: 0,
            end: 0,
            should_loop: false
        }
    }

    pub fn set_sample(&mut self, sample: SampleFile<T>) {
        self.index = sample.start;
        self.start = sample.start;
        self.end = sample.end;
        self.sample = Some(sample);
    }

    pub fn set_loop(&mut self, should_loop: bool) {
        self.should_loop = should_loop;
    }

    pub fn set_start(&mut self, start: usize) {
        self.start = start;
    }

    pub fn set_end(&mut self, end: usize) {
        self.end = end;
    }

    pub fn position(&self) -> usize {
        match &self.sample {
            Some(_sample) => self.index - self.start,
            None => self.index
        }
    }

    pub fn set_position(&mut self, position: usize) {
        match &self.sample {
            Some(_sample) => self.index = position + self.start,
            None => self.index = position
        }
    }

    pub fn progress(&self) -> f32 {
        match &self.sample {
            Some(_sample) => {
                (self.index - self.start) as f32 / (self.end - self.start) as f32
            },
            None => {
                0.0
            }
        }
    }

    pub fn playing(&self) -> bool {
        self.playing
    }

    pub fn play(&mut self) {
        self.playing = true;
    }

    pub fn pause(&mut self) {
        self.playing = false;
    }

    pub fn stop(&mut self) {
        self.playing = false;

        match &self.sample {
            Some(sample) => self.index = sample.start,
            None => self.index = 0
        }
    }
}

impl<T: Sample> Generator for SamplePlayer<T> {
    type Output = T;

    fn reset(&mut self) {}
    fn prepare(&mut self, _sample_rate: u32, _block_size: usize) {}

    fn generate(&mut self) -> Self::Output {
        if self.playing && self.start != self.end {
            if let Some(sample) = &self.sample {
                if self.should_loop {
                    self.index += 1;

                    return sample.buffer[self.start + (self.index - 1) % usize::max(self.end - self.start, 1)]
                } else {
                    if self.index < sample.end {
                        self.index += 1;
                        return sample.buffer[self.index - 1];
                    }
                }
            }
        }

        T::EQUILIBRIUM
    }
}

pub struct PitchedSamplePlayer<S: Sample> {
    player: Converter<S::Float, S, SamplePlayer<S>, Linear<S>>,
    pitch: f32,
}

impl<T: Sample> PitchedSamplePlayer<T> {
    pub fn new() -> Self {
        Self {
            player: Converter::from(SamplePlayer::new()),
            pitch: 440.0
        }
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.player.set_ratio(speed);
    }
}

impl<T: Sample> Generator for PitchedSamplePlayer<T> {
    type Output = T;

    fn reset(&mut self) {}

    fn prepare(&mut self, sample_rate: u32, block_size: usize) {
        self.player.prepare(sample_rate, block_size);
    }

    fn generate(&mut self) -> Self::Output {
        self.player.generate()
    }
}

impl<T: Sample> Pitched for PitchedSamplePlayer<T> {
    fn get_pitch(&self) -> f32 {
        self.pitch
    }

    fn set_pitch(&mut self, hz: f32) {
        match &self.sample {
            Some(sample) => {
                match sample.pitch {
                    Some(pitch) => self.player.set_ratio(hz / pitch),
                    None => self.player.set_ratio(1.0)
                }
            }
            None => ()
        }
    }
}

impl<T: Sample> Deref for PitchedSamplePlayer<T> {
    type Target = SamplePlayer<T>;

    fn deref(&self) -> &Self::Target {
        self.player.deref()
    }
}

impl<T: Sample> DerefMut for PitchedSamplePlayer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.player.deref_mut()
    }
}

pub struct GranularSamplePlayer<F: Sample> {
    player: PitchedSamplePlayer<F>,
    rng: ThreadRng,
    start_delta: f32,
}

impl<F: Sample> GranularSamplePlayer<F> {
    pub fn new() -> Self {
        let mut player = PitchedSamplePlayer::<F>::new();
        player.set_loop(true);

        Self {
            player,
            rng: rand::thread_rng(),
            start_delta: 0.0,
        }
    }

    pub fn play(&mut self, grain_position: f32, grain_spread: f32, grain_length: f32) {
        let length = match &self.sample {
            Some(sample) => sample.len(),
            None => 0
        };

        self.start_delta = self.rng.gen_range(
            -grain_spread..=grain_spread
        );

        if let Some(sample) = &self.sample {
            let start = usize::clamp(f32::round(
                length as f32 * f32::clamp(
                    grain_position + self.start_delta,
                    0.0,
                    1.0
                )
            ) as usize, 0, sample.len() - 1);

            let end = usize::clamp(start + f32::round(
                sample.len() as f32 * grain_length / 10.0,
            ) as usize, 0, sample.len() - 1);

            self.player.set_loop(true);
            self.player.set_start(start);
            self.player.set_end(end);
            self.player.set_position(start);
            self.player.play();
        }
    }

    pub fn update(&mut self, grain_position: f32, grain_length: f32) {
        let length = match &self.sample {
            Some(sample) => sample.len(),
            None => 0
        };

        if let Some(sample) = &self.sample {
            let start = usize::clamp(f32::round(
                length as f32 * f32::clamp(
                    grain_position + self.start_delta,
                    0.0,
                    1.0
                )
            ) as usize, 0, sample.len() - 1);

            let end = usize::clamp(start + f32::round(
                sample.len() as f32 * grain_length / 10.0,
            ) as usize, 0, sample.len() - 1);

            self.player.set_start(start);
            self.player.set_end(end);
        }
    }
}

impl<T: Sample> Generator for GranularSamplePlayer<T> {
    type Output = T;

    fn reset(&mut self) {}

    fn prepare(&mut self, sample_rate: u32, block_size: usize) {
        self.player.prepare(sample_rate, block_size);
    }

    fn generate(&mut self) -> Self::Output {
        self.player.generate()
    }
}

impl<T: Sample> Deref for GranularSamplePlayer<T> {
    type Target = PitchedSamplePlayer<T>;

    fn deref(&self) -> &Self::Target {
        &self.player
    }
}

impl<T: Sample> DerefMut for GranularSamplePlayer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.player
    }
}
