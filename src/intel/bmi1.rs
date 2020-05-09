#![cfg(target_feature = "bmi1")]

use super::*;

/// Bitwise `(!a) & b`, `u32`
/// ```
/// # use safe_arch::*;
/// let a = [1, 0, 1, 0];
/// let b = [1, 1, 0, 0];
/// let mut c = [0_u32; 4];
/// for i in 0..4 {
///   c[i] = andnot_u32(a[i], b[i]);
/// }
/// assert_eq!(c, [0, 1, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi1")))]
pub fn andnot_u32(a: u32, b: u32) -> u32 {
  unsafe { _andn_u32(a, b) }
}

/// Bitwise `(!a) & b`, `u64`
/// ```
/// # use safe_arch::*;
/// let a = [1_u64, 0, 1, 0];
/// let b = [1_u64, 1, 0, 0];
/// let mut c = [0_u64; 4];
/// for i in 0..4 {
///   c[i] = andnot_u64(a[i], b[i]);
/// }
/// assert_eq!(c, [0_u64, 1, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi1")))]
pub fn andnot_u64(a: u64, b: u64) -> u64 {
  unsafe { _andn_u64(a, b) }
}

/// Extract a span of bits from the `u32`, start and len style.
/// ```
/// # use safe_arch::*;
/// assert_eq!(bit_extract_u32(0b0110, 0, 3), 0b110);
/// assert_eq!(bit_extract_u32(0b0110, 0, 2), 0b10);
/// assert_eq!(bit_extract_u32(0b0110, 1, 2), 0b11);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_extract_u32(a: u32, start: u32, len: u32) -> u32 {
  unsafe { _bextr_u32(a, start, len) }
}

/// Extract a span of bits from the `u64`, start and len style.
/// ```
/// # use safe_arch::*;
/// assert_eq!(bit_extract_u64(0b0110, 0, 3), 0b110);
/// assert_eq!(bit_extract_u64(0b0110, 0, 2), 0b10);
/// assert_eq!(bit_extract_u64(0b0110, 1, 2), 0b11);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_extract_u64(a: u64, start: u32, len: u32) -> u64 {
  unsafe { _bextr_u64(a, start, len) }
}

/// Extract a span of bits from the `u32`, control value style.
///
/// * Bits 0 through 7: start position.
/// * Bits 8 through 15: span length.
/// ```
/// # use safe_arch::*;
/// assert_eq!(bit_extract2_u32(0b0110, (3 << 8) | 0), 0b110);
/// assert_eq!(bit_extract2_u32(0b0110, (2 << 8) | 0), 0b10);
/// assert_eq!(bit_extract2_u32(0b0110, (2 << 8) | 1), 0b11);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_extract2_u32(a: u32, control: u32) -> u32 {
  unsafe { _bextr2_u32(a, control) }
}

/// Extract a span of bits from the `u64`, control value style.
///
/// * Bits 0 through 7: start position.
/// * Bits 8 through 15: span length.
/// ```
/// # use safe_arch::*;
/// assert_eq!(bit_extract2_u64(0b0110, (3 << 8) | 0), 0b110);
/// assert_eq!(bit_extract2_u64(0b0110, (2 << 8) | 0), 0b10);
/// assert_eq!(bit_extract2_u64(0b0110, (2 << 8) | 1), 0b11);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_extract2_u64(a: u64, control: u64) -> u64 {
  unsafe { _bextr2_u64(a, control) }
}

/// Gets the *value* of the lowest set bit in a `u32`.
///
/// If the input is 0 you get 0 back.
///
/// * Formula: `(!a) & a`
/// ```
/// # use safe_arch::*;
/// assert_eq!(bit_lowest_set_value_u32(0b0), 0);
/// assert_eq!(bit_lowest_set_value_u32(0b1), 1);
/// assert_eq!(bit_lowest_set_value_u32(0b10), 2);
/// assert_eq!(bit_lowest_set_value_u32(0b100), 4);
/// assert_eq!(bit_lowest_set_value_u32(0b111100), 4);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_lowest_set_value_u32(a: u32) -> u32 {
  unsafe { _blsi_u32(a) }
}

/// Gets the *value* of the lowest set bit in a `u64`.
///
/// If the input is 0 you get 0 back.
///
/// * Formula: `(!a) & a`
/// ```
/// # use safe_arch::*;
/// assert_eq!(bit_lowest_set_value_u64(0b0), 0);
/// assert_eq!(bit_lowest_set_value_u64(0b1), 1);
/// assert_eq!(bit_lowest_set_value_u64(0b10), 2);
/// assert_eq!(bit_lowest_set_value_u64(0b100), 4);
/// assert_eq!(bit_lowest_set_value_u64(0b111100), 4);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_lowest_set_value_u64(a: u64) -> u64 {
  unsafe { _blsi_u64(a) }
}

