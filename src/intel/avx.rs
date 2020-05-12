#![cfg(target_feature = "avx")]

use super::*;

/// Lanewise `a + b` with `f64` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m256d::from_array([5.0, 6.0, 7.0, 8.5]);
/// let c = add_m256d(a, b).to_array();
/// assert_eq!(c, [6.0, 8.0, 10.0, 12.5]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn add_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_add_pd(a.0, b.0) })
}

/// Lanewise `a + b` with `f32` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 20.0, 30.0, 40.0, 50.0]);
/// let b = m256::from_array([5.0, 6.0, 7.0, 8.5, 90.0, 100.0, 110.0, 51.0]);
/// let c = add_m256(a, b).to_array();
/// assert_eq!(c, [6.0, 8.0, 10.0, 12.5, 110.0, 130.0, 150.0, 101.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn add_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_add_ps(a.0, b.0) })
}

/// Alternately, from the top, add `f64` then sub `f64`.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([10.0, 20.0, 30.0, 40.0]);
/// let b = m256d::from_array([100.0, 200.0, 300.0, 400.0]);
/// let c = add_sub_m256d(a, b).to_array();
/// assert_eq!(c, [-90.0, 220.0, -270.0, 440.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn add_sub_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_addsub_pd(a.0, b.0) })
}

/// Alternately, from the top, add `f32` then sub `f32`.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([10.0, 20.0, 30.0, 40.0, 1.0, 2.0, 3.0, 4.0]);
/// let b = m256::from_array([1.0, 20.0, 3.0, 40.0, 11.0, 12.0, 13.0, 14.0]);
/// let c = add_sub_m256(a, b).to_array();
/// assert_eq!(c, [9.0, 40.0, 27.0, 80.0, -10.0, 14.0, -10.0, 18.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn add_sub_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_addsub_ps(a.0, b.0) })
}

/// Bitwise `a & b`.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 0.0, 1.0, 0.0]);
/// let b = m256d::from_array([1.0, 1.0, 0.0, 0.0]);
/// let c = and_m256d(a, b).to_array();
/// assert_eq!(c, [1.0, 0.0, 0.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn and_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_and_pd(a.0, b.0) })
}

/// Bitwise `a & b`.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
/// let b = m256::from_array([1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0]);
/// let c = and_m256(a, b).to_array();
/// assert_eq!(c, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn and_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_and_ps(a.0, b.0) })
}

/// Bitwise `(!a) & b`.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 0.0, 1.0, 0.0]);
/// let b = m256d::from_array([1.0, 1.0, 0.0, 0.0]);
/// let c = and_m256d(a, b).to_array();
/// assert_eq!(c, [1.0, 0.0, 0.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn andnot_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_andnot_pd(a.0, b.0) })
}

/// Bitwise `(!a) & b`.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
/// let b = m256::from_array([1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0]);
/// let c = and_m256(a, b).to_array();
/// assert_eq!(c, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn andnot_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_andnot_ps(a.0, b.0) })
}

/// Blends the `f64` lanes according to the immediate mask.
///
/// Each bit 0 though 3 controls lane 0 through 3. Use 0 for the `$a` value and
/// 1 for the `$b` value.
///
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([10.0, 20.0, 30.0, 40.0]);
/// let b = m256d::from_array([100.0, 200.0, 300.0, 400.0]);
/// //
/// let c = blend_immediate_m256d!(a, b, 0b0110).to_array();
/// assert_eq!(c, [10.0, 200.0, 300.0, 40.0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! blend_immediate_m256d {
  ($a:expr, $b:expr, $imm:expr) => {{
    let a: m256d = $a;
    let b: m256d = $b;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm256_blend_pd;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm256_blend_pd;
    m256d(unsafe { _mm256_blend_pd(a.0, b.0, IMM) })
  }};
}

/// Blends the `f32` lanes according to the immediate mask.
///
/// Each bit 0 though 7 controls lane 0 through 7. Use 0 for the `$a` value and
/// 1 for the `$b` value.
///
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]);
/// let b =
///   m256::from_array([100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0]);
/// //
/// let c = blend_immediate_m256!(a, b, 0b0011_0110).to_array();
/// assert_eq!(c, [10.0, 200.0, 300.0, 40.0, 500.0, 600.0, 70.0, 80.0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! blend_immediate_m256 {
  ($a:expr, $b:expr, $imm:expr) => {{
    let a: m256 = $a;
    let b: m256 = $b;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm256_blend_ps;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm256_blend_ps;
    m256(unsafe { _mm256_blend_ps(a.0, b.0, IMM) })
  }};
}

/// Blend the lanes according to a runtime varying mask.
///
/// The sign bit of each lane in the `mask` value determines if the output
/// lane uses `a` (mask non-negative) or `b` (mask negative).
///
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([0.0, 1.0, 20.0, 30.0]);
/// let b = m256d::from_array([2.0, 3.0, 70.0, 80.0]);
/// let mask = m256d::from_array([-1.0, 0.0, 0.0, -1.0]);
/// let c = blend_varying_m256d(a, b, mask).to_array();
/// assert_eq!(c, [2.0, 1.0, 20.0, 80.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn blend_varying_m256d(a: m256d, b: m256d, mask: m256d) -> m256d {
  m256d(unsafe { _mm256_blendv_pd(a.0, b.0, mask.0) })
}

/// Blend the lanes according to a runtime varying mask.
///
/// The sign bit of each lane in the `mask` value determines if the output
/// lane uses `a` (mask non-negative) or `b` (mask negative).
///
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([0.0, 1.0, 2.0, 3.0, 8.0, 9.0, 10.0, 11.0]);
/// let b = m256::from_array([4.0, 5.0, 6.0, 7.0, -4.0, -5.0, -6.0, -7.0]);
/// let mask = m256::from_array([-1.0, 0.0, -1.0, 0.0, -1.0, -1.0, 0.0, 0.0]);
/// let c = blend_varying_m256(a, b, mask).to_array();
/// assert_eq!(c, [4.0, 1.0, 6.0, 3.0, -4.0, -5.0, 10.0, 11.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn blend_varying_m256(a: m256, b: m256, mask: m256) -> m256 {
  m256(unsafe { _mm256_blendv_ps(a.0, b.0, mask.0) })
}

