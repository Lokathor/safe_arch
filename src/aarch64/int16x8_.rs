//! The `int16x8` wrapper type.
//!
//! Intrinsics don't go here! Only non-intrinsic methods/trait-impls should go
//! in this module.

use super::*;

/// The data for a 128-bit Neon register of eight `i16` lanes.
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct int16x8(pub int16x8_t);

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for int16x8 {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for int16x8 {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::TransparentWrapper<int16x8_t> for int16x8 {}

impl int16x8 {
  /// Transmutes the `int16x8` to an array.
  ///
  /// Same as `m.into()`, just lets you be more explicit about what's happening.
  #[must_use]
  #[inline(always)]
  pub fn to_array(self) -> [i16; 8] {
    self.into()
  }

  /// Transmutes an array into `int16x8`.
  ///
  /// Same as `int16x8::from(arr)`, it just lets you be more explicit about
  /// what's happening.
  #[must_use]
  #[inline(always)]
  pub fn from_array(f: [i16; 8]) -> Self {
    f.into()
  }
}

impl Clone for int16x8 {
  #[must_use]
  #[inline(always)]
  fn clone(&self) -> Self {
    *self
  }
}
impl Copy for int16x8 {}

impl Default for int16x8 {
  #[must_use]
  #[inline(always)]
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

impl From<[i16; 8]> for int16x8 {
  #[must_use]
  #[inline(always)]
  fn from(arr: [i16; 8]) -> Self {
    // Safety: because this semantically moves the value from the input position
    // (align4) to the output position (align16) it is fine to increase our
    // required alignment without worry.
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<int16x8> for [i16; 8] {
  #[must_use]
  #[inline(always)]
  fn from(m: int16x8) -> Self {
    // We can of course transmute to a lower alignment
    unsafe { core::mem::transmute(m) }
  }
}

//
// PLEASE KEEP ALL THE FORMAT IMPL JUNK AT THE END OF THE FILE
//

impl Debug for int16x8 {
  /// Debug formats each float.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "int16x8(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Debug::fmt(float, f)?;
    }
    write!(f, ")")
  }
}

impl Display for int16x8 {
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

impl Binary for int16x8 {
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

impl LowerExp for int16x8 {
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

impl UpperExp for int16x8 {
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

impl LowerHex for int16x8 {
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

impl UpperHex for int16x8 {
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

impl Octal for int16x8 {
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
