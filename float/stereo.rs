use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

use crate::float::float::*;
use crate::float::sample::*;

/// Stereo float type
#[derive(Copy, Clone, Default, PartialEq)]
pub struct Stereo<T> {
    pub left: T,
    pub right: T,
}

impl<F: Float> Add<Self> for Stereo<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Stereo {
            left: self.left + rhs.left,
            right: self.right + rhs.right
        }
    }
}

impl<F: Float> Sub<Self> for Stereo<F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Stereo {
            left: self.left - rhs.left,
            right: self.right - rhs.right
        }
    }
}

impl<F: Float> Mul<Self> for Stereo<F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Stereo {
            left: self.left * rhs.left,
            right: self.right * rhs.right
        }
    }
}

impl<F: Float> Div<Self> for Stereo<F> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Stereo {
            left: self.left / rhs.left,
            right: self.right / rhs.right
        }
    }
}

impl<F: Float> AddAssign<Self> for Stereo<F> {
    fn add_assign(&mut self, rhs: Self) {
        self.left = self.left + rhs.left;
        self.right = self.right + rhs.right;
    }
}

impl<F: Float> SubAssign<Self> for Stereo<F> {
    fn sub_assign(&mut self, rhs: Self) {
        self.left = self.left - rhs.left;
        self.right = self.right - rhs.right;
    }
}

impl<F: Float> MulAssign<Self> for Stereo<F> {
    fn mul_assign(&mut self, rhs: Self) {
        self.left = self.left * rhs.left;
        self.right = self.right * rhs.right;
    }
}

impl<F: Float> DivAssign<Self> for Stereo<F> {
    fn div_assign(&mut self, rhs: Self) {
        self.left = self.left / rhs.left;
        self.right = self.right / rhs.right;
    }
}

impl<F: Float> Add<F> for Stereo<F> {
    type Output = Self;

    fn add(self, rhs: F) -> Self::Output {
        Stereo {
            left: self.left + rhs,
            right: self.right + rhs
        }
    }
}

impl<F: Float> Sub<F> for Stereo<F> {
    type Output = Self;

    fn sub(self, rhs: F) -> Self::Output {
        Stereo {
            left: self.left - rhs,
            right: self.right - rhs
        }
    }
}

impl<F: Float> Mul<F> for Stereo<F> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        Stereo {
            left: self.left * rhs,
            right: self.right * rhs
        }
    }
}

impl<F: Float> Div<F> for Stereo<F> {
    type Output = Self;

    fn div(self, rhs: F) -> Self::Output {
        Stereo {
            left: self.left / rhs,
            right: self.right / rhs
        }
    }
}

impl<F: Float> Sample for Stereo<F> {
    type Float = F;

    const CHANNELS: usize = 2;
    const EQUILIBRIUM: Self = Self {
        left: F::ZERO,
        right: F::ZERO
    };

    fn from_f32(v: f32) -> Self {
        Self::from(F::from(v))
    }

    fn from_usize(v: usize) -> Self {
        Self::from(F::from_usize(v))
    }

    fn apply<Function: Fn(Self::Float) -> Self::Float>(self, f: Function) -> Self where Self: Sized {
        Self {
            left: f(self.left),
            right: f(self.right),
        }
    }

    fn mono(self) -> Self::Float {
        Float::avg(self.left, self.right)
    }

    fn powf(self, e: Self) -> Self {
        Self {
            left: Float::powf(self.left, e.left),
            right: Float::powf(self.right, e.right),
        }
    }

    fn min(self, rhs: Self) -> Self {
        Self {
            left: Float::min(self.left, rhs.left),
            right: Float::min(self.right, rhs.right),
        }
    }

    fn max(self, rhs: Self) -> Self {
        Self {
            left: Float::max(self.left, rhs.left),
            right: Float::max(self.right, rhs.right),
        }
    }
}

impl<F: Float> From<F> for Stereo<F> {
    fn from(v: F) -> Self {
        Self {
            left: v,
            right: v
        }
    }
}