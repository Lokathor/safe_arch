//! This module is for the `m128i` wrapper type, its bonus methods, and all
//! necessary trait impls.
//!
//! Intrinsics should _not_ be in this module! They should all be free-functions
//! in the other modules, sorted by CPU target feature.

use super::*;

/// The data for a 128-bit SSE register of integer data.
///
/// * The exact layout to view the type as depends on the operation used.
/// * Formatting impls print as four `i32` values. If you want alternate
///   formatting you can use the appropriate `From`/`Into` conversion and then
///   format that.
/// * You can use `as_ref` and `as_mut` to view the type as if it was an array,
///   and from there you _could_ access an individual lane via indexing if you
///   wanted. However, doing this will usually kill your performance if you're
///   in the middle of a series of operations. The CPU has to move the type out
///   of register and into memory, then index the memory. In other words, you
///   should index the individual lanes as little as possible. Accordingly, we
///   make you use a "more obvious" trait if you want to do it.
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct m128i(pub __m128i);

#[test]
fn test_m128_size_align() {
  assert_eq!(core::mem::size_of::<m128i>(), 16);
  assert_eq!(core::mem::align_of::<m128i>(), 16);
}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroed for m128i {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for m128i {}

impl AsRef<[i32; 4]> for m128i {
  #[must_use]
  #[inline(always)]
  fn as_ref(&self) -> &[i32; 4] {
    // Safety: Since the alignment requirement of the output reference type is
    // lower than our own reference type this is safe.
    unsafe { core::mem::transmute(self) }
  }
}

impl AsMut<[i32; 4]> for m128i {
  #[must_use]
  #[inline(always)]
  fn as_mut(&mut self) -> &mut [i32; 4] {
    // Safety: Since the alignment requirement of the output reference type is
    // lower than our own reference type this is safe.
    unsafe { core::mem::transmute(self) }
  }
}

impl Clone for m128i {
  #[must_use]
  #[inline(always)]
  fn clone(&self) -> Self {
    *self
  }
}
impl Copy for m128i {}

impl Default for m128i {
  #[must_use]
  #[inline(always)]
  fn default() -> Self {
    // TODO: use the zeroed intrinsic
    unsafe { core::mem::zeroed() }
  }
}

// i8

impl From<[i8; 16]> for m128i {
  #[must_use]
  #[inline(always)]
  fn from(arr: [i8; 16]) -> Self {
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<m128i> for [i8; 16] {
  #[must_use]
  #[inline(always)]
  fn from(m: m128i) -> Self {
    unsafe { core::mem::transmute(m) }
  }
}

// i16

impl From<[i16; 8]> for m128i {
  #[must_use]
  #[inline(always)]
  fn from(arr: [i16; 8]) -> Self {
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<m128i> for [i16; 8] {
  #[must_use]
  #[inline(always)]
  fn from(m: m128i) -> Self {
    unsafe { core::mem::transmute(m) }
  }
}

// i32

impl From<[i32; 4]> for m128i {
  #[must_use]
  #[inline(always)]
  fn from(arr: [i32; 4]) -> Self {
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<m128i> for [i32; 4] {
  #[must_use]
  #[inline(always)]
  fn from(m: m128i) -> Self {
    unsafe { core::mem::transmute(m) }
  }
}

// i64

impl From<[i64; 2]> for m128i {
  #[must_use]
  #[inline(always)]
  fn from(arr: [i64; 2]) -> Self {
    unsafe { core::mem::transmute(arr) }
  }
}

impl From<m128i> for [i64; 2] {
  #[must_use]
  #[inline(always)]
  fn from(m: m128i) -> Self {
    unsafe { core::mem::transmute(m) }
  }
}

//
// PLEASE KEEP ALL THE FORMAT IMPL JUNK AT THE END OF THE FILE
//

impl Debug for m128i {
  /// Debug formats each `i32`.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:?}", m128i::default());
  /// assert_eq!(&f, "m128i(0, 0, 0, 0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "m128i(")?;
    for (i, int) in <[i32; 4]>::from(*self).iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Debug::fmt(int, f)?;
    }
    write!(f, ")")
  }
}

impl Display for m128i {
  /// Display formats each `i32`, and leaves the type name off of the font.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{}", m128i::default());
  /// assert_eq!(&f, "(0, 0, 0, 0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, int) in <[i32; 4]>::from(*self).iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Display::fmt(int, f)?;
    }
    write!(f, ")")
  }
}

impl Binary for m128i {
  /// Binary formats each `i32`.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:b}", m128i::default());
  /// assert_eq!(&f, "(0, 0, 0, 0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, int) in <[i32; 4]>::from(*self).iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Binary::fmt(int, f)?;
    }
    write!(f, ")")
  }
}

impl LowerExp for m128i {
  /// LowerExp formats each `i32`.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:e}", m128i::default());
  /// assert_eq!(&f, "(0e0, 0e0, 0e0, 0e0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, int) in <[i32; 4]>::from(*self).iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      LowerExp::fmt(int, f)?;
    }
    write!(f, ")")
  }
}

impl UpperExp for m128i {
  /// UpperExp formats each `i32`.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:E}", m128i::default());
  /// assert_eq!(&f, "(0E0, 0E0, 0E0, 0E0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, int) in <[i32; 4]>::from(*self).iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      UpperExp::fmt(int, f)?;
    }
    write!(f, ")")
  }
}

impl LowerHex for m128i {
  /// LowerHex formats each `i32`.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:x}", m128i::default());
  /// assert_eq!(&f, "(0, 0, 0, 0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, int) in <[i32; 4]>::from(*self).iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      LowerHex::fmt(int, f)?;
    }
    write!(f, ")")
  }
}

impl UpperHex for m128i {
  /// UpperHex formats each `i32`.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:X}", m128i::default());
  /// assert_eq!(&f, "(0, 0, 0, 0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, int) in <[i32; 4]>::from(*self).iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      UpperHex::fmt(int, f)?;
    }
    write!(f, ")")
  }
}

impl Octal for m128i {
  /// Octal formats each `i32`.
  /// ```
  /// # use safe_arch::*;
  /// let f = format!("{:o}", m128i::default());
  /// assert_eq!(&f, "(0, 0, 0, 0)");
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "(")?;
    for (i, int) in <[i32; 4]>::from(*self).iter().enumerate() {
      if i != 0 {
        write!(f, ", ")?;
      }
      Octal::fmt(int, f)?;
    }
    write!(f, ")")
  }
}
