//! The `float64x2` wrapper type.
//!
//! Intrinsics don't go here! Only non-intrinsic methods/trait-impls should go
//! in this module.

use super::*;

/// The data for a 128-bit Neon register of two `f64` lanes.
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct float64x2(pub float64x2_t);

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for float64x2 {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for float64x2 {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::TransparentWrapper<float64x2_t> for float64x2 {}

impl float64x2 {
  /// Transmutes the `float64x2` to an array.
  ///
  /// Same as `m.into()`, just lets you be more explicit about what's happening.
  #[must_use]
  #[inline(always)]
  pub fn to_array(self) -> [f64; 2] {
    self.into()
  }

  /// Transmutes an array into `float64x2`.
  ///
  /// Same as `float64x2::from(arr)`, it just lets you be more explicit about
  /// what's happening.
  #[must_use]
  #[inline(always)]
  pub fn from_array(f: [f64; 2]) -> Self {
    f.into()
  }

  //

  /// Converts into the bit patterns of these floats (`[u64; 2]`).
  ///
  /// Like [`f64::to_bits`](f64::to_bits), but both lanes at once.
  #[must_use]
  #[inline(always)]
  pub fn to_bits(self) -> [u64; 2] {
    unsafe { core::mem::transmute(self) }
  }

  /// Converts from the bit patterns of these floats (`[u64; 2]`).
  ///
  /// Like [`f64::from_bits`](f64::from_bits), but both lanes at once.
  #[must_use]
  #[inline(always)]
  pub fn from_bits(bits: [u64; 2]) -> Self {
    unsafe { core::mem::transmute(bits) }
  }
}

impl Clone for float64x2 {
  #[must_use]
  #[inline(always)]
  fn clone(&self) -> Self {
    *self
  }
}
impl Copy for float64x2 {}

impl Default for float64x2 {
  #[must_use]
  #[inline(always)]
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

impl From<[f64; 2]> for float64x2 {
  #[must_use]
  #[inline(always)]
  fn from(arr: [f64; 2]) -> Self {
    // Safety: because this semantically moves the value from the input position
    // (align4) to the output position (align16) it is fine to increase our
    // required alignment without worry.
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<float64x2> for [f64; 2] {
  #[must_use]
  #[inline(always)]
  fn from(m: float64x2) -> Self {
    // We can of course transmute to a lower alignment
    unsafe { core::mem::transmute(m) }
  }
}

//
// PLEASE KEEP ALL THE FORMAT IMPL JUNK AT THE END OF THE FILE
//

impl Debug for float64x2 {
  /// Debug formats each float.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "float64x2(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Debug::fmt(float, f)?;
    }
    write!(f, ")")
  }
}

impl Display for float64x2 {
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

impl Binary for float64x2 {
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

impl LowerExp for float64x2 {
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

impl UpperExp for float64x2 {
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

impl LowerHex for float64x2 {
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

impl UpperHex for float64x2 {
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

impl Octal for float64x2 {
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
