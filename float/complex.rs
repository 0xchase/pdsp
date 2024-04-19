use std::ops::{Mul, Add};

use crate::Float;

pub const fn complex<F: Float>(real: F, imaginary: F) -> Complex<F> {
    Complex { real, imaginary }
}

pub type Complex32 = Complex<f32>;
pub type Complex64 = Complex<f32>;

#[derive(Copy, Clone, PartialEq)]
pub struct Complex<F: Float> {
    pub real: F,
    pub imaginary: F
}

impl<F: Float> Complex<F> {
    pub const ZERO: Complex<F> = Self {
        real: F::ZERO,
        imaginary: F::ZERO
    };

    pub fn new() -> Self {
        Self {
            real: F::ZERO,
            imaginary: F::ZERO
        }
    }

    pub fn from(real: F, imaginary: F) -> Self {
        Self { real, imaginary }
    }
}

impl<F: Float> Mul for Complex<F> {
    type Output = Complex<F>;

    fn mul(self, rhs: Self) -> Self::Output {
        let ac = self.real * rhs.real;
        let adi = self.real * rhs.imaginary;
        let bci = self.imaginary * rhs.real;
        let bd = self.imaginary * rhs.imaginary;

        Self {
            real: ac + bd,
            imaginary: adi + bci
        }
    }
}

impl<F: Float> Add for Complex<F> {
    type Output = Complex<F>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary
        }
    }
}
