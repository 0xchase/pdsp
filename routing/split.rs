use crate::traits::*;
use crate::routing::node::*;
use crate::float::*;

pub fn split<In, Out, P>(processor: P) -> AudioNode<Split<In, Out, P>>
    where
        In: Sample,
        Out: Sample,
        P: Processor<Input = In, Output = Out>
{
    AudioNode(Split(processor))
}

#[derive(Copy, Clone)]
pub struct Split<In, Out, P>(pub P)
    where
        P: Processor<Input = In, Output = Out>;

impl<In, Out, P> Processor for Split<In, Out, P> 
    where
        Out: Copy,
        P: Processor<Input = In, Output = Out> {

    type Input = In;
    type Output = (Out, Out);

    fn reset(&mut self) {}

    fn prepare(&mut self, sample_rate: u32, block_size: usize) {
        self.0.prepare(sample_rate, block_size);
    }

    fn process(&mut self, input: Self::Input) -> Self::Output {
        let output = self.0.process(input);
        (output, output)
    }
}
