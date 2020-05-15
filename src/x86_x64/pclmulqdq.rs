#![cfg(target_feature = "pclmulqdq")]

use super::*;

/// Performs a "carryless" multiplication of two `i64` values.
///
/// You specify `m128i` expressions and then `, lane 0` or `, lane 1` for each
/// one to select which of the two `i64` lanes is used in the multiplication.
///
/// ```
/// # use safe_arch::*;
/// let x = m128i::from([2_i64, 3]);
/// let y = m128i::from([4_i64, 500]);
/// //
/// let c: [i64; 2] = mul_i64_carryless_m128i!(x, lane 0, y, lane 0).into();
/// assert_eq!(c, [8_i64, 0]);
/// let c: [i64; 2] = mul_i64_carryless_m128i!(x, lane 1, y, lane 0).into();
/// assert_eq!(c, [12_i64, 0]);
/// let c: [i64; 2] = mul_i64_carryless_m128i!(x, lane 0, y, lane 1).into();
/// assert_eq!(c, [1000_i64, 0]);
/// let c: [i64; 2] = mul_i64_carryless_m128i!(x, lane 1, y, lane 1).into();
/// assert_eq!(c, [540_i64, 0]); // not 1500 like a normal mul would be!
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "pclmulqdq")))]
macro_rules! mul_i64_carryless_m128i {
  ($a:expr, lane 0, $b:expr, lane 0) => {{
    let a: m128i = $a;
    let b: m128i = $b;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_clmulepi64_si128;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_clmulepi64_si128;
    m128i(unsafe { _mm_clmulepi64_si128(a.0, b.0, 0_i32) })
  }};
  ($a:expr, lane 1, $b:expr, lane 0) => {{
    let a: m128i = $a;
    let b: m128i = $b;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_clmulepi64_si128;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_clmulepi64_si128;
    m128i(unsafe { _mm_clmulepi64_si128(a.0, b.0, 0b1_i32) })
  }};
  ($a:expr, lane 0, $b:expr, lane 1) => {{
    let a: m128i = $a;
    let b: m128i = $b;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_clmulepi64_si128;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_clmulepi64_si128;
    m128i(unsafe { _mm_clmulepi64_si128(a.0, b.0, 0b1_0000_i32) })
  }};
  ($a:expr, lane 1, $b:expr, lane 1) => {{
    let a: m128i = $a;
    let b: m128i = $b;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_clmulepi64_si128;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_clmulepi64_si128;
    m128i(unsafe { _mm_clmulepi64_si128(a.0, b.0, 0b1_0001_i32) })
  }};
}
