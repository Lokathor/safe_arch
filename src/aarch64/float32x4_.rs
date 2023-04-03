//! The `float32x4` wrapper type.
//!
//! Intrinsics don't go here! Only non-intrinsic methods/trait-impls should go
//! in this module.

use super::*;

/// The data for a 128-bit Neon register of four `f32` lanes.
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct float32x4(pub float32x4_t);

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for float32x4 {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for float32x4 {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::TransparentWrapper<float32x4_t> for float32x4 {}

impl float32x4 {
  /// Transmutes the `float32x4` to an array.
  ///
  /// Same as `m.into()`, just lets you be more explicit about what's happening.
  #[must_use]
  #[inline(always)]
  pub fn to_array(self) -> [f32; 4] {
    self.into()
  }

  /// Transmutes an array into `float32x4`.
  ///
  /// Same as `float32x4::from(arr)`, it just lets you be more explicit about
  /// what's happening.
  #[must_use]
  #[inline(always)]
  pub fn from_array(f: [f32; 4]) -> Self {
    f.into()
  }

  //

  /// Converts into the bit patterns of these floats (`[u32;4]`).
  ///
  /// Like [`f32::to_bits`](f32::to_bits), but all four lanes at once.
  #[must_use]
  #[inline(always)]
  pub fn to_bits(self) -> [u32; 4] {
    unsafe { core::mem::transmute(self) }
  }

  /// Converts from the bit patterns of these floats (`[u32;4]`).
  ///
  /// Like [`f32::from_bits`](f32::from_bits), but all four lanes at once.
  #[must_use]
  #[inline(always)]
  pub fn from_bits(bits: [u32; 4]) -> Self {
    unsafe { core::mem::transmute(bits) }
  }
}

impl Clone for float32x4 {
  #[must_use]
  #[inline(always)]
  fn clone(&self) -> Self {
    *self
  }
}
impl Copy for float32x4 {}

impl Default for float32x4 {
  #[must_use]
  #[inline(always)]
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

impl From<[f32; 4]> for float32x4 {
  #[must_use]
  #[inline(always)]
  fn from(arr: [f32; 4]) -> Self {
    // Safety: because this semantically moves the value from the input position
    // (align4) to the output position (align16) it is fine to increase our
    // required alignment without worry.
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<float32x4> for [f32; 4] {
  #[must_use]
  #[inline(always)]
  fn from(m: float32x4) -> Self {
    // We can of course transmute to a lower alignment
    unsafe { core::mem::transmute(m) }
  }
}

#[doc(cfg(target_feature = "neon"))]
impl float32x4 {
  /// Lanewise absolute value.
  #[must_use]
  #[inline(always)]
  pub fn abs(self) -> Self {
    float32x4(unsafe { vabsq_f32(self.0) })
  }
}

//
// PLEASE KEEP ALL THE FORMAT IMPL JUNK AT THE END OF THE FILE
//

impl Debug for float32x4 {
  /// Debug formats each float.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "float32x4(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Debug::fmt(float, f)?;
    }
    write!(f, ")")
  }
}

impl Display for float32x4 {
  /// Display formats each float, and leaves the type name off of the font.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Display::fmt(float, f)?;
    }
    write!(f, ")")
  }
}

impl Binary for float32x4 {
  /// Binary formats each float's bit pattern (via [`f32::to_bits`]).
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Binary::fmt(&float.to_bits(), f)?;
    }
    write!(f, ")")
  }
}

impl LowerExp for float32x4 {
  /// LowerExp formats each float.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      LowerExp::fmt(float, f)?;
    }
    write!(f, ")")
  }
}

impl UpperExp for float32x4 {
  /// UpperExp formats each float.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      UpperExp::fmt(float, f)?;
    }
    write!(f, ")")
  }
}

impl LowerHex for float32x4 {
  /// LowerHex formats each float's bit pattern (via [`f32::to_bits`]).
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      LowerHex::fmt(&float.to_bits(), f)?;
    }
    write!(f, ")")
  }
}

impl UpperHex for float32x4 {
  /// UpperHex formats each float's bit pattern (via [`f32::to_bits`]).
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      UpperHex::fmt(&float.to_bits(), f)?;
    }
    write!(f, ")")
  }
}

impl Octal for float32x4 {
  /// Octal formats each float's bit pattern (via [`f32::to_bits`]).
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Octal::fmt(&float.to_bits(), f)?;
    }
    write!(f, ")")
  }
}
