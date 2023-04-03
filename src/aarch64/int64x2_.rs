//! The `int64x2` wrapper type.
//!
//! Intrinsics don't go here! Only non-intrinsic methods/trait-impls should go
//! in this module.

use super::*;

/// The data for a 128-bit Neon register of two `i64` lanes.
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct int64x2(pub int64x2_t);

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for int64x2 {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for int64x2 {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::TransparentWrapper<int64x2_t> for int64x2 {}

impl int64x2 {
  /// Transmutes the `int64x2` to an array.
  ///
  /// Same as `m.into()`, just lets you be more explicit about what's happening.
  #[must_use]
  #[inline(always)]
  pub fn to_array(self) -> [i64; 2] {
    self.into()
  }

  /// Transmutes an array into `int64x2`.
  ///
  /// Same as `int64x2::from(arr)`, it just lets you be more explicit about
  /// what's happening.
  #[must_use]
  #[inline(always)]
  pub fn from_array(f: [i64; 2]) -> Self {
    f.into()
  }
}

impl Clone for int64x2 {
  #[must_use]
  #[inline(always)]
  fn clone(&self) -> Self {
    *self
  }
}
impl Copy for int64x2 {}

impl Default for int64x2 {
  #[must_use]
  #[inline(always)]
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

impl From<[i64; 2]> for int64x2 {
  #[must_use]
  #[inline(always)]
  fn from(arr: [i64; 2]) -> Self {
    // Safety: because this semantically moves the value from the input position
    // (align4) to the output position (align16) it is fine to increase our
    // required alignment without worry.
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<int64x2> for [i64; 2] {
  #[must_use]
  #[inline(always)]
  fn from(m: int64x2) -> Self {
    // We can of course transmute to a lower alignment
    unsafe { core::mem::transmute(m) }
  }
}

//
// PLEASE KEEP ALL THE FORMAT IMPL JUNK AT THE END OF THE FILE
//

impl Debug for int64x2 {
  /// Debug formats each float.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "int64x2(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Debug::fmt(float, f)?;
    }
    write!(f, ")")
  }
}

impl Display for int64x2 {
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

impl Binary for int64x2 {
  /// Binary formats each float's bit pattern.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Binary::fmt(&float, f)?;
    }
    write!(f, ")")
  }
}

impl LowerExp for int64x2 {
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

impl UpperExp for int64x2 {
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

impl LowerHex for int64x2 {
  /// LowerHex formats each float's bit pattern.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      LowerHex::fmt(&float, f)?;
    }
    write!(f, ")")
  }
}

impl UpperHex for int64x2 {
  /// UpperHex formats each float's bit pattern.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      UpperHex::fmt(&float, f)?;
    }
    write!(f, ")")
  }
}

impl Octal for int64x2 {
  /// Octal formats each float's bit pattern.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Octal::fmt(&float, f)?;
    }
    write!(f, ")")
  }
}
