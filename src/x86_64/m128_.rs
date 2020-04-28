#![allow(clippy::transmute_ptr_to_ptr)]

//! This module is for the `m128` wrapper type, its bonus methods, and all
//! necessary trait impls.
//!
//! Intrinsics should _not_ be in this module! They should all be free-functions
//! in the other modules, sorted by CPU target feature.

use super::*;

/// The data for a 128-bit SSE lane.
///
/// * This is _very similar to_ having `[f32; 4]`. The main difference is that
///   it's aligned to 16 instead of just 4, and of course you can perform
///   various intrinsic operations on it.
/// * You can use `as_ref` and `as_mut` to convert a reference to this type to a
///   reference to an array, and from there you _could_ access an individual
///   lane via indexing if you wanted. However, doing this will really kill your
///   performance, because the CPU generally has to move the data out of a
///   register and into memory and then index to the memory location. So, we
///   implement the `AsFoo` trait pair, and _not_ the `DerefFoo` trait pair.
///   This makes any (slow) lane-wise access much more visible in the code.
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct m128(pub __m128);

impl m128 {
  /// Transmutes the data to an array.
  #[must_use]
  #[inline(always)]
  pub fn to_array(self) -> [f32; 4] {
    unsafe { core::mem::transmute(self) }
  }

  /// Transmutes an array into `m128`.
  #[must_use]
  #[inline(always)]
  pub fn from_array(f: [f32; 4]) -> Self {
    unsafe { core::mem::transmute(f) }
  }
}

impl AsRef<[f32; 4]> for m128 {
  #[must_use]
  #[inline(always)]
  fn as_ref(&self) -> &[f32; 4] {
    unsafe { core::mem::transmute(self) }
  }
}

impl AsMut<[f32; 4]> for m128 {
  #[must_use]
  #[inline(always)]
  fn as_mut(&mut self) -> &mut [f32; 4] {
    unsafe { core::mem::transmute(self) }
  }
}

impl Clone for m128 {
  #[must_use]
  #[inline(always)]
  fn clone(&self) -> Self {
    *self
  }
}
impl Copy for m128 {}

impl Default for m128 {
  #[must_use]
  #[inline(always)]
  fn default() -> Self {
    unsafe { core::mem::zeroed() }
  }
}

// TODO: operator overloading!

//
// PLEASE KEEP ALL THE FORMAT IMPL JUNK AT THE END OF THE FILE
//

impl Debug for m128 {
  /// Debug formats each float.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:?}", m128::default());
  /// assert_eq!(&f, "m128(0.0, 0.0, 0.0, 0.0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "m128(")?;
    for (i, float) in self.to_array().iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Debug::fmt(float, f)?;
    }
    write!(f, ")")
  }
}

impl Display for m128 {
  /// Display formats each float, and leaves the type name off of the font.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{}", m128::default());
  /// assert_eq!(&f, "(0, 0, 0, 0)");
  /// ```
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

impl Binary for m128 {
  /// Binary formats each float's bit pattern (via [`f32::to_bits`]).
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:b}", m128::default());
  /// assert_eq!(&f, "(0, 0, 0, 0)");
  /// ```
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

impl LowerExp for m128 {
  /// LowerExp formats each float.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:e}", m128::default());
  /// assert_eq!(&f, "(0e0, 0e0, 0e0, 0e0)");
  /// ```
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

impl UpperExp for m128 {
  /// UpperExp formats each float.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:E}", m128::default());
  /// assert_eq!(&f, "(0E0, 0E0, 0E0, 0E0)");
  /// ```
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

impl LowerHex for m128 {
  /// LowerHex formats each float's bit pattern (via [`f32::to_bits`]).
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:x}", m128::default());
  /// assert_eq!(&f, "(0, 0, 0, 0)");
  /// ```
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

impl UpperHex for m128 {
  /// UpperHex formats each float's bit pattern (via [`f32::to_bits`]).
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:X}", m128::default());
  /// assert_eq!(&f, "(0, 0, 0, 0)");
  /// ```
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

impl Octal for m128 {
  /// Octal formats each float's bit pattern (via [`f32::to_bits`]).
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:o}", m128::default());
  /// assert_eq!(&f, "(0, 0, 0, 0)");
  /// ```
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
