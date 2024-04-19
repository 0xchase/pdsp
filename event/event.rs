use std::{sync::{Arc, Mutex}, fmt::Display};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use crate::buffers::*;

pub const NOTE_NAMES: [&'static str; 120] = [
    "C0", "C#0", "D0", "D#0", "E0", "F0", "F#0", "G0", "G#0", "A0", "A#0", "B0", "C1", "C#1", "D1",
    "D#1", "E1", "F1", "F#1", "G1", "G#1", "A1", "A#1", "B1", "C2", "C#2", "D2", "D#2", "E2", "F2",
    "F#2", "G2", "G#2", "A2", "A#2", "B2", "C3", "C#3", "D3", "D#3", "E3", "F3", "F#3", "G3",
    "G#3", "A3", "A#3", "B3", "C4", "C#4", "D4", "D#4", "E4", "F4", "F#4", "G4", "G#4", "A4",
    "A#4", "B4", "C5", "C#5", "D5", "D#5", "E5", "F5", "F#5", "G5", "G#5", "A5", "A#5", "B5", "C6",
    "C#6", "D6", "D#6", "E6", "F6", "F#6", "G6", "G#6", "A6", "A#6", "B6", "C7", "C#7", "D7",
    "D#7", "E7", "F7", "F#7", "G7", "G#7", "A7", "A#7", "B7", "C8", "C#8", "D8", "D#8", "E8", "F8",
    "F#8", "G8", "G#8", "A8", "A#8", "B8", "C9", "C#9", "D9", "D#9", "E9", "F9", "F#9", "G9",
    "G#9", "A9", "A#9", "B9",
];

