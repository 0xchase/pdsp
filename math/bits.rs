#[inline]
pub fn to_bits(x: f32) -> u32 {
    unsafe { ::std::mem::transmute::<f32, u32>(x) }
}

/// Raw transmutation from `u32`.
///
/// Converts the given `u32` containing the float's raw memory representation into the `f32` type.
/// Similar to `f32::from_bits` but even more raw.
#[inline]
pub fn from_bits(x: u32) -> f32 {
    unsafe { ::std::mem::transmute::<u32, f32>(x) }
}
