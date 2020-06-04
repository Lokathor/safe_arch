#![cfg(target_feature = "sse4.1")]

use super::*;

/// Blends the `i16` lanes according to the immediate mask.
///
/// Each bit 0 though 7 controls lane 0 through 7. Use 0 for the `$a` value and
/// 1 for the `$b` value.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_i16, 1, 2, 3, 4, 5, 6, 7]);
/// let b = m128i::from([0_i16, -1, -2, -3, -4, -5, -6, -7]);
/// //
/// let c: [i16; 8] = blend_imm_i16_m128i!(a, b, 0b1111_0110).into();
/// assert_eq!(c, [0_i16, -1, -2, 3, -4, -5, -6, -7]);
/// ```
#[macro_export]
macro_rules! blend_imm_i16_m128i {
  ($a:expr, $b:expr, $imm:expr) => {{
    let a: m128i = $a;
    let b: m128i = $b;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_blend_epi16;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_blend_epi16;
    m128i(unsafe { _mm_blend_epi16(a.0, b.0, IMM) })
  }};
}

/// Blends the lanes according to the immediate mask.
///
/// Bits 0 and 1 control where output lane 0 and 1 come from. Use 0 for the `$a`
/// value and 1 for the `$b` value.
///
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([0.0, 1.0]);
/// let b = m128d::from_array([2.0, 3.0]);
/// let c = blend_imm_m128d!(a, b, 0b10).to_array();
/// assert_eq!(c, [0.0, 3.0]);
/// ```
#[macro_export]
macro_rules! blend_imm_m128d {
  ($a:expr, $b:expr, $imm:expr) => {{
    let a: m128d = $a;
    let b: m128d = $b;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_blend_pd;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_blend_pd;
    m128d(unsafe { _mm_blend_pd(a.0, b.0, IMM) })
  }};
}

/// Blends the lanes according to the immediate mask.
///
/// Bits 0 to 3 control where output lane 0 to 3 come from. Use 0 for the `$a`
/// value and 1 for the `$b` value.
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([0.0, 1.0, 2.0, 3.0]);
/// let b = m128::from_array([4.0, 5.0, 6.0, 7.0]);
/// let c = blend_imm_m128!(a, b, 0b0110).to_array();
/// assert_eq!(c, [0.0, 5.0, 6.0, 3.0]);
/// ```
#[macro_export]
macro_rules! blend_imm_m128 {
  ($a:expr, $b:expr, $imm:expr) => {{
    let a: m128 = $a;
    let b: m128 = $b;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_blend_ps;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_blend_ps;
    m128(unsafe { _mm_blend_ps(a.0, b.0, IMM) })
  }};
}

/// Blend the `i8` lanes according to a runtime varying mask.
///
/// The sign bit of each `i8` lane in the `mask` value determines if the output
/// lane uses `a` (mask non-negative) or `b` (mask negative).
///
/// ```
/// # use safe_arch::*;
/// let a =
///   m128i::from([0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let b = m128i::from([
///   0_i8, -1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14, -15,
/// ]);
/// let mask =
///   m128i::from([0_i8, -1, -1, 0, 0, 0, -1, -1, -1, 0, 0, 0, -1, -1, -1, 0]);
/// let c: [i8; 16] = blend_varying_i8_m128i(a, b, mask).into();
/// assert_eq!(c, [0, -1, -2, 3, 4, 5, -6, -7, -8, 9, 10, 11, -12, -13, -14, 15]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn blend_varying_i8_m128i(a: m128i, b: m128i, mask: m128i) -> m128i {
  m128i(unsafe { _mm_blendv_epi8(a.0, b.0, mask.0) })
}

/// Blend the lanes according to a runtime varying mask.
///
/// The sign bit of each lane in the `mask` value determines if the output
/// lane uses `a` (mask non-negative) or `b` (mask negative).
///
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([0.0, 1.0]);
/// let b = m128d::from_array([2.0, 3.0]);
/// let mask = m128d::from_array([-1.0, 0.0]);
/// let c = blend_varying_m128d(a, b, mask).to_array();
/// assert_eq!(c, [2.0, 1.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn blend_varying_m128d(a: m128d, b: m128d, mask: m128d) -> m128d {
  m128d(unsafe { _mm_blendv_pd(a.0, b.0, mask.0) })
}

/// Blend the lanes according to a runtime varying mask.
///
/// The sign bit of each lane in the `mask` value determines if the output
/// lane uses `a` (mask non-negative) or `b` (mask negative).
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([0.0, 1.0, 2.0, 3.0]);
/// let b = m128::from_array([4.0, 5.0, 6.0, 7.0]);
/// let mask = m128::from_array([-1.0, 0.0, -1.0, 0.0]);
/// let c = blend_varying_m128(a, b, mask).to_array();
/// assert_eq!(c, [4.0, 1.0, 6.0, 3.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn blend_varying_m128(a: m128, b: m128, mask: m128) -> m128 {
  m128(unsafe { _mm_blendv_ps(a.0, b.0, mask.0) })
}

/// Round each lane to a whole number, towards positive infinity
///
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([-0.1, 1.8]);
/// assert_eq!(ceil_m128d(a).to_array(), [0.0, 2.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn ceil_m128d(a: m128d) -> m128d {
  m128d(unsafe { _mm_ceil_pd(a.0) })
}

