#![cfg(target_feature = "lzcnt")]

use super::*;

/// Count the leading zeroes in a `u32`.
/// ```
/// # use safe_arch::*;
/// assert_eq!(leading_zero_count_u32(u32::MAX), 0);
/// assert_eq!(leading_zero_count_u32(u32::MAX >> 3), 3);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse")))]
pub fn leading_zero_count_u32(a: u32) -> u32 {
  unsafe { _lzcnt_u32(a) }
}

/// Count the leading zeroes in a `u64`.
/// ```
/// # use safe_arch::*;
/// assert_eq!(leading_zero_count_u64(u64::MAX), 0);
/// assert_eq!(leading_zero_count_u64(u64::MAX >> 3), 3);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse")))]
pub fn leading_zero_count_u64(a: u64) -> u64 {
  unsafe { _lzcnt_u64(a) }
}
