use crate::traits::*;
use crate::routing::node::*;
use crate::float::*;

pub fn parallel<F: Sample, G: Sample, H: Sample, J: Sample, A: Processor<Input = F, Output = G>, B: Processor<Input = H, Output = J>>(first: A, second: B) -> AudioNode<Parallel<A, B>> {
    AudioNode(Parallel(first, second))
}

#[derive(Copy, Clone)]
pub struct Parallel<A, B>(pub A, pub B);

impl<F, G, H, J, A, B> Processor for Parallel<A, B>
    where
        A: Processor<Input = F, Output = G>,
        B: Processor<Input = H, Output = J> {

    type Input = (F, H);
    type Output = (G, J);

    fn reset(&mut self) {}

    fn prepare(&mut self, sample_rate: u32, block_size: usize) {
        self.0.prepare(sample_rate, block_size);
        self.1.prepare(sample_rate, block_size);
    }

    fn process(&mut self, input: Self::Input) -> Self::Output {
        (self.0.process(input.0), self.1.process(input.1))
    }
}