/// Round each lane to a whole number, towards positive infinity
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([-0.1, 1.8, 2.5, 3.0]);
/// assert_eq!(ceil_m128(a).to_array(), [0.0, 2.0, 3.0, 3.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn ceil_m128(a: m128) -> m128 {
  m128(unsafe { _mm_ceil_ps(a.0) })
}

/// Round the low lane of `b` toward positive infinity, high lane is `a`.
///
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([-0.1, 1.8]);
/// let b = m128d::from_array([2.5, 3.0]);
/// assert_eq!(ceil_m128d_s(a, b).to_array(), [3.0, 1.8]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn ceil_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_ceil_sd(a.0, b.0) })
}

/// Round the low lane of `b` toward positive infinity, other lanes `a`.
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([-0.1, 1.8, 5.0, 6.0]);
/// let b = m128::from_array([2.5, 3.0, 10.0, 20.0]);
/// assert_eq!(ceil_m128_s(a, b).to_array(), [3.0, 1.8, 5.0, 6.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn ceil_m128_s(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_ceil_ss(a.0, b.0) })
}

/// Lanewise `a == b` with lanes as `i64`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5_i64, 6_i64]);
/// let b = m128i::from([5_i64, 7_i64]);
/// let c: [i64; 2] = cmp_eq_mask_i64_m128i(a, b).into();
/// assert_eq!(c, [-1_i64, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn cmp_eq_mask_i64_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmpeq_epi64(a.0, b.0) })
}

/// Convert the lower four `i16` lanes to four `i32` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_i16, -1, 2, -3, 4, 5, 6, 7]);
/// let c: [i32; 4] = convert_i16_lower4_to_i32_m128i(a).into();
/// assert_eq!(c, [0, -1, 2, -3]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_i16_lower4_to_i32_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepi16_epi32(a.0) })
}

/// Convert the lower two `i64` lanes to two `i32` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0_i16, -1, 2, -3, 4, 5, 6, 7]);
/// let c: [i64; 2] = convert_i16_lower2_to_i64_m128i(a).into();
/// assert_eq!(c, [0, -1]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_i16_lower2_to_i64_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepi16_epi64(a.0) })
}

/// Convert the lower two `i32` lanes to two `i64` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0, -1, 2, -3]);
/// let c: [i64; 2] = convert_i32_lower2_to_i64_m128i(a).into();
/// assert_eq!(c, [0, -1]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_i32_lower2_to_i64_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepi32_epi64(a.0) })
}

/// Convert the lower eight `i8` lanes to eight `i16` lanes.
/// ```
/// # use safe_arch::*;
/// let a =
///   m128i::from([0_i8, -1, 2, -3, 4, -5, 6, -7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let c: [i16; 8] = convert_i8_lower8_to_i16_m128i(a).into();
/// assert_eq!(c, [0_i16, -1, 2, -3, 4, -5, 6, -7]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_i8_lower8_to_i16_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepi8_epi16(a.0) })
}

/// Convert the lower four `i8` lanes to four `i32` lanes.
/// ```
/// # use safe_arch::*;
/// let a =
///   m128i::from([0_i8, -1, 2, -3, 4, -5, 6, -7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let c: [i32; 4] = convert_i8_lower4_to_i32_m128i(a).into();
/// assert_eq!(c, [0, -1, 2, -3]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_i8_lower4_to_i32_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepi8_epi32(a.0) })
}

/// Convert the lower two `i8` lanes to two `i64` lanes.
/// ```
/// # use safe_arch::*;
/// let a =
///   m128i::from([0_i8, -1, 2, -3, 4, -5, 6, -7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let c: [i64; 2] = convert_i8_lower2_to_i64_m128i(a).into();
/// assert_eq!(c, [0_i64, -1]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_i8_lower2_to_i64_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepi8_epi64(a.0) })
}

/// Convert the lower four `u16` lanes to four `u32` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([u16::MAX, 1, 2, 3, 4, 5, 6, 7]);
/// let c: [u32; 4] = convert_u16_lower4_to_u32_m128i(a).into();
/// assert_eq!(c, [u16::MAX as u32, 1, 2, 3]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_u16_lower4_to_u32_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepu16_epi32(a.0) })
}

/// Convert the lower two `u16` lanes to two `u64` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([u16::MAX, 1, 2, 3, 4, 5, 6, 7]);
/// let c: [u64; 2] = convert_u16_lower2_to_u64_m128i(a).into();
/// assert_eq!(c, [u16::MAX as u64, 1]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_u16_lower2_to_u64_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepu16_epi64(a.0) })
}

/// Convert the lower two `u32` lanes to two `u64` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([u32::MAX, 1, 2, 3]);
/// let c: [u64; 2] = convert_u32_lower2_to_u64_m128i(a).into();
/// assert_eq!(c, [u32::MAX as u64, 1]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_u32_lower2_to_u64_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepu32_epi64(a.0) })
}

/// Convert the lower eight `u8` lanes to eight `u16` lanes.
/// ```
/// # use safe_arch::*;
/// let a =
///   m128i::from([u8::MAX, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let c: [u16; 8] = convert_u8_lower8_to_u16_m128i(a).into();
/// assert_eq!(c, [u8::MAX as u16, 1, 2, 3, 4, 5, 6, 7]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_u8_lower8_to_u16_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepu8_epi16(a.0) })
}