lazy_static!(
    static ref LAST_ID: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Id(pub u64);

impl Id {
    pub fn num(&self) -> u64 {
        self.0
    }
}

impl Id {
    pub fn new() -> Self {
        let mut last_id = LAST_ID.lock().unwrap();
        *last_id = *last_id + 1;
        return Id(*last_id);
    }
}

#[derive(Copy, Clone)]
pub struct Note {
    pub id: Id,
    pub pitch: f32,
    pub pressure: f32
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
#[repr(C, u32)]
pub enum Event {
    NoteOn { pitch: f32, pressure: f32 },
    NoteOff,
    Pitch(f32),
    Pressure(f32),
    Other(u32, f32),
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::NoteOn { pitch, pressure } => {
                write!(f, "Event::NoteOn {{ pitch: {:.2}, pressure: {:.2} }}", *pitch, *pressure)
            },
            Event::NoteOff => {
                write!(f, "Event::NoteOff")
            },
            Event::Pitch(pitch) => {
                write!(f, "Event::Pitch {{ pitch: {:.2} }}", *pitch)
            },
            Event::Pressure(pressure) => {
                write!(f, "Event::Pressure {{ pressure: {:.2} }}", *pressure)
            },
            Event::Other(id, value) => {
                write!(f, "Event::Other {{ id: {}, value: {:.2} }}", *id, *value)
            },
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NoteMessage {
    pub id: Id,
    pub offset: usize,
    pub note: Event
}

impl NoteMessage {
    pub fn from_num(num: u32) -> Self {
        let hz = 440.0 * 2.0_f32.powf((num as f32 - 69.0) / 12.0);

        Self {
            id: Id::new(),
            offset: 0,
            note: Event::NoteOn { 
                pitch: hz,
                pressure: 0.5,
            }
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name_to_num(name) {
            Some(num) => Some(NoteMessage::from_num(num)),
            None => None
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct NoteEvent {
    pub id: Id,
    pub time: f64,
    pub note: Event
}

pub fn num_to_pitch(num: u32) -> f32 {
    440.0 * 2.0_f32.powf((num as f32 - 69.0) / 12.0)
}

pub fn pitch_to_num(pitch: f32) -> u32 {
    (f32::round(f32::log2(pitch / 440.0) * 12.0) + 69.0) as u32
}

pub fn name_to_num(name: &str) -> Option<u32> {
    let mut i = 12;
    for n in NOTE_NAMES {
        if n == name {
            return Some(i);
        }

        i += 1;
    }

    return None;
}

/* Iterator ideas */

// Chromatic iterator
// Scale iterator
// Filter by note name
// Filter by interval

/* Implement operations */

pub struct NoteQueued {
    voice_index: u32,
    message: NoteMessage
}

pub struct NotePlaying {
    voice_index: u32,
    id: Id,
    pitch: f32
}

pub struct NotePlayer {
    queue: Vec<NoteQueued>, // Queue of notes to play
    playing: Vec<NotePlaying>, // Playing voice index
    max_voice: u32 // Maximum playable voice
}

impl NotePlayer {
    pub fn new() -> Self {
        Self {
            queue: Vec::with_capacity(64),
            playing: Vec::with_capacity(64),
            max_voice: 16
        }
    }

    pub fn note_on(&mut self, id: Id, pitch: f32, pressure: f32) {
        for i in 0..self.max_voice {
            fn contains(playing: &Vec<NotePlaying>, voice: u32) -> bool {
                for e in playing {
                    if e.voice_index == voice {
                        return true;
                    }
                }
                return false;
            }

            fn contains2(playing: &Vec<NoteQueued>, voice: u32) -> bool {
                for e in playing {
                    if e.voice_index == voice {
                        return true;
                    }
                }
                return false;
            }

            if !contains(&self.playing, i) && !contains2(&self.queue, i){
                self.queue.push(
                    NoteQueued {
                        voice_index: i,
                        message: NoteMessage {
                            id,
                            offset: 0,
                            note: Event::NoteOn {
                                pitch,
                                pressure
                            }
                        }
                    }
                );

                break;
            }
        }
    }

    pub fn note_num_on(&mut self, num: u32, pressure: f32) {
        self.note_on(Id::new(), num_to_pitch(num), pressure);
    }

    pub fn note_off(&mut self, id: Id) {
        for playing in &self.playing {
            if playing.id == id {
                // println!("Note off");
                self.queue.push(
                    NoteQueued {
                        voice_index: playing.voice_index,
                        message: NoteMessage {
                            id,
                            offset: 0,
                            note: Event::NoteOff
                        }
                    }
                );
            }
        }

        self.playing.retain(| playing | {
            playing.id != id
        });
    }

    pub fn note_num_off(&mut self, num: u32) {
        for i in 0..self.playing.len() {
            // println!("Comparing to num {} to playing num {}", num, pitch_to_num(self.playing[i].pitch));
            if pitch_to_num(self.playing[i].pitch) == num {
                self.note_off(self.playing[i].id);
            }
        }

        self.queue.retain( | queued | {
            match queued.message.note {
                Event::NoteOn { pitch, pressure: _ } => {
                    if pitch_to_num(pitch) == num {
                        // println!("Removing queued note");
                        false
                    } else {
                        true
                    }
                },
                _ => true
            }
        });
    }

    pub fn message(&mut self, message: NoteMessage) {
        match message.note {
            Event::NoteOn { pitch, pressure } => {
                self.note_on(message.id, pitch, pressure);
            },
            Event::NoteOff => {
                self.note_off(message.id);
            },
            _ => {
                for playing in &self.playing {
                    if playing.id == message.id {
                        self.queue.push(
                            NoteQueued {
                                voice_index: playing.voice_index,
                                message
                            }
                        );
                    }
                }
            }
        }
    }

    pub fn generate(&mut self, voice: u32, output: &mut Buffer<NoteMessage>) {
        for queued in &self.queue {
            if queued.voice_index == voice {
                let message = queued.message;

                match message.note {
                    Event::NoteOn { pitch, pressure: _ } => {
                        self.playing.push(
                            NotePlaying {
                                voice_index: queued.voice_index,
                                id: message.id,
                                pitch: pitch
                            }
                        );
                    },
                    Event::NoteOff => {
                        self.playing.retain( | playing | {
                            playing.id != message.id
                        });
                    },
                    Event::Pitch(pitch) => {
                        for playing in &mut self.playing {
                            if playing.id == message.id {
                                playing.pitch = pitch;
                            }
                        }
                    },
                    _ => ()
                }

                output.push(queued.message);
            }
        }

        self.queue.retain(| queued | queued.voice_index != voice);
    }
}
