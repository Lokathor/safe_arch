#![cfg(target_feature = "rdseed")]

use super::*;

/// Try to obtain a random `u16` from the hardware RNG.
/// ```
/// # use safe_arch::*;
/// let mut val = 0_u16;
/// let it_worked = rdrand_u16(&mut val);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "rdrand")))]
pub fn rdseed_u16(out: &mut u16) -> i32 {
  unsafe { _rdrand16_step(out) }
}

/// Try to obtain a random `u32` from the hardware RNG.
/// ```
/// # use safe_arch::*;
/// let mut val = 0_u32;
/// let it_worked = rdrand_u32(&mut val);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "rdrand")))]
pub fn rdseed_u32(out: &mut u32) -> i32 {
  unsafe { _rdrand32_step(out) }
}

/// Try to obtain a random `u64` from the hardware RNG.
/// ```
/// # use safe_arch::*;
/// let mut val = 0_u64;
/// let it_worked = rdrand_u64(&mut val);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "rdrand")))]
pub fn rdseed_u64(out: &mut u64) -> i32 {
  unsafe { _rdrand64_step(out) }
}