/// Convert the lower four `u8` lanes to four `u32` lanes.
/// ```
/// # use safe_arch::*;
/// let a =
///   m128i::from([u8::MAX, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let c: [u32; 4] = convert_u8_lower4_to_u32_m128i(a).into();
/// assert_eq!(c, [u8::MAX as u32, 1, 2, 3]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_u8_lower4_to_u32_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepu8_epi32(a.0) })
}

/// Convert the lower two `u8` lanes to two `u64` lanes.
/// ```
/// # use safe_arch::*;
/// let a =
///   m128i::from([u8::MAX, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let c: [u64; 2] = convert_u8_lower2_to_u64_m128i(a).into();
/// assert_eq!(c, [u8::MAX as u64, 1]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn convert_u8_lower2_to_u64_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_cvtepu8_epi64(a.0) })
}

/// Performs a dot product of two `m128d` registers.
///
/// The output details are determined by a control mask:
/// * For each lane, you can multiply that lane from `$a` and `$b` or you can
///   take a default of 0.0
/// * This forms two temporary `f64` values which are summed to a single `f64`.
/// * For each output lane, you can have the sum in that lane or 0.0.
///
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 2.0]);
/// let b = m128d::from_array([3.0, 4.0]);
///
/// // Bits 4 determines if we mul lanes 0, and bit 5 if we mul lanes 1.
///
/// let c = dot_product_m128d!(a, b, 0b0000_0011).to_array();
/// assert_eq!(c, [0.0, 0.0]); // no mul
/// let c = dot_product_m128d!(a, b, 0b0001_0011).to_array();
/// assert_eq!(c, [3.0, 3.0]); // mul lane 0 (1 * 3)
/// let c = dot_product_m128d!(a, b, 0b0010_0011).to_array();
/// assert_eq!(c, [8.0, 8.0]); // mul lane 1 (2 * 4)
/// let c = dot_product_m128d!(a, b, 0b0011_0011).to_array();
/// assert_eq!(c, [11.0, 11.0]); // mul both lanes (and summed in the next step)
///
/// // After here we have two temp lanes, which get added to form `sum`.
///
/// // Bits 0 and 1 determines if an output lane is `sum` or `0.0`.
///
/// let c = dot_product_m128d!(a, b, 0b0011_0000).to_array();
/// assert_eq!(c, [0.0, 0.0]); // never use sum
/// let c = dot_product_m128d!(a, b, 0b0011_0001).to_array();
/// assert_eq!(c, [11.0, 0.0]); // sum in output lane 0
/// let c = dot_product_m128d!(a, b, 0b0011_0010).to_array();
/// assert_eq!(c, [0.0, 11.0]); // sum in output lane 1
/// let c = dot_product_m128d!(a, b, 0b0011_0011).to_array();
/// assert_eq!(c, [11.0, 11.0]); // sum in both output lanes.
/// ```
#[macro_export]
macro_rules! dot_product_m128d {
  ($a:expr, $b:expr, $imm:expr) => {{
    let a: m128d = $a;
    let b: m128d = $b;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_dp_pd;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_dp_pd;
    m128d(unsafe { _mm_dp_pd(a.0, b.0, IMM) })
  }};
}

/// Performs a dot product of two `m128` registers.
///
/// The output details are determined by a control mask:
/// * For each lane, you can multiply that lane from `$a` and `$b` or you can
///   take a default of 0.0
/// * This forms four temporary `f32` values which are summed to a single `f32`.
/// * For each output lane, you can have the sum in that lane or 0.0.
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([5.0, 6.0, 7.0, 8.0]);
///
/// // Bits 4 through 7 determine if we should mul lanes 0 through 3.
///
/// let c = dot_product_m128!(a, b, 0b0000_1111).to_array();
/// assert_eq!(c, [0.0, 0.0, 0.0, 0.0]); // no mul
/// let c = dot_product_m128!(a, b, 0b0001_1111).to_array();
/// assert_eq!(c, [5.0, 5.0, 5.0, 5.0]); // mul temp lane 0 (1 * 5)
/// let c = dot_product_m128!(a, b, 0b0010_1111).to_array();
/// assert_eq!(c, [12.0, 12.0, 12.0, 12.0]); // mul temp lane 1 (2 * 6)
/// let c = dot_product_m128!(a, b, 0b0100_1111).to_array();
/// assert_eq!(c, [21.0, 21.0, 21.0, 21.0]); // mul temp lane 2 (3 * 7)
/// let c = dot_product_m128!(a, b, 0b1000_1111).to_array();
/// assert_eq!(c, [32.0, 32.0, 32.0, 32.0]); // mul temp lane 3 (4 * 8)
/// let c = dot_product_m128!(a, b, 0b1111_1111).to_array();
/// assert_eq!(c, [70.0, 70.0, 70.0, 70.0]); // mul all lanes (and summed in the next step)
///
/// // After here we have four temp lanes, which get added to form `sum`.
///
/// // Bits 0 through 3 determines if the `sum` is in lanes 0 through 3.
///
/// let c = dot_product_m128!(a, b, 0b1111_0000).to_array();
/// assert_eq!(c, [0.0, 0.0, 0.0, 0.0]); // never use sum
///
/// let c = dot_product_m128!(a, b, 0b1111_0001).to_array();
/// assert_eq!(c, [70.0, 0.0, 0.0, 0.0]); // sum in output lane 0
///
/// let c = dot_product_m128!(a, b, 0b1111_0010).to_array();
/// assert_eq!(c, [0.0, 70.0, 0.0, 0.0]); // sum in output lane 1
///
/// let c = dot_product_m128!(a, b, 0b1111_0100).to_array();
/// assert_eq!(c, [0.0, 0.0, 70.0, 0.0]); // sum in output lane 2
///
/// let c = dot_product_m128!(a, b, 0b1111_1000).to_array();
/// assert_eq!(c, [0.0, 0.0, 0.0, 70.0]); // sum in output lane 3
///
/// let c = dot_product_m128!(a, b, 0b1111_1111).to_array();
/// assert_eq!(c, [70.0, 70.0, 70.0, 70.0]); // sum in all output lanes
/// ```
#[macro_export]
macro_rules! dot_product_m128 {
  ($a:expr, $b:expr, $imm:expr) => {{
    let a: m128 = $a;
    let b: m128 = $b;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_dp_ps;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_dp_ps;
    m128(unsafe { _mm_dp_ps(a.0, b.0, IMM) })
  }};
}