/// Load an `m128d` and splat it to the lower and upper half of an `m256d`
///
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([0.0, 1.0]);
/// let b = load_m128d_splat_m256d(&a).to_array();
/// assert_eq!(b, [0.0, 1.0, 0.0, 1.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn load_m128d_splat_m256d(a: &m128d) -> m256d {
  m256d(unsafe { _mm256_broadcast_pd(&a.0) })
}

/// Load an `m128` and splat it to the lower and upper half of an `m256`
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([0.0, 1.0, 2.0, 3.0]);
/// let b = load_m128_splat_m256(&a).to_array();
/// assert_eq!(b, [0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn load_m128_splat_m256(a: &m128) -> m256 {
  m256(unsafe { _mm256_broadcast_ps(&a.0) })
}

/// Load an `f64` and splat it to all lanes of an `m256d`
///
/// ```
/// # use safe_arch::*;
/// let a = 1.0;
/// let b = load_f64_splat_m256d(&a).to_array();
/// assert_eq!(b, [1.0, 1.0, 1.0, 1.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn load_f64_splat_m256d(a: &f64) -> m256d {
  m256d(unsafe { _mm256_broadcast_sd(&a) })
}

/// Load an `f32` and splat it to all lanes of an `m256d`
///
/// ```
/// # use safe_arch::*;
/// let a = 1.0;
/// let b = load_f32_splat_m256(&a).to_array();
/// assert_eq!(b, [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn load_f32_splat_m256(a: &f32) -> m256 {
  m256(unsafe { _mm256_broadcast_ss(&a) })
}

/// Bit-preserving cast from `m256d` to `m256`.
///
/// ```
/// # use safe_arch::*;
/// let a = load_f64_splat_m256d(&1.0);
/// assert_eq!(
///   cast_from_m256d_to_m256(a).to_bits(),
///   [0, 0x3FF0_0000, 0, 0x3FF0_0000, 0, 0x3FF0_0000, 0, 0x3FF0_0000]
/// );
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn cast_from_m256d_to_m256(a: m256d) -> m256 {
  m256(unsafe { _mm256_castpd_ps(a.0) })
}

/// Bit-preserving cast from `m256d` to `m256i`.
///
/// ```
/// # use safe_arch::*;
/// let a = load_f64_splat_m256d(&1.0);
/// let b: [u32; 8] = cast_from_m256d_to_m256i(a).into();
/// assert_eq!(
///   b,
///   [0, 0x3FF0_0000, 0, 0x3FF0_0000, 0, 0x3FF0_0000, 0, 0x3FF0_0000]
/// );
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn cast_from_m256d_to_m256i(a: m256d) -> m256i {
  m256i(unsafe { _mm256_castpd_si256(a.0) })
}

/// Bit-preserving cast from `m256` to `m256i`.
///
/// ```
/// # use safe_arch::*;
/// let a = load_f32_splat_m256(&1.0);
/// let b: [u64; 4] = cast_from_m256_to_m256d(a).to_bits();
/// assert_eq!(
///   b,
///   [
///     0x3f800000_3f800000,
///     0x3f800000_3f800000,
///     0x3f800000_3f800000,
///     0x3f800000_3f800000,
///   ]
/// );
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn cast_from_m256_to_m256d(a: m256) -> m256d {
  m256d(unsafe { _mm256_castps_pd(a.0) })
}

/// Bit-preserving cast from `m256` to `m256i`.
///
/// ```
/// # use safe_arch::*;
/// let a = load_f32_splat_m256(&1.0);
/// let b: [u64; 4] = cast_from_m256_to_m256i(a).into();
/// assert_eq!(
///   b,
///   [
///     0x3f800000_3f800000,
///     0x3f800000_3f800000,
///     0x3f800000_3f800000,
///     0x3f800000_3f800000,
///   ]
/// );
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn cast_from_m256_to_m256i(a: m256) -> m256i {
  m256i(unsafe { _mm256_castps_si256(a.0) })
}

/// Bit-preserving cast from `m256i` to `m256d`.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1.0_f64.to_bits(); 4]);
/// let b = cast_from_m256i_to_m256d(a).to_array();
/// assert_eq!(b, [1.0; 4]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn cast_from_m256i_to_m256d(a: m256i) -> m256d {
  m256d(unsafe { _mm256_castsi256_pd(a.0) })
}

/// Bit-preserving cast from `m256i` to `m256`.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1.0_f32.to_bits(); 8]);
/// let b = cast_from_m256i_to_m256(a).to_array();
/// assert_eq!(b, [1.0; 8]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn cast_from_m256i_to_m256(a: m256i) -> m256 {
  m256(unsafe { _mm256_castsi256_ps(a.0) })
}

/// Round `f64` lanes towards positive infinity.
///
/// ```
/// # use safe_arch::*;
/// let a = m256d::from([1.1, 2.5, 3.8, 5.0]);
/// let b = ceil_m256d(a).to_array();
/// assert_eq!(b, [2.0, 3.0, 4.0, 5.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn ceil_m256d(a: m256d) -> m256d {
  m256d(unsafe { _mm256_ceil_pd(a.0) })
}

/// Round `f32` lanes towards positive infinity.
///
/// ```
/// # use safe_arch::*;
/// let a = m256::from([1.1, 2.5, 3.8, 5.0, -0.5, -1.1, -2.7, -3.0]);
/// let b = ceil_m256(a).to_array();
/// assert_eq!(b, [2.0, 3.0, 4.0, 5.0, 0.0, -1.0, -2.0, -3.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn ceil_m256(a: m256) -> m256 {
  m256(unsafe { _mm256_ceil_ps(a.0) })
}

