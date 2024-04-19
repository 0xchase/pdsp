use std::slice;
use std::ops::{Add, Mul, Sub, Div, Index, IndexMut, AddAssign, SubAssign, MulAssign, DivAssign};

use crate::{event::*, Stereo};

use crate::float::sample::*;
use crate::buffers::block::*;

pub type AudioBuffer = Buffer<f32>;
pub type StereoBuffer = Buffer<Stereo<f32>>;
pub type NoteBuffer = Buffer<NoteMessage>;

pub struct Buffer<T> {
    items: Vec<T>,
}

impl<T> Buffer<T> {
    pub fn from(items: Vec<T>) -> Self {
        Self { items }
    }

    pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
        Self {
            items: Vec::from_raw_parts(ptr, length, capacity),
        }
    }

    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }

    pub fn as_ptr(&self) -> *const T {
        self.items.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.items.as_mut_ptr()
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl<T: Copy> Buffer<T> {
    pub fn init(value: T, size: usize) -> Self {
        let mut items = Vec::with_capacity(size);

        for _ in 0..size {
            items.push(value);
        }

        Self { items }
    }
}

impl<T> Block for Buffer<T> {
    type Item = T;

    fn as_slice<'a>(&'a self) -> &'a [T] {
        self.items.as_slice()
    }

    fn as_slice_mut<'a>(&'a mut self) -> &'a mut [T] {
        self.items.as_mut_slice()
    }
}

// Replace letters with useful names
impl Buffer<NoteMessage> {
    pub fn new() -> Self {
        Self {
            items: Vec::new()
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity)
        }
    }

    pub fn replace(&mut self, src: &Buffer<NoteMessage>) {
        self.items.clear();

        for s in src.as_slice() {
            self.items.push(*s);
        }
    }

    pub fn append(&mut self, src: &Buffer<NoteMessage>) {
        for s in src.as_slice() {
            self.items.push(*s);
        }
    }
}

/* Opeator assign implementations */

impl<T: Add<Output = T> + Copy, B: Block<Item = T>> AddAssign<&B> for Buffer<T> {
    fn add_assign(&mut self, rhs: &B) {
        for (a, b) in self.as_slice_mut().iter_mut().zip(rhs.as_slice()) {
            *a = *a + *b;
        }
    }
}

impl<T: Sub<Output = T> + Copy, B: Block<Item = T>> SubAssign<&B> for Buffer<T> {
    fn sub_assign(&mut self, rhs: &B) {
        for (a, b) in self.as_slice_mut().iter_mut().zip(rhs.as_slice()) {
            *a = *a - *b;
        }
    }
}

impl<T: Mul<Output = T> + Copy, B: Block<Item = T>> MulAssign<&B> for Buffer<T> {
    fn mul_assign(&mut self, rhs: &B) {
        for (a, b) in self.as_slice_mut().iter_mut().zip(rhs.as_slice()) {
            *a = *a * *b;
        }
    }
}

impl<T: Div<Output = T> + Copy, B: Block<Item = T>> DivAssign<&B> for Buffer<T> {
    fn div_assign(&mut self, rhs: &B) {
        for (a, b) in self.as_slice_mut().iter_mut().zip(rhs.as_slice()) {
            *a = *a / *b;
        }
    }
}

/* Index trait implementations */

impl<T: Copy> Index<usize> for Buffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<T: Copy> IndexMut<usize> for Buffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}

/* Iterator trait implementations */

impl<'a, T: Copy + Clone> IntoIterator for &'a Buffer<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> slice::Iter<'a, T> {
        self.as_slice().into_iter()
    }
}

impl<'a, T: Copy + Clone> IntoIterator for &'a mut Buffer<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> slice::IterMut<'a, T> {
        self.as_slice_mut().into_iter()
    }
}

/* Ring buffer - here temporarily until private types are unneeded */

pub struct RingBuffer<S> {
    buffer: Buffer<S>,
    length: usize,
    index: usize
}

impl<S: Sample> RingBuffer<S> {
    pub fn init(value: S, size: usize) -> Self {
        Self {
            buffer: Buffer::init(value, size),
            length: 1,
            index: 0
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    pub fn resize(&mut self, length: usize) {
        if length > self.buffer.capacity() {
            self.buffer.items.resize(length, S::EQUILIBRIUM);
        } else {
            self.length = length;
            for sample in self.buffer.as_slice_mut().iter_mut().skip(length) {
                *sample = S::EQUILIBRIUM;
            }
        }
    }

    pub fn next(&mut self, input: S) -> S {
        let output = self.buffer.items[self.index];
        self.buffer.items[self.index] = input;
        self.index = (self.index + 1) % self.length;
        output
    }
}