/// Gets the `i32` lane requested. Only the lowest 2 bits are considered.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5, 6, 7, 8]);
/// assert_eq!(extract_i32_imm_m128i!(a, 1), 6);
/// ```
#[macro_export]
macro_rules! extract_i32_imm_m128i {
  ($a:expr, $imm:expr) => {{
    let a: m128i = $a;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_extract_epi32;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_extract_epi32;
    unsafe { _mm_extract_epi32(a.0, IMM) }
  }};
}

/// Gets the `i64` lane requested. Only the lowest bit is considered.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5_i64, 6]);
/// assert_eq!(extract_i64_imm_m128i!(a, 1), 6_i64);
/// ```
#[macro_export]
#[cfg(target_arch = "x86_64")]
macro_rules! extract_i64_imm_m128i {
  ($a:expr, $imm:expr) => {{
    let a: m128i = $a;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_extract_epi64;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_extract_epi64;
    unsafe { _mm_extract_epi64(a.0, IMM) }
  }};
}

/// Gets the `i8` lane requested. Only the lowest 4 bits are considered.
///
/// ```
/// # use safe_arch::*;
/// let a =
///   m128i::from([0_i8, 1, 2, 3, 4, 5, 6, 101, 8, 9, 10, 11, 12, 13, 14, 15]);
/// assert_eq!(extract_i8_as_i32_imm_m128i!(a, 7), 101_i32);
/// ```
#[macro_export]
macro_rules! extract_i8_as_i32_imm_m128i {
  ($a:expr, $imm:expr) => {{
    let a: m128i = $a;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_extract_epi8;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_extract_epi8;
    unsafe { _mm_extract_epi8(a.0, IMM) }
  }};
}

/// Gets the `f32` lane requested. Returns as an `i32` bit pattern.
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([5.0, 6.0, 7.0, 8.0]);
/// assert_eq!(extract_f32_as_i32_bits_imm_m128!(a, 3), 8_f32.to_bits() as i32);
/// ```
#[macro_export]
macro_rules! extract_f32_as_i32_bits_imm_m128 {
  ($a:expr, $imm:expr) => {{
    let a: m128 = $a;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_extract_ps;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_extract_ps;
    unsafe { _mm_extract_ps(a.0, IMM) }
  }};
}

/// Round each lane to a whole number, towards negative infinity
///
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([-0.1, 1.8]);
/// assert_eq!(floor_m128d(a).to_array(), [-1.0, 1.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn floor_m128d(a: m128d) -> m128d {
  m128d(unsafe { _mm_floor_pd(a.0) })
}

/// Round each lane to a whole number, towards negative infinity
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([-0.1, 1.8, 2.5, 3.0]);
/// assert_eq!(floor_m128(a).to_array(), [-1.0, 1.0, 2.0, 3.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn floor_m128(a: m128) -> m128 {
  m128(unsafe { _mm_floor_ps(a.0) })
}

/// Round the low lane of `b` toward negative infinity, high lane is `a`.
///
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([-0.1, 1.8]);
/// let b = m128d::from_array([2.5, 3.0]);
/// assert_eq!(floor_m128d_s(a, b).to_array(), [2.0, 1.8]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn floor_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_floor_sd(a.0, b.0) })
}

/// Round the low lane of `b` toward negative infinity, other lanes `a`.
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([-0.1, 1.8, 5.0, 6.0]);
/// let b = m128::from_array([2.5, 3.0, 10.0, 20.0]);
/// assert_eq!(floor_m128_s(a, b).to_array(), [2.0, 1.8, 5.0, 6.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn floor_m128_s(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_floor_ss(a.0, b.0) })
}

/// Inserts a new value for the `i32` lane specified.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5, 6, 7, 8]);
/// let b: [i32; 4] = insert_i32_imm_m128i!(a, 23, 1).into();
/// assert_eq!(b, [5, 23, 7, 8]);
/// ```
#[macro_export]
macro_rules! insert_i32_imm_m128i {
  ($a:expr, $new:expr, $imm:expr) => {{
    let a: m128i = $a;
    let new: i32 = $new;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_insert_epi32;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_insert_epi32;
    m128i(unsafe { _mm_insert_epi32(a.0, new, IMM) })
  }};
}

