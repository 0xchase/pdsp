use crate::buffers::*;

pub type GeneratorGraph<Out> = Box<dyn Generator<Output = Out>>;
pub type ProcessorGraph<In, Out> = Box<dyn Processor<Input = In, Output = Out>>;

pub trait Generator {
    type Output;

    fn reset(&mut self);
    fn prepare(&mut self, sample_rate: u32, block_size: usize);
    fn generate(&mut self) -> Self::Output;
}

impl Generator for f32 {
    type Output = f32;

    fn reset(&mut self) {}
    fn prepare(&mut self, _sample_rate: u32, _block_size: usize) {}

    #[inline]
    fn generate(&mut self) -> Self::Output {
        *self
    }
}

impl Generator for f64 {
    type Output = f64;

    fn reset(&mut self) {}
    fn prepare(&mut self, _sample_rate: u32, _block_size: usize) {}

    #[inline]
    fn generate(&mut self) -> Self::Output {
        *self
    }
}

pub trait BlockGenerator {
    type Output;

    fn generate_block<OutBuffer: Block<Item = Self::Output>>(&mut self, output: &mut OutBuffer);
}

impl<Out, G: Generator<Output = Out>> BlockGenerator for G {
    type Output = Out;

    fn generate_block<OutBuffer: Block<Item = Self::Output>>(&mut self, output: &mut OutBuffer) {
        for dest in output.as_slice_mut().iter_mut() {
            *dest = self.generate();
        }
    }
}

pub trait Processor {
    type Input;
    type Output;

    fn reset(&mut self);
    fn prepare(&mut self, sample_rate: u32, block_size: usize);
    fn process(&mut self, input: Self::Input) -> Self::Output;
}

pub trait BlockProcessor {
    type Input;
    type Output;

    fn process_block<InBuffer, OutBuffer>(&mut self, input: &InBuffer, output: &mut OutBuffer)
        where
            InBuffer: Block<Item = Self::Input>,
            OutBuffer: Block<Item = Self::Output>;
}

impl<In: Copy, Out: Copy, P: Processor<Input = In, Output = Out>> BlockProcessor for P {
    type Input = In;
    type Output = Out;

    fn process_block<InBuffer: Block<Item = In>, OutBuffer: Block<Item = Out>>(&mut self, input: &InBuffer, output: &mut OutBuffer) {
        for (dest, src) in output.as_slice_mut().iter_mut().zip(input.as_slice()) {
            *dest = self.process(*src);
        }
    }
}
