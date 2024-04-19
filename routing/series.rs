use crate::traits::*;
use crate::float::sample::Sample;

#[derive(Copy, Clone)]
pub struct Series<F: Sample, A: Processor<Input = F, Output = F>, const C: usize>(pub [A; C]);

impl<F: Sample, A: Processor<Input = F, Output = F>, const C: usize> Processor for Series<F, A, C> {
    type Input = F;
    type Output = F;

    fn reset(&mut self) {}

    fn prepare(&mut self, _sample_rate: u32, _block_size: usize) {}

    fn process(&mut self, input: F) -> F {
        let mut v = input;
        for p in &mut self.0 {
           v = p.process(v);
        }

        return v;
    }
}
