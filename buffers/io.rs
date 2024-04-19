use crate::NoteMessage;
use crate::time::*;
use crate::float::*;
use crate::buffers::bus::*;
use crate::buffers::*;

pub struct IO {
    pub audio: Bus<Buffer<Stereo<f32>>>,
    pub events: Bus<Buffer<NoteMessage>>,
    pub control: Bus<Box<f32>>,
    pub time: Bus<Box<TimeMessage>>,
}
