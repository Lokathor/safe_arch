//! The `uint8x16` wrapper type.
//!
//! Intrinsics don't go here! Only non-intrinsic methods/trait-impls should go
//! in this module.

use super::*;

/// The data for a 128-bit Neon register of sixteen `u8` lanes.
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct uint8x16(pub uint8x16_t);

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for uint8x16 {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for uint8x16 {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::TransparentWrapper<uint8x16_t> for uint8x16 {}

impl uint8x16 {
  /// Transmutes the `uint8x16` to an array.
  ///
  /// Same as `m.into()`, just lets you be more explicit about what's happening.
  #[must_use]
  #[inline(always)]
  pub fn to_array(self) -> [u8; 16] {
    self.into()
  }

  /// Transmutes an array into `uint8x16`.
  ///
  /// Same as `uint8x16::from(arr)`, it just lets you be more explicit about
  /// what's happening.
  #[must_use]
  #[inline(always)]
  pub fn from_array(f: [u8; 16]) -> Self {
    f.into()
  }
}

impl Clone for uint8x16 {
  #[must_use]
  #[inline(always)]
  fn clone(&self) -> Self {
    *self
  }
}
impl Copy for uint8x16 {}

impl Default for uint8x16 {
  #[must_use]
  #[inline(always)]
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

impl From<[u8; 16]> for uint8x16 {
  #[must_use]
  #[inline(always)]
  fn from(arr: [u8; 16]) -> Self {
    // Safety: because this semantically moves the value from the input position
    // (align4) to the output position (align16) it is fine to increase our
    // required alignment without worry.
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<uint8x16> for [u8; 16] {
  #[must_use]
  #[inline(always)]
  fn from(m: uint8x16) -> Self {
    // We can of course transmute to a lower alignment
    unsafe { core::mem::transmute(m) }
  }
}

//
// PLEASE KEEP ALL THE FORMAT IMPL JUNK AT THE END OF THE FILE
//

impl Debug for uint8x16 {
  /// Debug formats each float.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "uint8x16(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Debug::fmt(float, f)?;
    }
    write!(f, ")")
  }
}

impl Display for uint8x16 {
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

impl Binary for uint8x16 {
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

impl LowerExp for uint8x16 {
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

impl UpperExp for uint8x16 {
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

impl LowerHex for uint8x16 {
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

impl UpperHex for uint8x16 {
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

impl Octal for uint8x16 {
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
