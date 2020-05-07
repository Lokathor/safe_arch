#![cfg(target_feature = "sse4.1")]

use super::*;

/// Lanewise `a > b` with lanes as `i64`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i64, 3]);
/// let b = m128i::from([0_i64, 3]);
/// let c: [i64; 2] = cmp_gt_mask_i64_m128i(a, b).into();
/// assert_eq!(c, [-1_i64, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn cmp_gt_mask_i64_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmpgt_epi64(a.0, b.0) })
}

/// Accumulates the `u8` into a running CRC32 value.
/// ```
/// # use safe_arch::*;
/// assert_eq!(crc32_u8(u32::MAX, u8::MAX), 16777215_u32);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn crc32_u8(crc: u32, v: u8) -> u32 {
  unsafe { _mm_crc32_u8(crc, v) }
}

/// Accumulates the `u16` into a running CRC32 value.
/// ```
/// # use safe_arch::*;
/// assert_eq!(crc32_u16(u32::MAX, u16::MAX), 65535_u32);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn crc32_u16(crc: u32, v: u16) -> u32 {
  unsafe { _mm_crc32_u16(crc, v) }
}

/// Accumulates the `u32` into a running CRC32 value.
/// ```
/// # use safe_arch::*;
/// assert_eq!(crc32_u32(u32::MAX, u32::MAX), 0_u32);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn crc32_u32(crc: u32, v: u32) -> u32 {
  unsafe { _mm_crc32_u32(crc, v) }
}

/// Accumulates the `u64` into a running CRC32 value.
///
/// Note that, unlike with the other `crc32` functions, the running CRC32 input
/// and output is a `u64` value _in addition_ to the new value to accumulate
/// being a `u64`.
///
/// ```
/// # use safe_arch::*;
/// assert_eq!(crc32_u64(u64::MAX, u64::MAX), 3080238136_u64);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn crc32_u64(crc: u64, v: u64) -> u64 {
  unsafe { _mm_crc32_u64(crc, v) }
}

// HARD

// ? a c i m o s z ?

// EXPLICIT LENGTHS

// _mm_cmpestra
// returns 1 if b did not contain a null character and the resulting mask was
// zero, and 0 otherwise.

// _mm_cmpestrc
// returns 1 if the resulting mask was non-zero, and 0 otherwise.

// _mm_cmpestri
// store the generated index in dst.

// _mm_cmpestrm
// store the generated mask in dst.

// _mm_cmpestro
// returns bit 0 of the resulting bit mask.

// _mm_cmpestrs
// returns 1 if any character in a was null, and 0 otherwise.

// _mm_cmpestrz
// returns 1 if any character in b was null, and 0 otherwise.

// IMPLICIT LENGTHS

// _mm_cmpistra

// _mm_cmpistrc

// _mm_cmpistri

// _mm_cmpistrm

// _mm_cmpistro

// _mm_cmpistrs

// _mm_cmpistrz