/// Inserts a new value for the `i64` lane specified.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5_i64, 6]);
/// let b: [i64; 2] = insert_i64_imm_m128i!(a, 23, 1).into();
/// assert_eq!(b, [5_i64, 23]);
/// ```
#[macro_export]
#[cfg(target_arch = "x86_64")]
macro_rules! insert_i64_imm_m128i {
  ($a:expr, $new:expr, $imm:expr) => {{
    let a: m128i = $a;
    let new: i64 = $new;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_insert_epi64;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_insert_epi64;
    m128i(unsafe { _mm_insert_epi64(a.0, new, IMM) })
  }};
}

/// Inserts a new value for the `i64` lane specified.
///
/// ```
/// # use safe_arch::*;
/// let a =
///   m128i::from([0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// let b: [i8; 16] = insert_i8_imm_m128i!(a, 23, 1).into();
/// assert_eq!(b, [0_i8, 23, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// ```
#[macro_export]
macro_rules! insert_i8_imm_m128i {
  ($a:expr, $new:expr, $imm:expr) => {{
    let a: m128i = $a;
    let new: i32 = $new;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_insert_epi8;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_insert_epi8;
    m128i(unsafe { _mm_insert_epi8(a.0, new, IMM) })
  }};
}

/// Inserts a lane from `$b` into `$a`, optionally at a new position.
///
/// Also, you can zero out any lanes you like for free as part of the same
/// operation. If you don't specify the mask argument then no lanes are zeroed.
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([5.0, 6.0, 7.0, 8.0]);
/// //
/// let c = insert_f32_imm_m128!(a, b, from 0, to 3).to_array();
/// assert_eq!(c, [1.0, 2.0, 3.0, 5.0]);
/// //
/// let c = insert_f32_imm_m128!(a, b, from 0, to 3, mask 0b0110).to_array();
/// assert_eq!(c, [1.0, 0.0, 0.0, 5.0]);
/// ```
#[macro_export]
macro_rules! insert_f32_imm_m128 {
  ($a:expr, $b:expr, from $b_lane_src:expr, to $a_lane_dest:expr, mask $clear_lanes:expr) => {{
    let a: m128 = $a;
    let b: m128 = $b;
    const IMM: i32 = ($b_lane_src & 0b11) << 6
      | ($a_lane_dest & 0b11) << 4
      | ($clear_lanes & 0b1111);
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_insert_ps;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_insert_ps;
    m128(unsafe { _mm_insert_ps(a.0, b.0, IMM) })
  }};
  ($a:expr, $b:expr, from $b_lane_src:expr, to $a_lane_dest:expr) => {{
    insert_f32_imm_m128!($a, $b, from $b_lane_src, to $a_lane_dest, mask 0)
  }};
}

/// Lanewise `max(a, b)` with lanes as `i32`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let b = m128i::from([5, 6, -7, 8]);
/// let c: [i32; 4] = max_i32_m128i(a, b).into();
/// assert_eq!(c, [5, 6, 3, 8]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn max_i32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_max_epi32(a.0, b.0) })
}

/// Lanewise `max(a, b)` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a =
///   m128i::from([0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 127]);
/// let b = m128i::from([
///   0_i8, 11, 2, -13, 4, 15, 6, -17, -8, 19, -20, 21, 22, -23, 24, 127,
/// ]);
/// let c: [i8; 16] = max_i8_m128i(a, b).into();
/// assert_eq!(c, [0, 11, 2, 3, 4, 15, 6, 7, 8, 19, 10, 21, 22, 13, 24, 127]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn max_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_max_epi8(a.0, b.0) })
}

/// Lanewise `max(a, b)` with lanes as `u16`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_u16, 2, 300, 400, 1, 2, 3, 4]);
/// let b = m128i::from([5_u16, 6, 7, 8, 15, 26, 37, 48]);
/// let c: [u16; 8] = max_u16_m128i(a, b).into();
/// assert_eq!(c, [5_u16, 6, 300, 400, 15, 26, 37, 48]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn max_u16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_max_epu16(a.0, b.0) })
}

/// Lanewise `max(a, b)` with lanes as `u32`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 200, 3, 4]);
/// let b = m128i::from([5, 6, 7, 8]);
/// let c: [u32; 4] = max_u32_m128i(a, b).into();
/// assert_eq!(c, [5, 200, 7, 8]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn max_u32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_max_epu32(a.0, b.0) })
}

/// Lanewise `min(a, b)` with lanes as `i32`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let b = m128i::from([5, 6, -7, 8]);
/// let c: [i32; 4] = min_i32_m128i(a, b).into();
/// assert_eq!(c, [1, 2, -7, 4]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn min_i32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_min_epi32(a.0, b.0) })
}

/// Lanewise `min(a, b)` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a =
///   m128i::from([0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 127]);
/// let b = m128i::from([
///   0_i8, 11, 2, -13, 4, 15, 6, -17, -8, 19, -20, 21, 22, -23, 24, 127,
/// ]);
/// let c: [i8; 16] = min_i8_m128i(a, b).into();
/// assert_eq!(
///   c,
///   [0_i8, 1, 2, -13, 4, 5, 6, -17, -8, 9, -20, 11, 12, -23, 14, 127]
/// );
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn min_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_min_epi8(a.0, b.0) })
}

/// Lanewise `min(a, b)` with lanes as `u16`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_u16, 2, 300, 400, 1, 2, 3, 4]);
/// let b = m128i::from([5_u16, 6, 7, 8, 15, 26, 37, 48]);
/// let c: [u16; 8] = min_u16_m128i(a, b).into();
/// assert_eq!(c, [1_u16, 2, 7, 8, 1, 2, 3, 4]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn min_u16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_min_epu16(a.0, b.0) })
}

