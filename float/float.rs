use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

pub trait Float: Copy + Clone + From<f32>
    + PartialEq + PartialOrd
    + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
    + AddAssign + SubAssign + MulAssign + DivAssign {

    const ZERO: Self;
    const MIN: Self;
    const MAX: Self;
    const PI: Self;

    fn from_usize(v: usize) -> Self;

    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn atan(self) -> Self;

    fn powf(self, e: Self) -> Self;
    fn avg(self, v: Self) -> Self;
    fn log(self, v: Self) -> Self;
    fn log10(self) -> Self;

    fn min(self, v: Self) -> Self {
        if self < v { self } else { v }
    }

    fn max(self, v: Self) -> Self {
        if self > v { self } else { v }
    }
}

impl Float for f32 {
    const ZERO: Self = 0.0;
    const MIN: Self = -1.0;
    const MAX: Self = 1.0;
    const PI: Self = std::f32::consts::PI;

    fn from_usize(v: usize) -> Self {
        v as f32
    }

    fn sin(self) -> Self {
        f32::sin(self)
    }

    fn cos(self) -> Self {
        f32::cos(self)
    }

    fn tan(self) -> Self {
        f32::tan(self)
    }

    fn atan(self) -> Self {
        f32::atan(self)
    }

    fn avg(self, v: Self) -> Self {
        (self + v) / 2.0
    }

    fn powf(self, e: Self) -> Self {
        f32::powf(self, e)
    }

    fn log(self, v: Self) -> Self {
        f32::log(self, v)
    }

    fn log10(self) -> Self {
        f32::log10(self)
    }
}

impl Float for f64 {
    const ZERO: Self = 0.0;
    const MIN: Self = -1.0;
    const MAX: Self = 1.0;
    const PI: Self = std::f64::consts::PI;

    fn from_usize(v: usize) -> Self {
        v as f64
    }

    fn sin(self) -> Self {
        f64::sin(self)
    }

    fn cos(self) -> Self {
        f64::cos(self)
    }

    fn tan(self) -> Self {
        f64::tan(self)
    }

    fn atan(self) -> Self {
        f64::atan(self)
    }

    fn avg(self, v: Self) -> Self {
        (self + v) / 2.0
    }

    fn powf(self, e: Self) -> Self {
        f64::powf(self, e)
    }

    fn log(self, v: Self) -> Self {
        f64::log(self, v)
    }

    fn log10(self) -> Self {
        f64::log10(self)
    }
}

// processor!(osc(440.0) >> reverb(5.0, 10.0, 1) >> gain(10.0))