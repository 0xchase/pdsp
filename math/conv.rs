// fm = 2^((m−69)/12) * (440 Hz)
pub fn note_to_freq(note: u32) -> f32 {
    (2.0_f32.powf((note as f32 - 69.0) / 12.0) * 440.0) as f32
}

// m = 12*log2(fm/440 Hz) + 69
pub fn freq_to_note(freq: f32) -> u32 {
    (12.0 * (freq / 440.0).log2() + 69.0) as u32
}