/// Turns a comparison operator token to the correct constant value.
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! comparison_operator_translation {
  (EqualOrdered) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_CMP_EQ_OQ;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_CMP_EQ_OQ;
    _CMP_EQ_OQ
  }};
  (EqualUnordered) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_CMP_EQ_UQ;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_CMP_EQ_UQ;
    _CMP_EQ_UQ
  }};
  (False) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_CMP_FALSE_OQ;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_CMP_FALSE_OQ;
    _CMP_FALSE_OQ
  }};
  (GreaterEqualOrdered) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_CMP_GE_OQ;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_CMP_GE_OQ;
    _CMP_GE_OQ
  }};
  (GreaterThanOrdered) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_CMP_GT_OQ;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_CMP_GT_OQ;
    _CMP_GT_OQ
  }};
  (LessEqualOrdered) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_CMP_LE_OQ;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_CMP_LE_OQ;
    _CMP_LE_OQ
  }};
  (LessThanOrdered) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_CMP_LT_OQ;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_CMP_LT_OQ;
    _CMP_LT_OQ
  }};
  (NotEqualOrdered) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_CMP_NEQ_OQ;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_CMP_NEQ_OQ;
    _CMP_NEQ_OQ
  }};
  (NotEqualUnordered) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_CMP_NEQ_UQ;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_CMP_NEQ_UQ;
    _CMP_NEQ_UQ
  }};
  (NotGreaterEqualUnordered) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_CMP_NGE_UQ;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_CMP_NGE_UQ;
    _CMP_NGE_UQ
  }};
  (NotGreaterThanUnordered) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_CMP_NGT_UQ;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_CMP_NGT_UQ;
    _CMP_NGT_UQ
  }};
  (NotLessEqualUnordered) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_CMP_NLE_UQ;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_CMP_NLE_UQ;
    _CMP_NLE_UQ
  }};
  (NotLessThanUnordered) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_CMP_NLT_UQ;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_CMP_NLT_UQ;
    _CMP_NLT_UQ
  }};
  (Ordered) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_CMP_ORD_Q;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_CMP_ORD_Q;
    _CMP_ORD_Q
  }};
  (True) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_CMP_TRUE_UQ;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_CMP_TRUE_UQ;
    _CMP_TRUE_UQ
  }};
  (Unordered) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_CMP_UNORD_Q;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_CMP_UNORD_Q;
    _CMP_UNORD_Q
  }};
  ($unknown_op:tt) => {{
    compile_error!("The operation name given is invalid.");
  }};
}

/// Compare `f32` lanes according to the operation specified, mask output.
///
/// * Operators are according to the [`comparison_operator_translation`] macro.
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.0, 0.0, -2.0, 0.0]);
/// let b = m128::from_array([1.0, 1.0, -1.0, -1.0]);
/// let c = cmp_op_mask_m128!(a, GreaterThanOrdered, b).to_bits();
/// assert_eq!(c, [u32::MAX, 0, 0, u32::MAX]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! cmp_op_mask_m128 {
  ($a:expr, $op:tt, $b:expr) => {{
    $crate::cmp_op_mask_m128!(
      @_raw_call $a, $b,
      $crate::comparison_operator_translation!($op)
    )
  }};
  (@_raw_call $a:expr, $b:expr, $imm:expr) => {{
    let a: m128 = $a;
    let b: m128 = $b;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmp_ps;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmp_ps;
    m128(unsafe { _mm_cmp_ps(a.0, b.0, IMM) })
  }};
}

/// Compare `f32` lanes according to the operation specified, mask output.
///
/// * Operators are according to the [`comparison_operator_translation`] macro.
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.0, 0.0, -2.0, 0.0]);
/// let b = m128::from_array([1.0, 1.0, -1.0, -1.0]);
/// let c = cmp_op_mask_m128_s!(a, GreaterThanOrdered, b).to_bits();
/// assert_eq!(c, [u32::MAX, 0, (-2_f32).to_bits(), 0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! cmp_op_mask_m128_s {
  ($a:expr, $op:tt, $b:expr) => {{
    $crate::cmp_op_mask_m128_s!(
      @_raw_call $a, $b,
      $crate::comparison_operator_translation!($op)
    )
  }};
  (@_raw_call $a:expr, $b:expr, $imm:expr) => {{
    let a: m128 = $a;
    let b: m128 = $b;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmp_ss;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmp_ss;
    m128(unsafe { _mm_cmp_ss(a.0, b.0, IMM) })
  }};
}

/// Compare `f32` lanes according to the operation specified, mask output.
///
/// * Operators are according to the [`comparison_operator_translation`] macro.
///
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 5.0, 0.0, 7.0, 5.0, 6.0, 7.0, -20.0]);
/// let b = m256::from_array([2.0, 1.0, 3.0, 4.0, 1.0, -2.0, -3.0, -4.0]);
/// let c = cmp_op_mask_m256!(a, LessThanOrdered, b).to_bits();
/// assert_eq!(c, [u32::MAX, 0, u32::MAX, 0, 0, 0, 0, u32::MAX]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! cmp_op_mask_m256 {
  ($a:expr, $op:tt, $b:expr) => {{
    $crate::cmp_op_mask_m256!(
      @_raw_call $a, $b,
      $crate::comparison_operator_translation!($op)
    )
  }};
  (@_raw_call $a:expr, $b:expr, $imm:expr) => {{
    let a: m256 = $a;
    let b: m256 = $b;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm256_cmp_ps;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm256_cmp_ps;
    m256(unsafe { _mm256_cmp_ps(a.0, b.0, IMM) })
  }};
}

