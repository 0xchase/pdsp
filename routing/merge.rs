use std::ops::Add;

use crate::traits::*;
use crate::routing::node::*;

pub fn merge<In, Out: TupleMerge<Output = Merged>, Merged, P>(processor: P) -> AudioNode<Merge<In, Out, Merged, P>>
    where
        P: Processor<Input = In, Output = Out>
{
    AudioNode(Merge(processor))
}

pub trait TupleMerge {
    type Output;

    fn merge(self) -> Self::Output;
}

impl<F: Add<Output = F>> TupleMerge for (F, F) {
    type Output = F;

    fn merge(self) -> Self::Output {
        self.0 + self.1
    }
}

impl<F: Add<Output = F>> TupleMerge for (F, F, F) {
    type Output = F;

    fn merge(self) -> Self::Output {
        self.0 + self.1 + self.2
    }
}

impl<F: Add<Output = F>> TupleMerge for (F, F, F, F) {
    type Output = F;

    fn merge(self) -> Self::Output {
        self.0 + self.1 + self.2 + self.3
    }
}

#[derive(Copy, Clone)]
pub struct Merge<In, Out, Merged, P>(pub P)
    where
        Out: TupleMerge<Output = Merged>,
        P: Processor<Input = In, Output = Out>;

impl<In, Out, Merged, P> Processor for Merge<In, Out, Merged, P> 
    where
        Out: TupleMerge<Output = Merged>,
        P: Processor<Input = In, Output = Out> {

    type Input = In;
    type Output = Merged;

    fn reset(&mut self) {}

    fn prepare(&mut self, sample_rate: u32, block_size: usize) {
        self.0.prepare(sample_rate, block_size);
    }

    fn process(&mut self, input: Self::Input) -> Self::Output {
        self.0.process(input).merge()
    }
}
