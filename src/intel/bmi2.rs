#![cfg(target_feature = "bmi2")]

use super::*;

/// Zero out all high bits in a `u32` starting at the index given.
/// ```
/// # use safe_arch::*;
/// assert_eq!(bit_zero_high_index_u32(0b1111, 0), 0b0000);
/// assert_eq!(bit_zero_high_index_u32(0b1111, 1), 0b0001);
/// assert_eq!(bit_zero_high_index_u32(0b1111, 2), 0b0011);
/// assert_eq!(bit_zero_high_index_u32(0b1111, 3), 0b0111);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi2")))]
pub fn bit_zero_high_index_u32(a: u32, index: u32) -> u32 {
  unsafe { _bzhi_u32(a, index) }
}

/// Zero out all high bits in a `u64` starting at the index given.
/// ```
/// # use safe_arch::*;
/// assert_eq!(bit_zero_high_index_u64(0b1111, 0), 0b0000);
/// assert_eq!(bit_zero_high_index_u64(0b1111, 1), 0b0001);
/// assert_eq!(bit_zero_high_index_u64(0b1111, 2), 0b0011);
/// assert_eq!(bit_zero_high_index_u64(0b1111, 3), 0b0111);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi2")))]
pub fn bit_zero_high_index_u64(a: u64, index: u32) -> u64 {
  unsafe { _bzhi_u64(a, index) }
}

/// Multiply two `u32`, outputting the low bits and storing the high bits in the
/// reference.
///
/// This does not read or write arithmetic flags.
/// ```
/// # use safe_arch::*;
/// let mut x = 0_u32;
/// assert_eq!(mul_extended_u32(u32::MAX, 17, &mut x), 4294967279);
/// assert_eq!(x, 16);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi2")))]
pub fn mul_extended_u32(a: u32, b: u32, extra: &mut u32) -> u32 {
  unsafe { _mulx_u32(a, b, extra) }
}

/// Multiply two `u64`, outputting the low bits and storing the high bits in the
/// reference.
///
/// This does not read or write arithmetic flags.
/// ```
/// # use safe_arch::*;
/// let mut x = 0_u64;
/// assert_eq!(mul_extended_u64(u64::MAX, 17, &mut x), 18446744073709551599);
/// assert_eq!(x, 16);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi2")))]
pub fn mul_extended_u64(a: u64, b: u64, extra: &mut u64) -> u64 {
  unsafe { _mulx_u64(a, b, extra) }
}

/// Deposit contiguous low bits from a `u32` according to a mask.
///
/// Other bits are zero.
/// ```
/// # use safe_arch::*;
/// assert_eq!(population_deposit_u32(0b1001, 0b1111), 0b1001);
/// assert_eq!(population_deposit_u32(0b1001, 0b1110), 0b0010);
/// assert_eq!(population_deposit_u32(0b1001, 0b1100), 0b0100);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi2")))]
pub fn population_deposit_u32(a: u32, index: u32) -> u32 {
  unsafe { _pdep_u32(a, index) }
}

/// Deposit contiguous low bits from a `u64` according to a mask.
///
/// Other bits are zero.
/// ```
/// # use safe_arch::*;
/// assert_eq!(population_deposit_u64(0b1001, 0b1111), 0b1001);
/// assert_eq!(population_deposit_u64(0b1001, 0b1110), 0b0010);
/// assert_eq!(population_deposit_u64(0b1001, 0b1100), 0b0100);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi2")))]
pub fn population_deposit_u64(a: u64, index: u64) -> u64 {
  unsafe { _pdep_u64(a, index) }
}

/// Extract bits from a `u32` according to a mask.
/// ```
/// # use safe_arch::*;
/// assert_eq!(population_extract_u32(0b1001, 0b1111), 0b1001);
/// assert_eq!(population_extract_u32(0b1001, 0b1110), 0b0100);
/// assert_eq!(population_extract_u32(0b1001, 0b1100), 0b0010);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi2")))]
pub fn population_extract_u32(a: u32, index: u32) -> u32 {
  unsafe { _pext_u32(a, index) }
}

/// Extract bits from a `u64` according to a mask.
/// ```
/// # use safe_arch::*;
/// assert_eq!(population_extract_u64(0b1001, 0b1111), 0b1001);
/// assert_eq!(population_extract_u64(0b1001, 0b1110), 0b0100);
/// assert_eq!(population_extract_u64(0b1001, 0b1100), 0b0010);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi2")))]
pub fn population_extract_u64(a: u64, index: u64) -> u64 {
  unsafe { _pext_u64(a, index) }
}