/// Compare `f64` lanes according to the operation specified, mask output.
///
/// * Operators are according to the [`comparison_operator_translation`] macro.
///
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_op_mask_m128d!(a, EqualOrdered, b).to_bits();
/// assert_eq!(c, [u64::MAX, 0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! cmp_op_mask_m128d {
  ($a:expr, $op:tt, $b:expr) => {{
    $crate::cmp_op_mask_m128d!(
      @_raw_call $a, $b,
      $crate::comparison_operator_translation!($op)
    )
  }};
  (@_raw_call $a:expr, $b:expr, $imm:expr) => {{
    let a: m128d = $a;
    let b: m128d = $b;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmp_pd;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmp_pd;
    m128d(unsafe { _mm_cmp_pd(a.0, b.0, IMM) })
  }};
}

/// Compare `f64` lanes according to the operation specified, mask output.
///
/// * Operators are according to the [`comparison_operator_translation`] macro.
///
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 7.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_op_mask_m128d_s!(a, EqualOrdered, b).to_bits();
/// assert_eq!(c, [u64::MAX, 7_f64.to_bits()]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! cmp_op_mask_m128d_s {
  ($a:expr, $op:tt, $b:expr) => {{
    $crate::cmp_op_mask_m128d_s!(
      @_raw_call $a, $b,
      $crate::comparison_operator_translation!($op)
    )
  }};
  (@_raw_call $a:expr, $b:expr, $imm:expr) => {{
    let a: m128d = $a;
    let b: m128d = $b;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmp_sd;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmp_sd;
    m128d(unsafe { _mm_cmp_sd(a.0, b.0, IMM) })
  }};
}

/// Compare `f64` lanes according to the operation specified, mask output.
///
/// * Operators are according to the [`comparison_operator_translation`] macro.
///
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 5.0, 0.0, 7.0]);
/// let b = m256d::from_array([2.0, 1.0, 3.0, 4.0]);
/// let c = cmp_op_mask_m256d!(a, LessThanOrdered, b).to_bits();
/// assert_eq!(c, [u64::MAX, 0, u64::MAX, 0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! cmp_op_mask_m256d {
  ($a:expr, $op:tt, $b:expr) => {{
    $crate::cmp_op_mask_m256d!(
      @_raw_call $a, $b,
      $crate::comparison_operator_translation!($op)
    )
  }};
  (@_raw_call $a:expr, $b:expr, $imm:expr) => {{
    let a: m256d = $a;
    let b: m256d = $b;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm256_cmp_pd;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm256_cmp_pd;
    m256d(unsafe { _mm256_cmp_pd(a.0, b.0, IMM) })
  }};
}

/// Convert `i32` lanes to be `f64` lanes.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([4, 5, 6, 7]);
/// let b = convert_to_m256d_from_i32_m128i(a).to_array();
/// assert_eq!(b, [4.0, 5.0, 6.0, 7.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_m256d_from_i32_m128i(a: m128i) -> m256d {
  m256d(unsafe { _mm256_cvtepi32_pd(a.0) })
}

/// Convert `i32` lanes to be `f32` lanes.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([4, 5, 6, 7, 8, -9, 1, 0]);
/// let b = convert_to_m256_from_i32_m256i(a).to_array();
/// assert_eq!(b, [4.0, 5.0, 6.0, 7.0, 8.0, -9.0, 1.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_m256_from_i32_m256i(a: m256i) -> m256 {
  m256(unsafe { _mm256_cvtepi32_ps(a.0) })
}

/// Convert `f64` lanes to be `i32` lanes.
///
/// ```
/// # use safe_arch::*;
/// let a = m256d::from([4.0, 5.0, 6.0, 7.0]);
/// let b: [i32; 4] = convert_to_m128i_from_m256d(a).into();
/// assert_eq!(b, [4, 5, 6, 7]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_m128i_from_m256d(a: m256d) -> m128i {
  m128i(unsafe { _mm256_cvtpd_epi32(a.0) })
}

/// Convert `f64` lanes to be `f32` lanes.
///
/// ```
/// # use safe_arch::*;
/// let a = m256d::from([4.0, 5.0, 6.0, 7.0]);
/// let b = convert_to_m128_from_m256d(a).to_array();
/// assert_eq!(b, [4.0, 5.0, 6.0, 7.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_m128_from_m256d(a: m256d) -> m128 {
  m128(unsafe { _mm256_cvtpd_ps(a.0) })
}

/// Convert `f32` lanes to be `i32` lanes.
///
/// ```
/// # use safe_arch::*;
/// let a = m256::from([4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0]);
/// let b: [i32; 8] = convert_to_m256i_from_m256(a).into();
/// assert_eq!(b, [4, 5, 6, 7, 8, 9, 10, 11]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_m256i_from_m256(a: m256) -> m256i {
  m256i(unsafe { _mm256_cvtps_epi32(a.0) })
}

/// Convert `f32` lanes to be `f64` lanes.
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from([4.0, 5.0, 6.0, 7.0]);
/// let b = convert_to_m256d_from_m128(a).to_array();
/// assert_eq!(b, [4.0, 5.0, 6.0, 7.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_m256d_from_m128(a: m128) -> m256d {
  m256d(unsafe { _mm256_cvtps_pd(a.0) })
}

/// Convert the lowest `f64` lane to a single `f64`.
///
/// ```
/// # use safe_arch::*;
/// let a = m256d::from([4.0, 5.0, 6.0, 7.0]);
/// let b = convert_to_f64_from_m256d_s(a);
/// assert_eq!(b, 4.0);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_f64_from_m256d_s(a: m256d) -> f64 {
  unsafe { _mm256_cvtsd_f64(a.0) }
}

/// Convert the lowest `f64` lane to a single `f64`.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([4, 5, 6, 7, 8, 9, 10, 11]);
/// let b = convert_to_i32_from_m256i_s(a);
/// assert_eq!(b, 4);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_i32_from_m256i_s(a: m256i) -> i32 {
  unsafe { _mm256_cvtsi256_si32(a.0) }
}