/// Lanewise `min(a, b)` with lanes as `u32`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 200, 3, 4]);
/// let b = m128i::from([5, 6, 7, 8]);
/// let c: [u32; 4] = min_u32_m128i(a, b).into();
/// assert_eq!(c, [1, 6, 3, 4]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn min_u32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_min_epu32(a.0, b.0) })
}

/// Min `u16` value, position, and other lanes zeroed.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([120_u16, 24, 300, 400, 90, 129, 31, 114]);
/// let c: [u16; 8] = min_position_u16_m128i(a).into();
/// assert_eq!(c, [24_u16, 1, 0, 0, 0, 0, 0, 0]);
///
/// // the position favors the leftmost minimum
/// let a = m128i::from([120_u16, 24, 24, 400, 90, 129, 31, 114]);
/// let c: [u16; 8] = min_position_u16_m128i(a).into();
/// assert_eq!(c, [24_u16, 1, 0, 0, 0, 0, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn min_position_u16_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_minpos_epu16(a.0) })
}

/// Computes eight `u16` "sum of absolute difference" values according to the
/// bytes selected.
///
/// * `a` can be 0 or 1, and specifies to skip the first fur `$a` values or not.
/// * `b` can be 0, 1, 2, or 3 and specifies to skip the first four times that
///   many values in `$b`.
///
/// This is used for some HD codec thing and I don't really get what the point
/// is but I'm sure someone uses it. If you can write better docs about what
/// this does please file a PR.
///
/// ```
/// # use safe_arch::*;
/// let a =
///   m128i::from([0_u8, 1, 56, 3, 255, 5, 127, 7, 128, 9, 100, 101, 123, 13, 154, 125]);
/// let b =
///   m128i::from([12_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// //
/// let c: [u16; 8] = multi_packed_sum_abs_diff_u8_m128i!(a, b, a 0, b 0).into();
/// assert_eq!(c, [66, 319, 301, 390, 376, 263, 253, 236]);
/// //
/// let c: [u16; 8] = multi_packed_sum_abs_diff_u8_m128i!(a, b, a 0, b 1).into();
/// assert_eq!(c, [62, 305, 305, 372, 372, 245, 249, 222]);
/// //
/// let c: [u16; 8] = multi_packed_sum_abs_diff_u8_m128i!(a, b, a 0, b 2).into();
/// assert_eq!(c, [70, 305, 305, 372, 372, 241, 241, 210]);
/// //
/// let c: [u16; 8] = multi_packed_sum_abs_diff_u8_m128i!(a, b, a 0, b 3).into();
/// assert_eq!(c, [78, 305, 305, 372, 372, 241, 241, 210]);
/// //
/// let c: [u16; 8] = multi_packed_sum_abs_diff_u8_m128i!(a, b, a 1, b 0).into();
/// assert_eq!(c, [376, 263, 253, 236, 320, 321, 319, 373]);
/// //
/// let c: [u16; 8] = multi_packed_sum_abs_diff_u8_m128i!(a, b, a 1, b 1).into();
/// assert_eq!(c, [372, 245, 249, 222, 316, 311, 315, 369]);
/// //
/// let c: [u16; 8] = multi_packed_sum_abs_diff_u8_m128i!(a, b, a 1, b 2).into();
/// assert_eq!(c, [372, 241, 241, 210, 300, 295, 299, 353]);
/// //
/// let c: [u16; 8] = multi_packed_sum_abs_diff_u8_m128i!(a, b, a 1, b 3).into();
/// assert_eq!(c, [372, 241, 241, 210, 292, 285, 287, 339]);
/// ```
#[macro_export]
macro_rules! multi_packed_sum_abs_diff_u8_m128i {
  ($a:expr, $b:expr, a $a_pick:expr, b $b_pick:expr) => {{
    let a: m128i = $a;
    let b: m128i = $b;
    const IMM: i32 = ((($a_pick & 0b1) << 2) | ($b_pick & 0b11)) as i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_mpsadbw_epu8;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_mpsadbw_epu8;
    m128i(unsafe { _mm_mpsadbw_epu8(a.0, b.0, IMM) })
  }};
}

/// Multiplies the lower 32 bits (only) of each `i64` lane into 64-bit `i64`
/// values.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i64, i32::MAX as i64]);
/// let b = m128i::from([5_i64, i32::MAX as i64]);
/// let c: [i64; 2] = mul_u64_widen_low_bits_m128i(a, b).into();
/// assert_eq!(c, [5_i64, (i32::MAX as i64 * i32::MAX as i64)]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn mul_i64_widen_low_bits_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_mul_epi32(a.0, b.0) })
}

/// Lanewise `a * b` with lanes as `i32`, keep the low bits of the `i64`
/// intermediates.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2000000, -300, 45689]);
/// let b = m128i::from([5, 6000000, 700, -89109]);
/// let c: [i32; 4] = mul_i32_keep_low_m128i(a, b).into();
/// assert_eq!(c, [5, -138625024, -210000, 223666195]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn mul_i32_keep_low_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_mullo_epi32(a.0, b.0) })
}

