use std::marker::PhantomData;

use crate::traits::*;
use crate::float::sample::*;
use crate::routing::param::*;
use crate::routing::chain::*;
use crate::routing::merge::*;
use crate::routing::parallel::*;

#[derive(Copy, Clone)]
pub struct AudioNode<P>(pub P);

impl<P> std::ops::Deref for AudioNode<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P> std::ops::DerefMut for AudioNode<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<P: Param> Param for AudioNode<P> {
    fn set_param(&mut self, name: &'static str, value: f32) {
        self.0.set_param(name, value);
    } 
}

impl<Out, G> Generator for AudioNode<G>
    where
        G: Generator<Output = Out> {

    type Output = Out;

    fn reset(&mut self) {}
    fn prepare(&mut self, sample_rate: u32, block_size: usize) {}

    fn generate(&mut self) -> Self::Output {
        self.0.generate()
    }
}

impl<In, Out, P> Processor for AudioNode<P>
    where
        P: Processor<Input = In, Output = Out> {

    type Input = In;
    type Output = Out;

    fn reset(&mut self) {}

    fn prepare(&mut self, sample_rate: u32, block_size: usize) {
        self.0.prepare(sample_rate, block_size);
    }

    fn process(&mut self, input: Self::Input) -> Self::Output {
        self.0.process(input)
    }
}

impl<A, B> std::ops::BitOr<AudioNode<B>> for AudioNode<A> {
    type Output = AudioNode<Parallel<A, B>>;

    fn bitor(self, rhs: AudioNode<B>) -> Self::Output {
        AudioNode(Parallel(self.0, rhs.0))
    }
}

impl<In1, Out1, Merged, Out2, P1, P2> std::ops::BitAnd<AudioNode<P2>> for AudioNode<P1> 
    where
        Out1: TupleMerge<Output = Merged>,
        P1: Processor<Input = In1, Output = Out1>,
        P2: Processor<Input = Merged, Output = Out2> {

    type Output = AudioNode<Chain<Merge<In1, Out1, Merged, P1>, P2>>;

    fn bitand(self, rhs: AudioNode<P2>) -> Self::Output {
        AudioNode(Chain(Merge(self.0), rhs.0))
    }
}