/// Convert the lowest `f64` lane to a single `f64`.
///
/// ```
/// # use safe_arch::*;
/// let a = m256::from([4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0]);
/// let b = convert_to_f32_from_m256_s(a);
/// assert_eq!(b, 4.0);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_f32_from_m256_s(a: m256) -> f32 {
  unsafe { _mm256_cvtss_f32(a.0) }
}

/// Convert `f64` lanes to `i32` lanes.
///
/// ```
/// # use safe_arch::*;
/// let a = m256d::from([4.0, 5.0, 6.0, 7.0]);
/// let b: [i32; 4] = convert_to_i32_m128i_from_m256d(a).into();
/// assert_eq!(b, [4, 5, 6, 7]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_i32_m128i_from_m256d(a: m256d) -> m128i {
  m128i(unsafe { _mm256_cvttpd_epi32(a.0) })
}

/// Convert `f32` lanes to `i32` lanes.
///
/// ```
/// # use safe_arch::*;
/// let a = m256::from([4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0]);
/// let b: [i32; 8] = convert_to_i32_m256i_from_m256(a).into();
/// assert_eq!(b, [4, 5, 6, 7, 8, 9, 10, 11]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn convert_to_i32_m256i_from_m256(a: m256) -> m256i {
  m256i(unsafe { _mm256_cvttps_epi32(a.0) })
}

/// Lanewise `a / b` with `f64`.
///
/// ```
/// # use safe_arch::*;
/// let a = m256d::from([4.0, 5.0, 6.0, 7.0]);
/// let b = m256d::from([2.0, 2.0, 3.0, 7.0]);
/// let c = div_m256d(a, b).to_array();
/// assert_eq!(c, [2.0, 2.5, 2.0, 1.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn div_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_div_pd(a.0, b.0) })
}

/// Lanewise `a / b` with `f32`.
///
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0]);
/// let b = m256::from_array([2.0, 2.0, 3.0, 7.0, 2.0, 3.0, 4.0, 11.0]);
/// let c = div_m256(a, b).to_array();
/// assert_eq!(c, [2.0, 2.5, 2.0, 1.0, 4.0, 3.0, 2.5, 1.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn div_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_div_ps(a.0, b.0) })
}

/// This works like [`dot_product_m128`], but twice as wide.
///
/// The given control is used for the lower 4 lanes and then separately also the
/// upper four lanes. See the other macro for more info on how the control
/// works.
///
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
/// let b = m256::from_array([9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
/// let c = dot_product_m256!(a, b, 0b1111_1111).to_array();
/// assert_eq!(c, [110.0, 110.0, 110.0, 110.0, 382.0, 382.0, 382.0, 382.0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! dot_product_m256 {
  ($a:expr, $b:expr, $imm:expr) => {{
    let a: m256 = $a;
    let b: m256 = $b;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm256_dp_ps;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm256_dp_ps;
    m256(unsafe { _mm256_dp_ps(a.0, b.0, IMM) })
  }};
}

/// Extracts an `i32` lane from `m256i`
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([9, 10, 11, 12, 13, 14, 15, 16]);
/// assert_eq!(extract_i32_from_m256i!(a, 3), 12);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! extract_i32_from_m256i {
  ($a:expr, $imm:expr) => {{
    let a: m256i = $a;
    const IMM: i32 = ($imm & 0b111) as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm256_extract_epi32;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm256_extract_epi32;
    unsafe { _mm256_extract_epi32(a.0, IMM) }
  }};
}

/// Extracts an `i64` lane from `m256i`
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([9_i64, 10, 11, 12]);
/// assert_eq!(extract_i64_from_m256i!(a, 1), 10_i64);
/// ```
#[macro_export]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! extract_i64_from_m256i {
  ($a:expr, $imm:expr) => {{
    let a: m256i = $a;
    const IMM: i32 = ($imm & 0b111) as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm256_extract_epi64;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm256_extract_epi64;
    unsafe { _mm256_extract_epi64(a.0, IMM) }
  }};
}

/// Extracts an `m128d` from `m256d`
///
/// ```
/// # use safe_arch::*;
/// let a = m256d::from([13.0, 14.0, 15.0, 16.0]);
/// let b = m128d::from([15.0, 16.0]).to_array();
/// let c = extract_m128d_from_m256d!(a, 1).to_array();
/// assert_eq!(b, c);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! extract_m128d_from_m256d {
  ($a:expr, $imm:expr) => {{
    let a: m256d = $a;
    const IMM: i32 = ($imm & 0b111) as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm256_extractf128_pd;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm256_extractf128_pd;
    m128d(unsafe { _mm256_extractf128_pd(a.0, IMM) })
  }};
}

/// Extracts an `m128` from `m256`
///
/// ```
/// # use safe_arch::*;
/// let a = m256::from([9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
/// let b = m128::from([13.0, 14.0, 15.0, 16.0]).to_array();
/// let c = extract_m128_from_m256!(a, 1).to_array();
/// assert_eq!(b, c);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! extract_m128_from_m256 {
  ($a:expr, $imm:expr) => {{
    let a: m256 = $a;
    const IMM: i32 = ($imm & 0b111) as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm256_extractf128_ps;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm256_extractf128_ps;
    m128(unsafe { _mm256_extractf128_ps(a.0, IMM) })
  }};
}

/// Extracts an `m128i` from `m256i`
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([9, 10, 11, 12, 13, 14, 15, 16]);
/// let b: [i32; 4] = m128i::from([13, 14, 15, 16]).into();
/// let c: [i32; 4] = extract_m128i_from_m256i!(a, 1).into();
/// assert_eq!(b, c);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! extract_m128i_from_m256i {
  ($a:expr, $imm:expr) => {{
    let a: m256i = $a;
    const IMM: i32 = ($imm & 0b111) as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm256_extractf128_si256;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm256_extractf128_si256;
    m128i(unsafe { _mm256_extractf128_si256(a.0, IMM) })
  }};
}

