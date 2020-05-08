#![cfg(target_feature = "popcnt")]

use super::*;

/// Count the number of bits set within an `i32`
/// ```
/// # use safe_arch::*;
/// assert_eq!(population_count_i32(0), 0);
/// assert_eq!(population_count_i32(0b1), 1);
/// assert_eq!(population_count_i32(0b1001), 2);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "popcnt")))]
pub fn population_count_i32(a: i32) -> i32 {
  unsafe { _popcnt32(a) }
}

/// Count the number of bits set within an `i64`
/// ```
/// # use safe_arch::*;
/// assert_eq!(population_count_i64(0), 0);
/// assert_eq!(population_count_i64(0b1), 1);
/// assert_eq!(population_count_i64(0b1001), 2);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "popcnt")))]
pub fn population_count_i64(a: i64) -> i32 {
  unsafe { _popcnt64(a) }
}
