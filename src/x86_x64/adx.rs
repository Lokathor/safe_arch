#![cfg(target_feature = "adx")]

use super::*;

/// Add two `u32` with a carry value.
///
/// Writes the sum to the reference and returns the new carry flag.
/// ```
/// # use safe_arch::*;
/// let mut out = 0_u32;
/// assert_eq!(add_carry_u32(1, u32::MAX, 5, &mut out), 1);
/// assert_eq!(out, 5);
/// ```
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "adx")))]
pub fn add_carry_u32(c_in: u8, a: u32, b: u32, out: &mut u32) -> u8 {
  unsafe { _addcarryx_u32(c_in, a, b, out) }
}

/// Add two `u64` with a carry value.
///
/// Writes the sum to the reference and returns the new carry flag.
/// ```
/// # use safe_arch::*;
/// let mut out = 0_u64;
/// assert_eq!(add_carry_u64(1, u64::MAX, 5, &mut out), 1);
/// assert_eq!(out, 5);
/// ```
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "adx")))]
#[cfg(target_arch = "x86_64")]
pub fn add_carry_u64(c_in: u8, a: u64, b: u64, out: &mut u64) -> u8 {
  unsafe { _addcarryx_u64(c_in, a, b, out) }
}