/// Round `f64` lanes towards negative infinity.
///
/// ```
/// # use safe_arch::*;
/// let a = m256d::from([1.1, 2.5, 3.8, 5.0]);
/// let b = floor_m256d(a).to_array();
/// assert_eq!(b, [1.0, 2.0, 3.0, 5.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn floor_m256d(a: m256d) -> m256d {
  m256d(unsafe { _mm256_floor_pd(a.0) })
}

/// Round `f32` lanes towards negative infinity.
///
/// ```
/// # use safe_arch::*;
/// let a = m256::from([1.1, 2.5, 3.8, 5.0, -0.5, -1.1, -2.7, -3.0]);
/// let b = floor_m256(a).to_array();
/// assert_eq!(b, [1.0, 2.0, 3.0, 5.0, -1.0, -2.0, -3.0, -3.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn floor_m256(a: m256) -> m256 {
  m256(unsafe { _mm256_floor_ps(a.0) })
}

/// Add adjacent `f64` lanes.
///
/// ```
/// # use safe_arch::*;
/// let a = m256d::from([1.0, 2.0, 3.0, 4.0]);
/// let b = m256d::from([1.0, 3.0, 5.0, 7.0]);
/// let c = add_horizontal_m256d(a, b).to_array();
/// assert_eq!(c, [3.0, 4.0, 7.0, 12.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn add_horizontal_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_hadd_pd(a.0, b.0) })
}

/// Add adjacent `f32` lanes.
///
/// ```
/// # use safe_arch::*;
/// let a = m256::from([8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
/// let b = m256::from([0.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0]);
/// let c = add_horizontal_m256(a, b).to_array();
/// assert_eq!(c, [15.0, 11.0, 2.0, 12.0, 7.0, 3.0, 48.0, 192.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn add_horizontal_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_hadd_ps(a.0, b.0) })
}

/// Subtract adjacent `f64` lanes.
///
/// ```
/// # use safe_arch::*;
/// let a = m256d::from([1.0, 2.0, 3.0, 4.0]);
/// let b = m256d::from([1.0, 3.0, 5.0, 70.0]);
/// let c = sub_horizontal_m256d(a, b).to_array();
/// assert_eq!(c, [-1.0, -2.0, -1.0, -65.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn sub_horizontal_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_hsub_pd(a.0, b.0) })
}

/// Subtract adjacent `f32` lanes.
///
/// ```
/// # use safe_arch::*;
/// let a = m256::from([8.0, 17.0, 6.0, 5.0, 4.0, 23.0, 2.0, 1.0]);
/// let b = m256::from([0.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0]);
/// let c = sub_horizontal_m256(a, b).to_array();
/// assert_eq!(c, [-9.0, 1.0, -2.0, -4.0, -19.0, 1.0, -16.0, -64.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn sub_horizontal_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_hsub_ps(a.0, b.0) })
}

/// Inserts an `i8` to `m256i`
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i8; 32]);
/// let b: [i8; 32] = insert_i8_to_m256i!(a, 5_i8, 3).into();
/// let c: [i8; 32] = m256i::from([
///   0_i8, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
///   0, 0, 0, 0, 0, 0, 0, 0,
/// ])
/// .into();
/// assert_eq!(b, c);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! insert_i8_to_m256i {
  ($a:expr, $i:expr, $imm:expr) => {{
    let a: m256i = $a;
    let i: i8 = $i;
    const IMM: i32 = ($imm & 0b11111) as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm256_insert_epi8;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm256_insert_epi8;
    m256i(unsafe { _mm256_insert_epi8(a.0, i, IMM) })
  }};
}

/// Inserts an `i16` to `m256i`
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i16; 16]);
/// let b: [i16; 16] = insert_i16_to_m256i!(a, 5_i16, 3).into();
/// let c: [i16; 16] =
///   m256i::from([0_i16, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).into();
/// assert_eq!(b, c);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! insert_i16_to_m256i {
  ($a:expr, $i:expr, $imm:expr) => {{
    let a: m256i = $a;
    let i: i16 = $i;
    const IMM: i32 = ($imm & 0b1111) as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm256_insert_epi16;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm256_insert_epi16;
    m256i(unsafe { _mm256_insert_epi16(a.0, i, IMM) })
  }};
}

/// Inserts an `i32` to `m256i`
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i32; 8]);
/// let b: [i32; 8] = insert_i32_to_m256i!(a, 5_i32, 3).into();
/// let c: [i32; 8] = m256i::from([0, 0, 0, 5, 0, 0, 0, 0]).into();
/// assert_eq!(b, c);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! insert_i32_to_m256i {
  ($a:expr, $i:expr, $imm:expr) => {{
    let a: m256i = $a;
    let i: i32 = $i;
    const IMM: i32 = ($imm & 0b111) as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm256_insert_epi32;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm256_insert_epi32;
    m256i(unsafe { _mm256_insert_epi32(a.0, i, IMM) })
  }};
}

/// Inserts an `i64` to `m256i`
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i64; 4]);
/// let b: [i64; 4] = insert_i64_to_m256i!(a, 5_i64, 3).into();
/// let c: [i64; 4] = m256i::from([0, 0, 0, 5_i64]).into();
/// assert_eq!(b, c);
/// ```
#[macro_export]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! insert_i64_to_m256i {
  ($a:expr, $i:expr, $imm:expr) => {{
    let a: m256i = $a;
    let i: i64 = $i;
    const IMM: i32 = ($imm & 0b11) as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm256_insert_epi64;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm256_insert_epi64;
    m256i(unsafe { _mm256_insert_epi64(a.0, i, IMM) })
  }};
}