/// Saturating convert `i32` to `u16`, and pack the values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let b = m128i::from([9, -10, -11, i32::MAX]);
/// let c: [u16; 8] = pack_i32_to_u16_m128i(a, b).into();
/// assert_eq!(c, [1, 2, 3, 4, 9, 0, 0, u16::MAX]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn pack_i32_to_u16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_packus_epi32(a.0, b.0) })
}

/// Rounds each lane in the style specified.
///
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([-0.1, 1.6]);
/// //
/// assert_eq!(round_m128d!(a, Nearest).to_array(), [0.0, 2.0]);
/// //
/// assert_eq!(round_m128d!(a, NegInf).to_array(), [-1.0, 1.0]);
/// //
/// assert_eq!(round_m128d!(a, PosInf).to_array(), [0.0, 2.0]);
/// //
/// assert_eq!(round_m128d!(a, Zero).to_array(), [0.0, 1.0]);
/// ```
#[macro_export]
macro_rules! round_m128d {
  ($a:expr, Nearest) => {{
    let a: m128d = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm_round_pd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEAREST_INT,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm_round_pd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEAREST_INT,
    };
    m128d(unsafe {
      _mm_round_pd(a.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_NEAREST_INT)
    })
  }};
  ($a:expr, NegInf) => {{
    let a: m128d = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm_round_pd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEG_INF,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm_round_pd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEG_INF,
    };
    m128d(unsafe {
      _mm_round_pd(a.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_NEG_INF)
    })
  }};
  ($a:expr, PosInf) => {{
    let a: m128d = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm_round_pd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_POS_INF,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm_round_pd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_POS_INF,
    };
    m128d(unsafe {
      _mm_round_pd(a.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_POS_INF)
    })
  }};
  ($a:expr, Zero) => {{
    let a: m128d = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm_round_pd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_ZERO,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm_round_pd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_ZERO,
    };
    m128d(unsafe { _mm_round_pd(a.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_ZERO) })
  }};
}

/// Rounds `$b` low as specified, keeps `$a` high.
///
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([f64::NAN, 900.0]);
/// //
/// let b = m128d::from_array([-0.1, f64::NAN]);
/// //
/// assert_eq!(round_m128d_s!(a, b, Nearest).to_array(), [0.0, 900.0]);
/// assert_eq!(round_m128d_s!(a, b, NegInf).to_array(), [-1.0, 900.0]);
/// //
/// let b = m128d::from_array([2.4, f64::NAN]);
/// //
/// assert_eq!(round_m128d_s!(a, b, PosInf).to_array(), [3.0, 900.0]);
/// assert_eq!(round_m128d_s!(a, b, Zero).to_array(), [2.0, 900.0]);
/// ```
#[macro_export]
macro_rules! round_m128d_s {
  ($a:expr, $b:expr, Nearest) => {{
    let a: m128d = $a;
    let b: m128d = $b;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm_round_sd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEAREST_INT,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm_round_sd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEAREST_INT,
    };
    m128d(unsafe {
      _mm_round_sd(a.0, b.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_NEAREST_INT)
    })
  }};
  ($a:expr, $b:expr, NegInf) => {{
    let a: m128d = $a;
    let b: m128d = $b;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm_round_sd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEG_INF,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm_round_sd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEG_INF,
    };
    m128d(unsafe {
      _mm_round_sd(a.0, b.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_NEG_INF)
    })
  }};
  ($a:expr, $b:expr, PosInf) => {{
    let a: m128d = $a;
    let b: m128d = $b;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm_round_sd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_POS_INF,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm_round_sd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_POS_INF,
    };
    m128d(unsafe {
      _mm_round_sd(a.0, b.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_POS_INF)
    })
  }};
  ($a:expr, $b:expr, Zero) => {{
    let a: m128d = $a;
    let b: m128d = $b;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm_round_sd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_ZERO,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm_round_sd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_ZERO,
    };
    m128d(unsafe {
      _mm_round_sd(a.0, b.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_ZERO)
    })
  }};
}

/// Rounds each lane in the style specified.
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([-0.1, 1.6, 3.3, 4.5]);
/// //
/// assert_eq!(round_m128!(a, Nearest).to_array(), [0.0, 2.0, 3.0, 4.0]);
/// //
/// assert_eq!(round_m128!(a, NegInf).to_array(), [-1.0, 1.0, 3.0, 4.0]);
/// //
/// assert_eq!(round_m128!(a, PosInf).to_array(), [0.0, 2.0, 4.0, 5.0]);
/// //
/// assert_eq!(round_m128!(a, Zero).to_array(), [0.0, 1.0, 3.0, 4.0]);
/// ```
#[macro_export]
macro_rules! round_m128 {
  ($a:expr, Nearest) => {{
    let a: m128 = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm_round_ps, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEAREST_INT,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm_round_ps, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEAREST_INT,
    };
    m128(unsafe {
      _mm_round_ps(a.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_NEAREST_INT)
    })
  }};
  ($a:expr, NegInf) => {{
    let a: m128 = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm_round_ps, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEG_INF,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm_round_ps, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEG_INF,
    };
    m128(unsafe {
      _mm_round_ps(a.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_NEG_INF)
    })
  }};
  ($a:expr, PosInf) => {{
    let a: m128 = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm_round_ps, _MM_FROUND_NO_EXC, _MM_FROUND_TO_POS_INF,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm_round_ps, _MM_FROUND_NO_EXC, _MM_FROUND_TO_POS_INF,
    };
    m128(unsafe {
      _mm_round_ps(a.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_POS_INF)
    })
  }};
  ($a:expr, Zero) => {{
    let a: m128 = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm_round_ps, _MM_FROUND_NO_EXC, _MM_FROUND_TO_ZERO,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm_round_ps, _MM_FROUND_NO_EXC, _MM_FROUND_TO_ZERO,
    };
    m128(unsafe { _mm_round_ps(a.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_ZERO) })
  }};
}

