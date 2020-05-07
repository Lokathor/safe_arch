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

/// ?
///
/// ```
/// # use safe_arch::*;
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_e_str_a {
  ($a:expr, $la:expr, $b:expr, $lb:expr, $imm:expr) => {{
    let a: m128i = $a;
    let la: i32 = $la;
    let b: m128i = $b;
    let lb: i32 = $lb;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpestra;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpestra;
    unsafe { _mm_cmpestra(a.0, la, b.0, lb, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_e_str_c {
  ($a:expr, $la:expr, $b:expr, $lb:expr, $imm:expr) => {{
    let a: m128i = $a;
    let la: i32 = $la;
    let b: m128i = $b;
    let lb: i32 = $lb;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpestrc;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpestrc;
    unsafe { _mm_cmpestrc(a.0, la, b.0, lb, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_e_str_i {
  ($a:expr, $la:expr, $b:expr, $lb:expr, $imm:expr) => {{
    let a: m128i = $a;
    let la: i32 = $la;
    let b: m128i = $b;
    let lb: i32 = $lb;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpestri;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpestri;
    unsafe { _mm_cmpestri(a.0, la, b.0, lb, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_e_str_m {
  ($a:expr, $la:expr, $b:expr, $lb:expr, $imm:expr) => {{
    let a: m128i = $a;
    let la: i32 = $la;
    let b: m128i = $b;
    let lb: i32 = $lb;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpestrm;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpestrm;
    m128i(unsafe { _mm_cmpestrm(a.0, la, b.0, lb, IMM) })
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_e_str_o {
  ($a:expr, $la:expr, $b:expr, $lb:expr, $imm:expr) => {{
    let a: m128i = $a;
    let la: i32 = $la;
    let b: m128i = $b;
    let lb: i32 = $lb;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpestro;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpestro;
    unsafe { _mm_cmpestro(a.0, la, b.0, lb, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_e_str_s {
  ($a:expr, $la:expr, $b:expr, $lb:expr, $imm:expr) => {{
    let a: m128i = $a;
    let la: i32 = $la;
    let b: m128i = $b;
    let lb: i32 = $lb;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpestrs;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpestrs;
    unsafe { _mm_cmpestrs(a.0, la, b.0, lb, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_e_str_z {
  ($a:expr, $la:expr, $b:expr, $lb:expr, $imm:expr) => {{
    let a: m128i = $a;
    let la: i32 = $la;
    let b: m128i = $b;
    let lb: i32 = $lb;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpestrz;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpestrz;
    unsafe { _mm_cmpestrz(a.0, la, b.0, lb, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_i_str_a {
  ($a:expr, $la:expr, $b:expr, $lb:expr, $imm:expr) => {{
    let a: m128i = $a;
    let la: i32 = $la;
    let b: m128i = $b;
    let lb: i32 = $lb;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpistra;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpistra;
    unsafe { _mm_cmpistra(a.0, la, b.0, lb, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_i_str_c {
  ($a:expr, $la:expr, $b:expr, $lb:expr, $imm:expr) => {{
    let a: m128i = $a;
    let la: i32 = $la;
    let b: m128i = $b;
    let lb: i32 = $lb;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpistrc;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpistrc;
    unsafe { _mm_cmpistrc(a.0, la, b.0, lb, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_i_str_i {
  ($a:expr, $la:expr, $b:expr, $lb:expr, $imm:expr) => {{
    let a: m128i = $a;
    let la: i32 = $la;
    let b: m128i = $b;
    let lb: i32 = $lb;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpistri;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpistri;
    unsafe { _mm_cmpistri(a.0, la, b.0, lb, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_i_str_m {
  ($a:expr, $la:expr, $b:expr, $lb:expr, $imm:expr) => {{
    let a: m128i = $a;
    let la: i32 = $la;
    let b: m128i = $b;
    let lb: i32 = $lb;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpistrm;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpistrm;
    m128i(unsafe { _mm_cmpistrm(a.0, la, b.0, lb, IMM) })
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_i_str_o {
  ($a:expr, $la:expr, $b:expr, $lb:expr, $imm:expr) => {{
    let a: m128i = $a;
    let la: i32 = $la;
    let b: m128i = $b;
    let lb: i32 = $lb;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpistro;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpistro;
    unsafe { _mm_cmpistro(a.0, la, b.0, lb, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_i_str_s {
  ($a:expr, $la:expr, $b:expr, $lb:expr, $imm:expr) => {{
    let a: m128i = $a;
    let la: i32 = $la;
    let b: m128i = $b;
    let lb: i32 = $lb;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpistrs;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpistrs;
    unsafe { _mm_cmpistrs(a.0, la, b.0, lb, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_i_str_z {
  ($a:expr, $la:expr, $b:expr, $lb:expr, $imm:expr) => {{
    let a: m128i = $a;
    let la: i32 = $la;
    let b: m128i = $b;
    let lb: i32 = $lb;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpistrz;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpistrz;
    unsafe { _mm_cmpistrz(a.0, la, b.0, lb, IMM) }
  }};
}