/// Inserts an `m128d` to `m256d`
///
/// ```
/// # use safe_arch::*;
/// let a = m256d::from([0.0; 4]);
/// let b: [f64; 4] =
///   insert_m128d_to_m256d!(a, m128d::from([3.0, 4.0]), 1).to_array();
/// assert_eq!(b, [0.0, 0.0, 3.0, 4.0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! insert_m128d_to_m256d {
  ($a:expr, $b:expr, $imm:expr) => {{
    let a: m256d = $a;
    let b: m128d = $b;
    const IMM: i32 = ($imm & 0b1) as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm256_insertf128_pd;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm256_insertf128_pd;
    m256d(unsafe { _mm256_insertf128_pd(a.0, b.0, IMM) })
  }};
}

/// Inserts an `m128` to `m256`
///
/// ```
/// # use safe_arch::*;
/// let a = m256::from([0.0; 8]);
/// let b: [f32; 8] =
///   insert_m128_to_m256!(a, m128::from([1.0, 2.0, 3.0, 4.0]), 1).to_array();
/// assert_eq!(b, [0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! insert_m128_to_m256 {
  ($a:expr, $b:expr, $imm:expr) => {{
    let a: m256 = $a;
    let b: m128 = $b;
    const IMM: i32 = ($imm & 0b1) as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm256_insertf128_ps;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm256_insertf128_ps;
    m256(unsafe { _mm256_insertf128_ps(a.0, b.0, IMM) })
  }};
}

/// Inserts an `m128i` to `m256i`
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i32; 8]);
/// let b: [i32; 8] =
///   insert_m128i_to_m256i!(a, m128i::from([1, 2, 3, 4]), 1).into();
/// assert_eq!(b, [0, 0, 0, 0, 1, 2, 3, 4]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! insert_m128i_to_m256i {
  ($a:expr, $b:expr, $imm:expr) => {{
    let a: m256i = $a;
    let b: m128i = $b;
    const IMM: i32 = ($imm & 0b1) as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm256_insertf128_si256;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm256_insertf128_si256;
    m256i(unsafe { _mm256_insertf128_si256(a.0, b.0, IMM) })
  }};
}

/// Load data from memory into a register.
///
/// ```
/// # use safe_arch::*;
/// let a = m256d::from([8.0, 17.0, 6.0, 5.0]);
/// let b = load_m256d(&a);
/// assert_eq!(a.to_array(), b.to_array());
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn load_m256d(a: &m256d) -> m256d {
  m256d(unsafe { _mm256_load_pd(a as *const m256d as *const f64) })
}

/// Load data from memory into a register.
///
/// ```
/// # use safe_arch::*;
/// let a = m256::from([8.0, 17.0, 6.0, 5.0, 4.0, 23.0, 2.0, 1.0]);
/// let b = load_m256(&a);
/// assert_eq!(a.to_array(), b.to_array());
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn load_m256(a: &m256) -> m256 {
  m256(unsafe { _mm256_load_ps(a as *const m256 as *const f32) })
}

/// Load data from memory into a register.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([8, 17, 6, 5, 4, 23, 2, 1]);
/// let b = load_m256i(&a);
/// assert_eq!(<[i32; 8]>::from(a), <[i32; 8]>::from(b));
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn load_m256i(a: &m256i) -> m256i {
  m256i(unsafe { _mm256_load_si256(a as *const m256i as *const __m256i) })
}

/// Load data from memory into a register.
///
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   load_unaligned_m256d(&[8.0, 17.0, 6.0, 5.0]).to_array(),
///   [8.0, 17.0, 6.0, 5.0]
/// );
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn load_unaligned_m256d(a: &[f64; 4]) -> m256d {
  m256d(unsafe { _mm256_loadu_pd(a as *const [f64; 4] as *const f64) })
}

/// Load data from memory into a register.
///
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   load_unaligned_m256(&[8.0, 17.0, 6.0, 5.0, 1.0, 2.0, 3.0, 4.0]).to_array(),
///   [8.0, 17.0, 6.0, 5.0, 1.0, 2.0, 3.0, 4.0]
/// );
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn load_unaligned_m256(a: &[f32; 8]) -> m256 {
  m256(unsafe { _mm256_loadu_ps(a as *const [f32; 8] as *const f32) })
}

/// Load data from memory into a register.
///
/// ```
/// # use safe_arch::*;
/// assert_eq!(<[i8; 32]>::from(load_unaligned_m256i(&[7_i8; 32])), [7_i8; 32]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn load_unaligned_m256i(a: &[i8; 32]) -> m256i {
  m256i(unsafe { _mm256_loadu_si256(a as *const [i8; 32] as *const __m256i) })
}

/// Load data from memory into a register.
///
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   load_unaligned_hi_lo_m256d(&[3.0, 4.0], &[1.0, 2.0]).to_array(),
///   [1.0, 2.0, 3.0, 4.0]
/// );
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn load_unaligned_hi_lo_m256d(a: &[f64; 2], b: &[f64; 2]) -> m256d {
  m256d(unsafe {
    _mm256_loadu2_m128d(
      a as *const [f64; 2] as *const f64,
      b as *const [f64; 2] as *const f64,
    )
  })
}

/// Load data from memory into a register.
///
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   load_unaligned_hi_lo_m256(&[5.0, 6.0, 7.0, 8.0], &[1.0, 2.0, 3.0, 4.0])
///     .to_array(),
///   [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
/// );
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn load_unaligned_hi_lo_m256(a: &[f32; 4], b: &[f32; 4]) -> m256 {
  m256(unsafe {
    _mm256_loadu2_m128(
      a as *const [f32; 4] as *const f32,
      b as *const [f32; 4] as *const f32,
    )
  })
}

/// Load data from memory into a register.
///
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   <[i8; 32]>::from(load_unaligned_hi_lo_m256i(&[7_i8; 16], &[9_i8; 16])),
///   [
///     9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 7, 7, 7, 7, 7, 7, 7, 7,
///     7, 7, 7, 7, 7, 7, 7, 7,
///   ]
/// );
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn load_unaligned_hi_lo_m256i(a: &[i8; 16], b: &[i8; 16]) -> m256i {
  m256i(unsafe {
    _mm256_loadu2_m128i(
      a as *const [i8; 16] as *const __m128i,
      b as *const [i8; 16] as *const __m128i,
    )
  })
}

