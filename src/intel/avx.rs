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
#[cfg(target_feature = "sse")]
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

// _mm256_cvtepi32_pd
// _mm256_cvtepi32_ps
// _mm256_cvtpd_epi32
// _mm256_cvtpd_ps
// _mm256_cvtps_epi32
// _mm256_cvtps_pd
// _mm256_cvtsd_f64
// _mm256_cvtsi256_si32
// _mm256_cvtss_f32
// _mm256_cvttpd_epi32
// _mm256_cvttps_epi32

// _mm256_div_pd
// _mm256_div_ps

// _mm256_dp_ps

// _mm256_extract_epi32
// _mm256_extract_epi64
// _mm256_extractf128_pd
// _mm256_extractf128_ps
// _mm256_extractf128_si256

// _mm256_floor_pd
// _mm256_floor_ps

// _mm256_hadd_pd
// _mm256_hadd_ps

// _mm256_hsub_pd
// _mm256_hsub_ps

// _mm256_insert_epi16
// _mm256_insert_epi32
// _mm256_insert_epi64
// _mm256_insert_epi8
// _mm256_insertf128_pd
// _mm256_insertf128_ps
// _mm256_insertf128_si256

// _mm256_lddqu_si256

// _mm256_load_pd
// _mm256_load_ps
// _mm256_load_si256

// _mm256_loadu_pd
// _mm256_loadu_ps
// _mm256_loadu_si256

// _mm256_loadu2_m128
// _mm256_loadu2_m128d
// _mm256_loadu2_m128i

// _mm_maskload_pd
// _mm256_maskload_pd
// _mm_maskload_ps
// _mm256_maskload_ps

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

// _mm256_undefined_pd
// _mm256_undefined_ps
// _mm256_undefined_si256

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
