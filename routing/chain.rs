use crate::Block;
use crate::float::*;
use crate::traits::*;
use crate::routing::node::*;

pub fn chain<F: Sample, G: Sample, H: Sample, A: Processor<Input = F, Output = G>, B: Processor<Input = G, Output = H>>(first: A, second: B) -> AudioNode<Chain<A, B>> {
    AudioNode(Chain(first, second))
}

#[derive(Copy, Clone)]
pub struct Chain<P1, P2>(pub P1, pub P2);

impl<In, Between, Out, P1, P2> Processor for Chain<P1, P2> 
    where
        P1: Processor<Input = In, Output = Between>,
        P2: Processor<Input = Between, Output = Out> {

    type Input = In;
    type Output = Out;

    fn reset(&mut self) {}

    fn prepare(&mut self, sample_rate: u32, block_size: usize) {
        self.0.prepare(sample_rate, block_size);
        self.1.prepare(sample_rate, block_size);
    }

    fn process(&mut self, input: Self::Input) -> Self::Output {
        self.1.process(self.0.process(input))
    }
}

impl<Between, Out, G, P> Generator for Chain<G, P> 
    where
        G: Generator<Output = Between>,
        P: Processor<Input = Between, Output = Out> {

    type Output = Out;

    fn reset(&mut self) {}
    fn prepare(&mut self, _sample_rate: u32, _block_size: usize) {}

    fn generate(&mut self) -> Self::Output {
        self.1.process(self.0.generate())
    }
}

impl<A, B> std::ops::Shr<AudioNode<B>> for AudioNode<A> {
    type Output = AudioNode<Chain<A, B>>;

    fn shr(self, rhs: AudioNode<B>) -> Self::Output {
        AudioNode(Chain(self.0, rhs.0))
    }
}

// Chain to mutable block
impl<S, A: Generator<Output = S>, B: Block<Item = S>> std::ops::Shr<&mut B> for AudioNode<A> {
    type Output = ();

    fn shr(mut self, rhs: &mut B) -> Self::Output {
        self.generate_block(rhs);
    }
}