/// Load data from memory into a register according to a mask.
///
/// When the high bit of a mask lane isn't set the loaded lane will be zero.
///
/// ```
/// # use safe_arch::*;
/// let a = m128d::from([8.0, 17.0]);
/// let b = load_masked_m128d(&a, m128i::from([0_i64, -1])).to_array();
/// assert_eq!(b, [0.0, 17.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn load_masked_m128d(a: &m128d, mask: m128i) -> m128d {
  m128d(unsafe { _mm_maskload_pd(a as *const m128d as *const f64, mask.0) })
}

/// Load data from memory into a register according to a mask.
///
/// When the high bit of a mask lane isn't set the loaded lane will be zero.
///
/// ```
/// # use safe_arch::*;
/// let a = m256d::from([8.0, 17.0, 16.0, 20.0]);
/// let b = load_masked_m256d(&a, m256i::from([0_i64, -1, -1, 0])).to_array();
/// assert_eq!(b, [0.0, 17.0, 16.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn load_masked_m256d(a: &m256d, mask: m256i) -> m256d {
  m256d(unsafe { _mm256_maskload_pd(a as *const m256d as *const f64, mask.0) })
}

/// Load data from memory into a register according to a mask.
///
/// When the high bit of a mask lane isn't set the loaded lane will be zero.
///
/// ```
/// # use safe_arch::*;
/// let a = m128::from([8.0, 17.0, 16.0, 12.0]);
/// let b = load_masked_m128(&a, m128i::from([0, -1, -1, 0])).to_array();
/// assert_eq!(b, [0.0, 17.0, 16.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn load_masked_m128(a: &m128, mask: m128i) -> m128 {
  m128(unsafe { _mm_maskload_ps(a as *const m128 as *const f32, mask.0) })
}

/// Load data from memory into a register according to a mask.
///
/// When the high bit of a mask lane isn't set the loaded lane will be zero.
///
/// ```
/// # use safe_arch::*;
/// let a = m256::from([8.0, 17.0, 16.0, 20.0, 80.0, 1.0, 2.0, 3.0]);
/// let b =
///   load_masked_m256(&a, m256i::from([0, -1, -1, 0, -1, -1, 0, 0])).to_array();
/// assert_eq!(b, [0.0, 17.0, 16.0, 0.0, 80.0, 1.0, 0.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn load_masked_m256(a: &m256, mask: m256i) -> m256 {
  m256(unsafe { _mm256_maskload_ps(a as *const m256 as *const f32, mask.0) })
}

// _mm_maskstore_pd
// _mm256_maskstore_pd
// _mm_maskstore_ps
// _mm256_maskstore_ps

// _mm256_max_pd
// _mm256_max_ps

// _mm256_min_pd
// _mm256_min_ps

// _mm256_movedup_pd

// _mm256_movehdup_ps

// _mm256_moveldup_ps

// _mm256_movemask_pd
// _mm256_movemask_ps

// _mm256_mul_pd
// _mm256_mul_ps

// _mm256_or_pd
// _mm256_or_ps

// _mm_permute_pd
// _mm256_permute_pd
// _mm_permute_ps
// _mm256_permute_ps

// _mm256_permute2f128_pd
// _mm256_permute2f128_ps
// _mm256_permute2f128_si256

// _mm_permutevar_pd
// _mm256_permutevar_pd
// _mm_permutevar_ps
// _mm256_permutevar_ps

// _mm256_rcp_ps

// _mm256_round_pd
// _mm256_round_ps

// _mm256_rsqrt_ps

// _mm256_set_epi16
// _mm256_set_epi32
// _mm256_set_epi64x
// _mm256_set_epi8
// _mm256_set_m128
// _mm256_set_m128d
// _mm256_set_m128i
// _mm256_set_pd
// _mm256_set_ps

// _mm256_set1_epi16
// _mm256_set1_epi32
// _mm256_set1_epi64x
// _mm256_set1_epi8
// _mm256_set1_pd
// _mm256_set1_ps

// _mm256_setr_epi16
// _mm256_setr_epi32
// _mm256_setr_epi64x
// _mm256_setr_epi8
// _mm256_setr_m128
// _mm256_setr_m128d
// _mm256_setr_m128i
// _mm256_setr_pd
// _mm256_setr_ps

// _mm256_setzero_pd
// _mm256_setzero_ps
// _mm256_setzero_si256

// _mm256_shuffle_pd
// _mm256_shuffle_ps

// _mm256_sqrt_pd
// _mm256_sqrt_ps

// _mm256_store_pd
// _mm256_store_ps
// _mm256_store_si256

// _mm256_storeu_pd
// _mm256_storeu_ps
// _mm256_storeu_si256

// _mm256_storeu2_m128
// _mm256_storeu2_m128d
// _mm256_storeu2_m128i

// _mm256_stream_pd
// _mm256_stream_ps
// _mm256_stream_si256

// _mm256_sub_pd
// _mm256_sub_ps

// _mm_testc_pd
// _mm256_testc_pd
// _mm_testc_ps
// _mm256_testc_ps
// _mm256_testc_si256

// _mm_testnzc_pd
// _mm256_testnzc_pd
// _mm_testnzc_ps
// _mm256_testnzc_ps
// _mm256_testnzc_si256

// _mm_testz_pd
// _mm256_testz_pd
// _mm_testz_ps
// _mm256_testz_ps
// _mm256_testz_si256

// _mm256_unpackhi_pd
// _mm256_unpackhi_ps

// _mm256_unpacklo_pd
// _mm256_unpacklo_ps

// _mm256_xor_pd
// _mm256_xor_ps

// _mm256_zeroall
// _mm256_zeroupper

// _mm256_zextpd128_pd256
// _mm256_zextps128_ps256
// _mm256_zextsi128_si256
// TODO: if we have these we should also include the lane truncate ops