/// Gets the mask of all bits up to and including the lowest set bit in a `u32`.
///
/// If the input is 0, you get `u32::MAX`
///
/// * Formula: `(a - 1) ^ a`
/// ```
/// # use safe_arch::*;
/// assert_eq!(bit_lowest_set_mask_u32(0b0), u32::MAX);
/// assert_eq!(bit_lowest_set_mask_u32(0b1), 0b1);
/// assert_eq!(bit_lowest_set_mask_u32(0b10), 0b11);
/// assert_eq!(bit_lowest_set_mask_u32(0b100), 0b111);
/// assert_eq!(bit_lowest_set_mask_u32(0b111100), 0b111);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_lowest_set_mask_u32(a: u32) -> u32 {
  unsafe { _blsmsk_u32(a) }
}

/// Gets the mask of all bits up to and including the lowest set bit in a `u64`.
///
/// If the input is 0, you get `u64::MAX`
///
/// * Formula: `(a - 1) ^ a`
/// ```
/// # use safe_arch::*;
/// assert_eq!(bit_lowest_set_mask_u64(0b0), u64::MAX);
/// assert_eq!(bit_lowest_set_mask_u64(0b1), 0b1);
/// assert_eq!(bit_lowest_set_mask_u64(0b10), 0b11);
/// assert_eq!(bit_lowest_set_mask_u64(0b100), 0b111);
/// assert_eq!(bit_lowest_set_mask_u64(0b111100), 0b111);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_lowest_set_mask_u64(a: u64) -> u64 {
  unsafe { _blsmsk_u64(a) }
}

/// Resets (clears) the lowest set bit.
///
/// If the input is 0 you get 0 back.
///
/// * Formula: `(a - 1) & a`
/// ```
/// # use safe_arch::*;
/// assert_eq!(bit_lowest_set_reset_u32(0b0), 0);
/// assert_eq!(bit_lowest_set_reset_u32(0b1), 0b0);
/// assert_eq!(bit_lowest_set_reset_u32(0b10), 0b00);
/// assert_eq!(bit_lowest_set_reset_u32(0b100), 0b000);
/// assert_eq!(bit_lowest_set_reset_u32(0b111100), 0b111000);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_lowest_set_reset_u32(a: u32) -> u32 {
  unsafe { _blsr_u32(a) }
}

/// Resets (clears) the lowest set bit.
///
/// If the input is 0 you get 0 back.
///
/// * Formula: `(a - 1) & a`
/// ```
/// # use safe_arch::*;
/// assert_eq!(bit_lowest_set_reset_u64(0b0), 0);
/// assert_eq!(bit_lowest_set_reset_u64(0b1), 0b0);
/// assert_eq!(bit_lowest_set_reset_u64(0b10), 0b00);
/// assert_eq!(bit_lowest_set_reset_u64(0b100), 0b000);
/// assert_eq!(bit_lowest_set_reset_u64(0b111100), 0b111000);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi1")))]
pub fn bit_lowest_set_reset_u64(a: u64) -> u64 {
  unsafe { _blsr_u64(a) }
}

/// Counts the number of trailing zero bits in a `u32`.
///
/// An input of 0 gives 32.
/// ```
/// # use safe_arch::*;
/// assert_eq!(trailing_zero_count_u32(0b0), 32);
/// assert_eq!(trailing_zero_count_u32(0b1), 0);
/// assert_eq!(trailing_zero_count_u32(0b10), 1);
/// assert_eq!(trailing_zero_count_u32(0b100), 2);
/// assert_eq!(trailing_zero_count_u32(0b111100), 2);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi1")))]
pub fn trailing_zero_count_u32(a: u32) -> u32 {
  unsafe { _tzcnt_u32(a) }
}

/// Counts the number of trailing zero bits in a `u64`.
///
/// An input of 0 gives 64.
/// ```
/// # use safe_arch::*;
/// assert_eq!(trailing_zero_count_u64(0b0), 64);
/// assert_eq!(trailing_zero_count_u64(0b1), 0);
/// assert_eq!(trailing_zero_count_u64(0b10), 1);
/// assert_eq!(trailing_zero_count_u64(0b100), 2);
/// assert_eq!(trailing_zero_count_u64(0b111100), 2);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "bmi1")))]
pub fn trailing_zero_count_u64(a: u64) -> u64 {
  unsafe { _tzcnt_u64(a) }
}