/// Rounds `$b` low as specified, other lanes use `$a`.
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([f32::NAN, 6.0, 7.0, 8.0]);
/// //
/// let b = m128::from_array([-0.1, f32::NAN, f32::NAN, f32::NAN]);
/// //
/// assert_eq!(round_m128_s!(a, b, Nearest).to_array(), [0.0, 6.0, 7.0, 8.0]);
/// assert_eq!(round_m128_s!(a, b, NegInf).to_array(), [-1.0, 6.0, 7.0, 8.0]);
/// //
/// let b = m128::from_array([2.4, f32::NAN, f32::NAN, f32::NAN]);
/// //
/// assert_eq!(round_m128_s!(a, b, PosInf).to_array(), [3.0, 6.0, 7.0, 8.0]);
/// assert_eq!(round_m128_s!(a, b, Zero).to_array(), [2.0, 6.0, 7.0, 8.0]);
/// ```
#[macro_export]
macro_rules! round_m128_s {
  ($a:expr, $b:expr, Nearest) => {{
    let a: m128 = $a;
    let b: m128 = $b;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm_round_ss, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEAREST_INT,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm_round_ss, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEAREST_INT,
    };
    m128(unsafe {
      _mm_round_ss(a.0, b.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_NEAREST_INT)
    })
  }};
  ($a:expr, $b:expr, NegInf) => {{
    let a: m128 = $a;
    let b: m128 = $b;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm_round_ss, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEG_INF,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm_round_ss, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEG_INF,
    };
    m128(unsafe {
      _mm_round_ss(a.0, b.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_NEG_INF)
    })
  }};
  ($a:expr, $b:expr, PosInf) => {{
    let a: m128 = $a;
    let b: m128 = $b;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm_round_ss, _MM_FROUND_NO_EXC, _MM_FROUND_TO_POS_INF,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm_round_ss, _MM_FROUND_NO_EXC, _MM_FROUND_TO_POS_INF,
    };
    m128(unsafe {
      _mm_round_ss(a.0, b.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_POS_INF)
    })
  }};
  ($a:expr, $b:expr, Zero) => {{
    let a: m128 = $a;
    let b: m128 = $b;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm_round_ss, _MM_FROUND_NO_EXC, _MM_FROUND_TO_ZERO,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm_round_ss, _MM_FROUND_NO_EXC, _MM_FROUND_TO_ZERO,
    };
    m128(unsafe {
      _mm_round_ss(a.0, b.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_ZERO)
    })
  }};
}

/// Tests if all bits are 1.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(0_u128);
/// let b = m128i::from(u128::MAX);
/// assert_eq!(test_all_ones_m128i(a), 0);
/// assert_eq!(test_all_ones_m128i(b), 1);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn test_all_ones_m128i(a: m128i) -> i32 {
  unsafe { _mm_test_all_ones(a.0) }
}

/// Returns if all masked bits are 0, `(a & mask) as u128 == 0`
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(0b111_u128);
/// let mask = m128i::from(u128::MAX);
/// assert_eq!(test_all_zeroes_m128i(a, mask), 0);
/// //
/// let a = m128i::from(0b0_u128);
/// let mask = m128i::from(u128::MAX);
/// assert_eq!(test_all_zeroes_m128i(a, mask), 1);
/// //
/// let a = m128i::from(0b1_0000_u128);
/// let mask = m128i::from(0b0_1111_u128);
/// assert_eq!(test_all_zeroes_m128i(a, mask), 1);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn test_all_zeroes_m128i(a: m128i, mask: m128i) -> i32 {
  unsafe { _mm_test_all_zeros(a.0, mask.0) }
}

/// Returns if, among the masked bits, there's both 0s and 1s
///
/// * Zero Flag = `(a & mask) as u128 == 0`
/// * Carry Flag = `((!a) & mask) as u128 == 0`
/// * Return `ZeroFlag == 0 && Carry Flag == 0`
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(0b111_u128);
/// let mask = m128i::from(u128::MAX);
/// assert_eq!(test_mixed_ones_and_zeroes_m128i(a, mask), 1);
/// //
/// let a = m128i::from(0b0_u128);
/// let mask = m128i::from(u128::MAX);
/// assert_eq!(test_mixed_ones_and_zeroes_m128i(a, mask), 0);
/// //
/// let a = m128i::from(0b1_0000_u128);
/// let mask = m128i::from(0b0_1111_u128);
/// assert_eq!(test_mixed_ones_and_zeroes_m128i(a, mask), 0);
/// //
/// let a = m128i::from(0b1_0000_u128);
/// let mask = m128i::from(0b1_1111_u128);
/// assert_eq!(test_mixed_ones_and_zeroes_m128i(a, mask), 1);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn test_mixed_ones_and_zeroes_m128i(a: m128i, mask: m128i) -> i32 {
  unsafe { _mm_test_mix_ones_zeros(a.0, mask.0) }
}
