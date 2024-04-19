use crate::Block;
use crate::traits::*;
use crate::routing::node::*;

/*pub const fn output_block<'a, B: Block>(b: &'a B) -> AudioNode<Output<'a, B>> {
    AudioNode(Output { b, i: 0})
}

#[derive(Copy, Clone)]
pub struct Output<'a, B: Block> {
    b: &'a B,
    i: usize
}

impl<'a, F: Copy, B: Block<Item = F>> Generator for Output<'a, B> {
    type Output = B::Item;

    fn reset(&mut self) {}
    fn prepare(&mut self, _sample_rate: u32, _block_size: usize) {}

    fn generate(&mut self) -> Self::Output {
        let f = self.b.as_slice()[self.i];
        self.i += 1;
        return f;
    }
}*/

/*impl<Between, Out, G, P> Generator for Chain<G, P> 
    where
        G: Block<Item = Between>,
        P: Processor<Output = Between, Output = Out> {

    type Output = Out;

    fn reset(&mut self) {}
    fn prepare(&mut self, _sample_rate: u32, _block_size: usize) {}

    fn generate(&mut self) -> Self::Output {
        self.1.process(self.0.generate())
    }
}*/

/*impl<A, B> std::ops::Shr<AudioNode<B>> for AudioNode<A> {
    type Output = AudioNode<Output<A, B>>;

    fn shr(self, rhs: AudioNode<B>) -> Self::Output {
        AudioNode(Output(self.0, rhs.0))
    }
}*/
