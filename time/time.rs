#[derive(Copy, Clone)]
pub struct TimeMessage {
    pub start: f64,
    pub length: f64,
    pub rate: f64,
    pub cycle: Option<TimeCycle>,
}

#[derive(Copy, Clone)]
pub struct TimeCycle {
    start: f64,
    end: f64
}

impl TimeMessage {
    pub fn from(start: f64, end: f64) -> Self {
        Self {
            start,
            length: end - start,
            rate: 1.0,
            cycle: None
        }
    }

    pub fn start(&self) -> f64 {
        self.start
    }

    pub fn end(&self) -> f64 {
        self.start + self.length
    }

    pub fn length(&self) -> f64 {
        self.length
    }

    pub fn shift(&self, beats: f64) -> TimeMessage {
        TimeMessage {
            start: self.start + beats,
            length: self.length,
            rate: self.rate,
            cycle: match &self.cycle {
                Some(cycle) => Some(
                    TimeCycle {
                        start: cycle.start + beats,
                        end: cycle.end + beats
                    }
                ),
                None => None
            }
        }
    }

    pub fn contains(&self, beat: f64) -> bool {
        match self.cycle {
            Some(cycle) => {
                if self.length() >= 0.0 {
                    if (self.start <= beat
                        && self.start + self.length() >= beat
                        && self.length() != 0.0)
                        || (0.0 >= beat && (cycle.end - self.start) >= beat && self.length() != 0.0)
                    {
                        true
                    } else {
                        false
                    }
                } else {
                    if (self.start >= beat
                        && self.start + self.length() <= beat
                        && self.length() != 0.0)
                        || (0.0 >= beat && (cycle.end - self.start) <= beat && self.length() != 0.0)
                    {
                        true
                    } else {
                        false
                    }
                }
            }
            None => {
                if self.length() >= 0.0 {
                    if self.start <= beat && self.start + self.length() >= beat && self.length() != 0.0
                    {
                        true
                    } else {
                        false
                    }
                } else {
                    if self.start >= beat && self.start + self.length() <= beat && self.length() != 0.0
                    {
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }

    pub fn get_rate(&self) -> f32 {
        self.rate as f32
    }

    pub fn rate(&self, rate: f64) -> TimeMessage {
        match self.cycle {
            Some(cycle) => TimeMessage {
                start: (self.start * rate) % cycle.start,
                length: self.length * rate,
                rate: self.rate * rate,
                cycle: Some(cycle),
            },
            None => TimeMessage {
                start: self.start * rate,
                length: self.length * rate,
                rate: self.rate * rate,
                cycle: None,
            },
        }
    }

    pub fn cycle(&self, beats: f64) -> TimeMessage {
        match self.cycle {
            Some(cycle) => {
                if beats < cycle.end {
                    TimeMessage {
                        start: self.start % beats,
                        length: self.length,
                        rate: self.rate,
                        cycle: Some(cycle),
                    }
                } else {
                    *self
                }
            }
            None => TimeMessage {
                start: self.start % beats,
                length: self.length,
                rate: self.rate,
                cycle: Some(TimeCycle {
                    start: 0.0,
                    end: beats
                })
            },
        }
    }

    pub fn on_each<F: FnMut(usize)>(&self, rate: f64, mut f: F) {
        let end = self.start + self.length();

        if self.length() > 0.0 {
            match &self.cycle {
                Some(cycle) => {
                    if self.start + self.length() <= cycle.end {
                        if self.start % rate > end % rate {
                            (f)(((end - (end % rate)) / rate).round() as usize - 1);
                        }
                    } else {
                        if self.start % rate > cycle.end % rate {
                            (f)(((cycle.end - (cycle.end % rate)) / rate).round() as usize - 1);
                        }

                        if 0.0 % rate > (self.start + self.length() - cycle.end) % rate {
                            (f)(0)
                        }
                    }
                }
                None => {
                    if self.start % rate > end % rate {
                        (f)(((end - (end % rate)) / rate).round() as usize - 1);
                    }
                }
            }
        } else if self.length() < 0.0 {
            if self.start % rate < end % rate {
                (f)(((self.start - (self.start % rate)) / rate).round() as usize - 1);
            }
        }
    }

    pub fn start_sample(&self, bpm: f64, sample_rate: u32) -> usize {
        let beat = self.start;
        let secs = beat * (1.0 / (bpm / 60.0));
        let sample = secs * sample_rate as f64;
        return f64::floor(sample) as usize;
    }

    pub fn end_sample(&self, bpm: f64, sample_rate: u32) -> usize {
        let beat = self.start + self.length;
        let secs = beat * (1.0 / (bpm / 60.0));
        let sample = secs * sample_rate as f64;
        return f64::floor(sample) as usize;
    }
}
