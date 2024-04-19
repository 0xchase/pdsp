use crate::Block;
use crate::traits::*;
use crate::routing::node::*;

pub const fn input_block<'a, B: Block>(b: &'a B) -> AudioNode<Input<'a, B>> {
    AudioNode(Input { b, i: 0})
}

#[derive(Copy, Clone)]
pub struct Input<'a, B: Block> {
    b: &'a B,
    i: usize
}

impl<'a, F: Copy, B: Block<Item = F>> Generator for Input<'a, B> {
    type Output = B::Item;

    fn reset(&mut self) {}
    fn prepare(&mut self, _sample_rate: u32, _block_size: usize) {}

    fn generate(&mut self) -> Self::Output {
        let f = self.b.as_slice()[self.i];
        self.i += 1;
        return f;
    }
}

/*impl<Between, Out, G, P> Generator for Chain<G, P> 
    where
        G: Block<Item = Between>,
        P: Processor<Input = Between, Output = Out> {

    type Output = Out;

    fn reset(&mut self) {}
    fn prepare(&mut self, _sample_rate: u32, _block_size: usize) {}

    fn generate(&mut self) -> Self::Output {
        self.1.process(self.0.generate())
    }
}*/

impl<'a, B: Block, P> std::ops::Shr<AudioNode<P>> for Input<'a, B> {
    type Output = AudioNode<InputChain<'a, B, P>>;

    fn shr(self, rhs: AudioNode<P>) -> Self::Output {
        AudioNode(InputChain(self.b, rhs.0))
    }
}

#[derive(Copy, Clone)]
pub struct InputChain<'a, B: Block, P>(&'a B, P);

impl<'a, Between, Out, G, P> Generator for InputChain<'a, G, P>
    where
        G: Block<Item = Between>,
        P: Processor<Input = Between, Output = Out> {

    type Output = Out;

    fn reset(&mut self) {}
    fn prepare(&mut self, _sample_rate: u32, _block_size: usize) {}

    fn generate(&mut self) -> Self::Output {
        todo!()
    }
}
