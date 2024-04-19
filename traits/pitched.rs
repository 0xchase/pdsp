use crate::{Block, NoteMessage};

pub trait Pitched {
    fn get_pitch(&self) -> f32;
    fn set_pitch(&mut self, hz: f32);

    /*fn update_pitch<B: Block<Item = NoteMessage>>(&mut self, block: &B) {
        for item in block.as_slice() {
            match item.note {
                crate::Event::NoteOn { pitch, pressure: _ } => {
                    self.set_pitch(pitch);
                },
                crate::Event::NoteOff => (),
                crate::Event::Pitch(hz) => {
                    self.set_pitch(hz);
                },
                crate::Event::Pressure(_) => (),
                crate::Event::Other(_, _) => (),
            }
        }
    }*/
}
