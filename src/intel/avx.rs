#![cfg(target_feature = "avx")]

use super::*;

/// ?
/// ```
/// # use safe_arch::*;
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn ___(_a: m128, _b: m128) -> m128 {
  todo!()
}
