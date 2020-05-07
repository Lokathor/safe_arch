#![cfg(target_feature = "bmi1")]

use super::*;

/// Bitwise `(!a) & b`.
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

// _andn_u64
// _bextr_u32
// _bextr_u64
// _bextr2_u32
// _bextr2_u64
// _blsi_u32
// _blsi_u64
// _blsmsk_u32
// _blsmsk_u64
// _blsr_u32
// _blsr_u64
// _mm_tzcnt_32
// _mm_tzcnt_64
// _tzcnt_u32
// _tzcnt_u64
