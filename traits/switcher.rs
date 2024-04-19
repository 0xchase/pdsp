use crate::Generator;

pub struct Switcher<const C: usize, G: Generator> {
    v: [G; C],
    index: usize
}

impl<const C: usize, G: Generator> Switcher<C, G> {
    pub fn from(v: [G; C]) -> Self {
        Self {
            v,
            index: 0
        }
    }

    pub fn switch(&mut self, index: usize) {
        self.index = index;
    }
}

impl<const C: usize, G: Generator> Generator for Switcher<C, G> {
    type Output = G::Output;

    fn reset(&mut self) {
        for v in self.v.iter_mut() {
            v.reset();
        }
    }

    fn prepare(&mut self, sample_rate: u32, block_size: usize) {
        for v in self.v.iter_mut() {
            v.prepare(sample_rate, block_size);
        }
    }

    fn generate(&mut self) -> Self::Output {
        self.v[self.index].generate()
    }
} 
