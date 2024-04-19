pub fn db_to_gain(db: f32) -> f32 {
    f32::powf(10.0, db / 20.0)
}

pub fn db_to_gain_floor(db: f32, floor: f32) -> f32 {
    if db > floor {
        f32::powf(10.0, db / 20.0)
    } else {
        floor
    }
}

pub fn gain_to_db(gain: f32) -> f32 {
    20.0 * f32::log10(f32::max(f32::MIN, gain))
}

pub fn db_to_linear(value: f32) -> f32 {
    f32::powf(10.0, value / 40.0) / 2.0
}

pub fn linear_to_db(value: f32) -> f32 {
    40.0 * f32::log10(f32::max(f32::MIN, value * 2.0))
}

pub trait IntoDecibals {
    fn dbs(self) -> f32;
}

impl IntoDecibals for f32 {
    #[inline]
    fn dbs(self) -> f32 {
        if self != 0.0 {
            20.0 * self.log10()
        } else {
            -144.0
        }
    }
}

/*pub trait Float {
    fn zero(&mut self);
    fn neg(&mut self);
    fn add(&mut self, value: f32);
    fn sub(&mut self, value: f32);
    fn mul(&mut self, value: f32);
    fn div(&mut self, value: f32);

    fn sin(&mut self);
    fn cos(&mut self);
    fn tan(&mut self);
    fn sinh(&mut self);
    fn cosh(&mut self);
    fn tanh(&mut self);

    fn gain(&mut self, amount: f32);
    fn bias(&mut self, decibals: f32);
}

impl Float for f32 {
    /* Basic Operations */

    #[inline]
    fn zero(&mut self) {
        *self = 0.0;
    }

    #[inline]
    fn neg(&mut self) {
        *self = *self * -1.0;
    }

    #[inline]
    fn add(&mut self, value: f32) {
        *self = *self + value;
    }

    #[inline]
    fn sub(&mut self, value: f32) {
        *self = *self - value;
    }

    #[inline]
    fn mul(&mut self, value: f32) {
        *self = *self * value;
    }

    #[inline]
    fn div(&mut self, value: f32) {
        *self = *self / value;
    }

    /* Trigonometric Operations */

    #[inline]
    fn sin(&mut self) {
        *self = faster::sin(*self);
    }

    #[inline]
    fn cos(&mut self) {
        *self = faster::cos(*self);
    }

    #[inline]
    fn tan(&mut self) {
        *self = faster::tan(*self);
    }

    #[inline]
    fn sinh(&mut self) {
        *self = faster::sinh(*self);
    }

    #[inline]
    fn cosh(&mut self) {
        *self = faster::cosh(*self);
    }

    #[inline]
    fn tanh(&mut self) {
        *self = faster::tanh(*self);
    }

    /* Music Operations */

    #[inline]
    fn bias(&mut self, decibals: f32) {
        self.add(decibals);
    }

    #[inline]
    fn gain(&mut self, amount: f32) {
        self.mul(amount);
    }
}

pub trait StereoFloat {
    fn pan(&mut self, amount: f32);
    fn zero(&mut self);
    fn add(&mut self, value: f32);
    fn sub(&mut self, value: f32);
    fn mul(&mut self, value: f32);
    fn div(&mut self, value: f32);

    fn sin(&mut self);
    fn cos(&mut self);
    fn tan(&mut self);
    fn sinh(&mut self);
    fn cosh(&mut self);
    fn tanh(&mut self);

    fn gain(&mut self, amount: f32);
    fn bias(&mut self, decibals: f32);
}

impl StereoFloat for (&mut f32, &mut f32) {
    /* Stereo Operations */

    fn pan(&mut self, decibals: f32) {
        self.0.gain(decibals);
        self.1.gain(-decibals);
    }

    /* Basic Operations */

    #[inline]
    fn zero(&mut self) {
        self.0.zero();
        self.1.zero();
    }

    #[inline]
    fn add(&mut self, value: f32) {
        self.0.add(value);
        self.1.add(value);
    }

    #[inline]
    fn sub(&mut self, value: f32) {
        self.0.sub(value);
        self.1.sub(value);
    }

    #[inline]
    fn mul(&mut self, value: f32) {
        self.0.mul(value);
        self.1.mul(value);
    }

    #[inline]
    fn div(&mut self, value: f32) {
        self.0.div(value);
        self.1.div(value);
    }

    /* Trigonometric Operations */

    #[inline]
    fn sin(&mut self) {
        Float::sin(self.0);
        Float::sin(self.1);
    }

    #[inline]
    fn cos(&mut self) {
        Float::cos(self.0);
        Float::cos(self.1);
    }

    #[inline]
    fn tan(&mut self) {
        Float::tan(self.0);
        Float::tan(self.1);
    }

    #[inline]
    fn sinh(&mut self) {
        Float::sinh(self.0);
        Float::sinh(self.1);
    }

    #[inline]
    fn cosh(&mut self) {
        Float::cosh(self.0);
        Float::cosh(self.1);
    }

    #[inline]
    fn tanh(&mut self) {
        Float::tanh(self.0);
        Float::tanh(self.1);
    }

    /* Music Operations */

    #[inline]
    fn bias(&mut self, decibals: f32) {
        Float::bias(self.0, decibals);
        Float::bias(self.1, decibals);
    }

    #[inline]
    fn gain(&mut self, amount: f32) {
        Float::gain(self.0, amount);
        Float::gain(self.1, amount);
    }
}*/

/*impl StereoFloat for (f32, f32) {
    fn pan(&mut self, decibals: f32) {
        self.0.gain(decibals);
        self.1.gain(-decibals);
    }
}*/
