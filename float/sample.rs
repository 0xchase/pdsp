use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

use crate::float::float::*;

pub trait Sample: Copy + Clone + From<Self::Float>
    + PartialEq
    + Add<Self, Output = Self> + Sub<Self, Output = Self> + Mul<Self, Output = Self> + Div<Self, Output = Self>
    + Add<Self::Float, Output = Self> + Sub<Self::Float, Output = Self> + Mul<Self::Float, Output = Self> + Div<Self::Float, Output = Self>
    + AddAssign + SubAssign + MulAssign + DivAssign {

    type Float: Float;
    const CHANNELS: usize;
    const EQUILIBRIUM: Self;

    fn from_f32(v: f32) -> Self;
    fn from_usize(v: usize) -> Self;

    fn apply<Function: Fn(Self::Float) -> Self::Float>(self, f: Function) -> Self where Self: Sized;

    fn mono(self) -> Self::Float;

    fn sin(self) -> Self {
        self.apply(Float::sin)
    }

    fn cos(self) -> Self {
        Self::apply(self, Float::cos)
    }

    fn tan(self) -> Self {
        Self::apply(self, Float::tan)
    }

    fn atan(self) -> Self {
        Self::apply(self, Float::atan)
    }

    fn sqrt(self) -> Self {
        todo!()
    }

    fn gain(&self, db: Self::Float) -> Self {
        let linear = Self::Float::powf(
            Self::Float::from(10.0),
            db / Self::Float::from(10.0));

        self.apply(| v | v * linear)
    }

    fn powf(self, e: Self) -> Self;
    fn min(self, rhs: Self) -> Self;
    fn max(self, rhs: Self) -> Self;
}

impl Sample for f32 {
    type Float = f32;

    const CHANNELS: usize = 1;
    const EQUILIBRIUM: Self = Self::ZERO;

    fn from_f32(v: f32) -> Self {
        v
    }

    fn from_usize(v: usize) -> Self {
        v as f32
    }

    fn apply<Function: Fn(Self::Float) -> Self::Float>(self, f: Function) -> Self where Self: Sized {
        f(self)
    }

    fn mono(self) -> Self::Float {
        self
    }

    fn powf(self, e: Self) -> Self {
        Float::powf(self, e)
    }

    fn min(self, rhs: Self) -> Self {
        Float::min(self, rhs)
    }

    fn max(self, rhs: Self) -> Self {
        Float::min(self, rhs)
    }
}

impl Sample for f64 {
    type Float = f64;

    const CHANNELS: usize = 1;
    const EQUILIBRIUM: Self = Self::ZERO;

    fn from_f32(v: f32) -> Self {
        v as f64
    }

    fn from_usize(v: usize) -> Self {
        v as f64
    }

    fn apply<Function: Fn(Self::Float) -> Self::Float>(self, f: Function) -> Self where Self: Sized {
        f(self)
    }

    fn mono(self) -> Self::Float {
        self
    }

    fn powf(self, e: Self) -> Self {
        Float::powf(self, e)
    }

    fn min(self, rhs: Self) -> Self {
        Float::min(self, rhs)
    }

    fn max(self, rhs: Self) -> Self {
        Float::min(self, rhs)
    }
}
