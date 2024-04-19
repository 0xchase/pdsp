use std::ops::{Add, Sub, Mul, Div};

use crate::Sample;

pub trait Block {
    type Item;

    fn as_slice<'a>(&'a self) -> &'a [Self::Item];
    fn as_slice_mut<'a>(&'a mut self) -> &'a mut [Self::Item];

    fn len(&self) -> usize {
        self.as_slice().len()
    }

    fn copy_to<B: Block<Item = Self::Item>>(&self, dest: &mut B) where Self: Sized, Self::Item: Copy {
        dest.copy_from(self);
    }

    fn rms(&self) -> Self::Item where Self::Item: Sample {
        let mut total = Self::Item::EQUILIBRIUM;
        let mut count = Self::Item::EQUILIBRIUM;

        for s in self.as_slice() {
            total += *s;
            count += Self::Item::from_f32(1.0);
        }

        total = total / count;

        return total;
    }

    fn apply<F: Fn(Self::Item) -> Self::Item>(&mut self, f: F) where Self::Item: Copy {
        for v in self.as_slice_mut() {
            *v = f(*v);
        }
    }

    fn zip_apply<I: Copy, B: Block<Item = I>, F: Fn(Self::Item, I) -> Self::Item>(&mut self, b: &B, f: F) where Self::Item: Copy {
        for (v, i) in self.as_slice_mut().iter_mut().zip(b.as_slice()) {
            *v = f(*v, *i);
        }
    }

    /* Element operations */

    fn fill(&mut self, value: Self::Item) where Self::Item: Copy {
        self.apply(| _v | { value } );
    }

    /* Block operations */

    fn copy_from<B: Block<Item = Self::Item>>(&mut self, src: &B) where Self::Item: Copy {
        self.as_slice_mut().copy_from_slice(src.as_slice());
    }

    fn add_from<B: Block<Item = Self::Item>>(&mut self, src: &B) where Self::Item: Copy + Add<Output = Self::Item> {
        self.zip_apply(src, | a, b | a + b);
    }

    fn sub_from<B: Block<Item = Self::Item>>(&mut self, src: &B) where Self::Item: Copy + Sub<Output = Self::Item> {
        self.zip_apply(src, | a, b | a - b);
    }

    fn mul_from<B: Block<Item = Self::Item>>(&mut self, src: &B) where Self::Item: Copy + Mul<Output = Self::Item> {
        self.zip_apply(src, | a, b | a * b);
    }

    fn div_from<B: Block<Item = Self::Item>>(&mut self, src: &B) where Self::Item: Copy + Div<Output = Self::Item> {
        self.zip_apply(src, | a, b | a / b);
    }

    /* Sample specific */

    fn gain(&mut self, db: <Self::Item as Sample>::Float) where Self::Item: Sample {
        self.apply(| v | v.gain(db))
    }

    fn equilibrate(&mut self) where Self::Item: Sample {
        self.fill(Self::Item::EQUILIBRIUM);
    }

    /* Note specific */

    // fn transpose
}

/* Slice implementations */

impl<S> Block for [S] {
    type Item = S;

    fn as_slice<'a>(&'a self) -> &'a [Self::Item] {
        self
    }

    fn as_slice_mut<'a>(&'a mut self) -> &'a mut [Self::Item] {
        self
    }
}

impl<S> Block for &[S] {
    type Item = S;

    fn as_slice<'a>(&'a self) -> &'a [Self::Item] {
        self
    }

    fn as_slice_mut<'a>(&'a mut self) -> &'a mut [Self::Item] {
        unreachable!()
    }
}

impl<S> Block for &mut [S] {
    type Item = S;

    fn as_slice<'a>(&'a self) -> &'a [Self::Item] {
        self
    }

    fn as_slice_mut<'a>(&'a mut self) -> &'a mut [Self::Item] {
        self
    }
}


/* Raw pointer implementations */

impl<S> Block for (*const S, usize) {
    type Item = S;

    fn as_slice<'a>(&'a self) -> &'a [Self::Item] {
        unsafe {
            std::slice::from_raw_parts(self.0, self.1)
        }
    }

    fn as_slice_mut<'a>(&'a mut self) -> &'a mut [Self::Item] {
        unreachable!()
    }
}

impl<S> Block for (*mut S, usize) {
    type Item = S;

    fn as_slice<'a>(&'a self) -> &'a [Self::Item] {
        unsafe {
            std::slice::from_raw_parts(self.0, self.1)
        }
    }

    fn as_slice_mut<'a>(&'a mut self) -> &'a mut [Self::Item] {
        unsafe {
            std::slice::from_raw_parts_mut(self.0, self.1)
        }
    }
}
