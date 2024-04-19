use crate::traits::*;

pub const fn param(name: &'static str, value: f32) -> Parameter {
    Parameter { name, value }
}

pub trait Param {
    // type Value;

    fn set_param(&mut self, name: &'static str, value: f32);
}

#[derive(Copy, Clone)]
pub struct Parameter {
    name: &'static str,
    value: f32
}

impl Param for Parameter {
    fn set_param(&mut self, name: &'static str, value: f32) {
        if name == self.name {
            self.value = value;
        }
    }
}

impl Generator for Parameter {
    type Output = f32;

    fn reset(&mut self) {}
    fn prepare(&mut self, _sample_rate: u32, _block_size: usize) {}

    fn generate(&mut self) -> Self::Output {
        self.value
    }
}
