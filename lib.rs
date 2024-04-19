pub mod buffers;
pub mod math;
pub mod sample;
pub mod time;
pub mod event;
pub mod traits;
pub mod float;
pub mod routing;

pub use buffers::*;
pub use math::*;
pub use sample::*;
pub use time::*;
pub use event::*;
pub use traits::*;
pub use float::*;
pub use routing::*;

extern crate lazy_static;

#[derive(Clone, PartialEq)]
pub struct SoundRegion<T> {
    pub low_note: u32,
    pub high_note: u32,
    pub low_velocity: f32,
    pub high_velocity: f32,
    pub sounds: Vec<T>,
    pub index: usize,
}
