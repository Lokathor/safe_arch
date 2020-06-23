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
/// let c = addsub_m256d(a, b).to_array();
/// assert_eq!(c, [-90.0, 220.0, -270.0, 440.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn addsub_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_addsub_pd(a.0, b.0) })
}

/// Alternately, from the top, add `f32` then sub `f32`.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([10.0, 20.0, 30.0, 40.0, 1.0, 2.0, 3.0, 4.0]);
/// let b = m256::from_array([1.0, 20.0, 3.0, 40.0, 11.0, 12.0, 13.0, 14.0]);
/// let c = addsub_m256(a, b).to_array();
/// assert_eq!(c, [9.0, 40.0, 27.0, 80.0, -10.0, 14.0, -10.0, 18.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn addsub_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_addsub_ps(a.0, b.0) })
}

/// Bitwise `a & b`.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 0.0, 1.0, 0.0]);
/// let b = m256d::from_array([1.0, 1.0, 0.0, 0.0]);
/// let c = bitand_m256d(a, b).to_array();
/// assert_eq!(c, [1.0, 0.0, 0.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn bitand_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_and_pd(a.0, b.0) })
}

/// Bitwise `a & b`.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
/// let b = m256::from_array([1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0]);
/// let c = bitand_m256(a, b).to_array();
/// assert_eq!(c, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn bitand_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_and_ps(a.0, b.0) })
}

/// Bitwise `(!a) & b`.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 0.0, 1.0, 0.0]);
/// let b = m256d::from_array([1.0, 1.0, 0.0, 0.0]);
/// let c = bitandnot_m256d(a, b).to_array();
/// assert_eq!(c, [0.0, 1.0, 0.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn bitandnot_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_andnot_pd(a.0, b.0) })
}

/// Bitwise `(!a) & b`.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
/// let b = m256::from_array([1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0]);
/// let c = bitandnot_m256(a, b).to_array();
/// assert_eq!(c, [0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn bitandnot_m256(a: m256, b: m256) -> m256 {
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
/// let c = blend_imm_m256d!(a, b, 0b0110).to_array();
/// assert_eq!(c, [10.0, 200.0, 300.0, 40.0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! blend_imm_m256d {
  ($a:expr, $b:expr, $imm:expr) => {{
    let a: m256d = $a;
    let b: m256d = $b;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm256_blend_pd;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_blend_pd;
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
/// let c = blend_imm_m256!(a, b, 0b0011_0110).to_array();
/// assert_eq!(c, [10.0, 200.0, 300.0, 40.0, 500.0, 600.0, 70.0, 80.0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! blend_imm_m256 {
  ($a:expr, $b:expr, $imm:expr) => {{
    let a: m256 = $a;
    let b: m256 = $b;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm256_blend_ps;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_blend_ps;
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
    use ::core::arch::x86::_CMP_EQ_OQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_EQ_OQ;
    _CMP_EQ_OQ
  }};
  (EqualUnordered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_EQ_UQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_EQ_UQ;
    _CMP_EQ_UQ
  }};
  (False) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_FALSE_OQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_FALSE_OQ;
    _CMP_FALSE_OQ
  }};
  (GreaterEqualOrdered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_GE_OQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_GE_OQ;
    _CMP_GE_OQ
  }};
  (GreaterThanOrdered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_GT_OQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_GT_OQ;
    _CMP_GT_OQ
  }};
  (LessEqualOrdered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_LE_OQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_LE_OQ;
    _CMP_LE_OQ
  }};
  (LessThanOrdered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_LT_OQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_LT_OQ;
    _CMP_LT_OQ
  }};
  (NotEqualOrdered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_NEQ_OQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_NEQ_OQ;
    _CMP_NEQ_OQ
  }};
  (NotEqualUnordered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_NEQ_UQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_NEQ_UQ;
    _CMP_NEQ_UQ
  }};
  (NotGreaterEqualUnordered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_NGE_UQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_NGE_UQ;
    _CMP_NGE_UQ
  }};
  (NotGreaterThanUnordered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_NGT_UQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_NGT_UQ;
    _CMP_NGT_UQ
  }};
  (NotLessEqualUnordered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_NLE_UQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_NLE_UQ;
    _CMP_NLE_UQ
  }};
  (NotLessThanUnordered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_NLT_UQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_NLT_UQ;
    _CMP_NLT_UQ
  }};
  (Ordered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_ORD_Q;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_ORD_Q;
    _CMP_ORD_Q
  }};
  (True) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_TRUE_UQ;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_TRUE_UQ;
    _CMP_TRUE_UQ
  }};
  (Unordered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_CMP_UNORD_Q;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_CMP_UNORD_Q;
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
    use ::core::arch::x86::_mm_cmp_ps;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_cmp_ps;
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
    use ::core::arch::x86::_mm_cmp_ss;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_cmp_ss;
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
    use ::core::arch::x86::_mm256_cmp_ps;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_cmp_ps;
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
    use ::core::arch::x86::_mm_cmp_pd;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_cmp_pd;
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
    use ::core::arch::x86::_mm_cmp_sd;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_cmp_sd;
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
    use ::core::arch::x86::_mm256_cmp_pd;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_cmp_pd;
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
    use ::core::arch::x86::_mm256_dp_ps;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_dp_ps;
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
    use ::core::arch::x86::_mm256_extract_epi32;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_extract_epi32;
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
    use ::core::arch::x86::_mm256_extract_epi64;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_extract_epi64;
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
    use ::core::arch::x86::_mm256_extractf128_pd;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_extractf128_pd;
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
    use ::core::arch::x86::_mm256_extractf128_ps;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_extractf128_ps;
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
    use ::core::arch::x86::_mm256_extractf128_si256;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_extractf128_si256;
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
    use ::core::arch::x86::_mm256_insert_epi8;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_insert_epi8;
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
    use ::core::arch::x86::_mm256_insert_epi16;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_insert_epi16;
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
    use ::core::arch::x86::_mm256_insert_epi32;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_insert_epi32;
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
    use ::core::arch::x86::_mm256_insert_epi64;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_insert_epi64;
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
    use ::core::arch::x86::_mm256_insertf128_pd;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_insertf128_pd;
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
    use ::core::arch::x86::_mm256_insertf128_ps;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_insertf128_ps;
    m256(unsafe { _mm256_insertf128_ps(a.0, b.0, IMM) })
  }};
}

/// Slowly inserts an `m128i` to `m256i`.
///
/// This is a "historical artifact" that was potentially useful if you have AVX
/// but not AVX2. If you plan on having AVX2 available please use
/// [`insert_m128i_to_m256i`], it will do the same task with better performance.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i32; 8]);
/// let b: [i32; 8] =
///   insert_m128i_to_m256i_slow_avx!(a, m128i::from([1, 2, 3, 4]), 1).into();
/// assert_eq!(b, [0, 0, 0, 0, 1, 2, 3, 4]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! insert_m128i_to_m256i_slow_avx {
  ($a:expr, $b:expr, $imm:expr) => {{
    let a: m256i = $a;
    let b: m128i = $b;
    const IMM: i32 = ($imm & 0b1) as i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm256_insertf128_si256;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_insertf128_si256;
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

/// Store data from a register into memory according to a mask.
///
/// When the high bit of a mask lane isn't set that lane is not written.
///
/// ```
/// # use safe_arch::*;
/// let mut a = m128d::default();
/// store_masked_m128d(
///   &mut a,
///   m128i::from([0_i64, -1]),
///   m128d::from([8.0, 17.0]),
/// );
/// assert_eq!(a.to_array(), [0.0, 17.0]);
/// ```
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn store_masked_m128d(addr: &mut m128d, mask: m128i, a: m128d) {
  unsafe { _mm_maskstore_pd(addr as *mut m128d as *mut f64, mask.0, a.0) }
}

/// Store data from a register into memory according to a mask.
///
/// When the high bit of a mask lane isn't set that lane is not written.
///
/// ```
/// # use safe_arch::*;
/// let mut a = m256d::default();
/// store_masked_m256d(
///   &mut a,
///   m256i::from([0_i64, -1, -1, 0]),
///   m256d::from([8.0, 17.0, 16.0, 20.0]),
/// );
/// assert_eq!(a.to_array(), [0.0, 17.0, 16.0, 0.0]);
/// ```
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn store_masked_m256d(addr: &mut m256d, mask: m256i, a: m256d) {
  unsafe { _mm256_maskstore_pd(addr as *mut m256d as *mut f64, mask.0, a.0) }
}

/// Store data from a register into memory according to a mask.
///
/// When the high bit of a mask lane isn't set that lane is not written.
///
/// ```
/// # use safe_arch::*;
/// let mut a = m128::default();
/// store_masked_m128(
///   &mut a,
///   m128i::from([0, -1, -1, 0]),
///   m128::from([8.0, 17.0, 16.0, 20.0]),
/// );
/// assert_eq!(a.to_array(), [0.0, 17.0, 16.0, 0.0]);
/// ```
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn store_masked_m128(addr: &mut m128, mask: m128i, a: m128) {
  unsafe { _mm_maskstore_ps(addr as *mut m128 as *mut f32, mask.0, a.0) }
}

/// Store data from a register into memory according to a mask.
///
/// When the high bit of a mask lane isn't set that lane is not written.
///
/// ```
/// # use safe_arch::*;
/// let mut a = m256::default();
/// store_masked_m256(
///   &mut a,
///   m256i::from([0, -1, -1, 0, -1, -1, 0, 0]),
///   m256::from([8.0, 17.0, 16.0, 20.0, 80.0, 1.0, 2.0, 3.0]),
/// );
/// assert_eq!(a.to_array(), [0.0, 17.0, 16.0, 0.0, 80.0, 1.0, 0.0, 0.0]);
/// ```
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn store_masked_m256(addr: &mut m256, mask: m256i, a: m256) {
  unsafe { _mm256_maskstore_ps(addr as *mut m256 as *mut f32, mask.0, a.0) }
}

/// Lanewise `max(a, b)`.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 12.0, -1.0, 3.0]);
/// let b = m256d::from_array([5.0, 6.0, -0.5, 2.2]);
/// let c = max_m256d(a, b).to_array();
/// assert_eq!(c, [5.0, 12.0, -0.5, 3.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn max_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_max_pd(a.0, b.0) })
}

/// Lanewise `max(a, b)`.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 12.0, -1.0, 3.0, 10.0, 0.0, 1.0, 2.0]);
/// let b = m256::from_array([5.0, 6.0, -0.5, 2.2, 5.0, 6.0, 7.0, 8.0]);
/// let c = max_m256(a, b).to_array();
/// assert_eq!(c, [5.0, 12.0, -0.5, 3.0, 10.0, 6.0, 7.0, 8.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn max_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_max_ps(a.0, b.0) })
}

/// Lanewise `min(a, b)`.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 12.0, -1.0, 3.0]);
/// let b = m256d::from_array([5.0, 6.0, -0.5, 2.2]);
/// let c = min_m256d(a, b).to_array();
/// assert_eq!(c, [1.0, 6.0, -1.0, 2.2]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn min_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_min_pd(a.0, b.0) })
}

/// Lanewise `min(a, b)`.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 12.0, -1.0, 3.0, 10.0, 0.0, 1.0, 2.0]);
/// let b = m256::from_array([5.0, 6.0, -0.5, 2.2, 5.0, 6.0, 7.0, 8.0]);
/// let c = min_m256(a, b).to_array();
/// assert_eq!(c, [1.0, 6.0, -1.0, 2.2, 5.0, 0.0, 1.0, 2.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn min_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_min_ps(a.0, b.0) })
}

/// Duplicate the odd-indexed lanes to the even lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 12.0, -1.0, 3.0]);
/// let c = duplicate_odd_lanes_m256d(a).to_array();
/// assert_eq!(c, [1.0, 1.0, -1.0, -1.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn duplicate_odd_lanes_m256d(a: m256d) -> m256d {
  m256d(unsafe { _mm256_movedup_pd(a.0) })
}

/// Duplicate the even-indexed lanes to the odd lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 12.0, -1.0, 3.0, 0.0, 7.0, 2.0, 50.0]);
/// let c = duplicate_even_lanes_m256(a).to_array();
/// assert_eq!(c, [12.0, 12.0, 3.0, 3.0, 7.0, 7.0, 50.0, 50.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn duplicate_even_lanes_m256(a: m256) -> m256 {
  m256(unsafe { _mm256_movehdup_ps(a.0) })
}

/// Duplicate the odd-indexed lanes to the even lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 12.0, -1.0, 3.0, 0.0, 7.0, 2.0, 50.0]);
/// let c = duplicate_odd_lanes_m256(a).to_array();
/// assert_eq!(c, [1.0, 1.0, -1.0, -1.0, 0.0, 0.0, 2.0, 2.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn duplicate_odd_lanes_m256(a: m256) -> m256 {
  m256(unsafe { _mm256_moveldup_ps(a.0) })
}

/// Collects the sign bit of each lane into a 4-bit value.
/// ```
/// # use safe_arch::*;
/// assert_eq!(0b0100, move_mask_m256d(m256d::from([1.0, 12.0, -1.0, 3.0])));
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn move_mask_m256d(a: m256d) -> i32 {
  unsafe { _mm256_movemask_pd(a.0) }
}

/// Collects the sign bit of each lane into a 4-bit value.
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   0b00110100,
///   move_mask_m256(m256::from([1.0, 12.0, -1.0, 3.0, -1.0, -2.0, 3.0, 4.0]))
/// );
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn move_mask_m256(a: m256) -> i32 {
  unsafe { _mm256_movemask_ps(a.0) }
}

/// Lanewise `a * b` with `f64` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m256d::from_array([5.0, 6.0, 7.0, 8.5]);
/// let c = mul_m256d(a, b).to_array();
/// assert_eq!(c, [5.0, 12.0, 21.0, 34.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn mul_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_mul_pd(a.0, b.0) })
}

/// Lanewise `a * b` with `f32` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 20.0, 30.0, 40.0, 50.0]);
/// let b = m256::from_array([5.0, 6.0, 7.0, 8.5, 90.0, 100.0, 110.0, 51.0]);
/// let c = mul_m256(a, b).to_array();
/// assert_eq!(c, [5.0, 12.0, 21.0, 34.0, 1800.0, 3000.0, 4400.0, 2550.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn mul_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_mul_ps(a.0, b.0) })
}

/// Bitwise `a | b`.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 1.0, 0.0, 0.0]);
/// let b = m256d::from_array([1.0, 0.0, 1.0, 0.0]);
/// let c = bitor_m256d(a, b).to_array();
/// assert_eq!(c, [1.0, 1.0, 1.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn bitor_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_or_pd(a.0, b.0) })
}

/// Bitwise `a | b`.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0]);
/// let b = m256::from_array([1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
/// let c = bitor_m256(a, b).to_array();
/// assert_eq!(c, [1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn bitor_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_or_ps(a.0, b.0) })
}

/// Permutes the lanes around.
///
/// * Different from "shuffle" because there is only one input.
/// * Generally gives better overall performance than shuffle if it's available
///   because it reduces register pressure.
///
/// This is a macro because the shuffle pattern must be a compile time constant,
/// and Rust doesn't currently support that for functions.
///
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 2.0]);
/// //
/// let b = permute_m128d!(a, 0, 0).to_array();
/// assert_eq!(b, [1.0, 1.0]);
/// //
/// let b = permute_m128d!(a, 0, 1).to_array();
/// assert_eq!(b, [1.0, 2.0]);
/// //
/// let b = permute_m128d!(a, 1, 0).to_array();
/// assert_eq!(b, [2.0, 1.0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! permute_m128d {
  ($a:expr, $z:expr, $o:expr) => {{
    const MASK: i32 = (($z & 0b1) | ($o & 0b1) << 1) as i32;
    let a: m128d = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_permute_pd;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_permute_pd;
    m128d(unsafe { _mm_permute_pd(a.0, MASK) })
  }};
}

/// Permutes the lanes around.
///
/// * Each index is 0 or 1, picking the low or high lane of the associated
///   128-bit portion of that index.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
/// //
/// let b = permute_within_m128d_m256d!(a, 0, 0, 0, 0).to_array();
/// assert_eq!(b, [1.0, 1.0, 3.0, 3.0]);
/// //
/// let b = permute_within_m128d_m256d!(a, 0, 1, 0, 1).to_array();
/// assert_eq!(b, [1.0, 2.0, 3.0, 4.0]);
/// //
/// let b = permute_within_m128d_m256d!(a, 1, 0, 1, 1).to_array();
/// assert_eq!(b, [2.0, 1.0, 4.0, 4.0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! permute_within_m128d_m256d {
  ($a:expr, $z:expr, $o:expr, $t:expr, $h:expr) => {{
    const MASK: i32 =
      (($z & 0b1) | ($o & 0b1) << 1 | ($t & 0b1) << 2 | ($h & 0b1) << 3) as i32;
    let a: m256d = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm256_permute_pd;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_permute_pd;
    m256d(unsafe { _mm256_permute_pd(a.0, MASK) })
  }};
}

/// Permutes the lanes around.
///
/// * Different from "shuffle" because there is only one input.
/// * Generally gives better overall performance than shuffle if it's available
///   because it reduces register pressure.
/// * The permute has to be a const.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// //
/// let b = permute_m128!(a, 0, 0, 0, 0).to_array();
/// assert_eq!(b, [1.0, 1.0, 1.0, 1.0]);
/// //
/// let b = permute_m128!(a, 0, 1, 0, 3).to_array();
/// assert_eq!(b, [1.0, 2.0, 1.0, 4.0]);
/// //
/// let b = permute_m128!(a, 0, 0, 2, 2).to_array();
/// assert_eq!(b, [1.0, 1.0, 3.0, 3.0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! permute_m128 {
  ($a:expr, $z:expr, $o:expr, $t:expr, $h:expr) => {{
    const MASK: i32 =
      (($z & 0b11) | ($o & 0b11) << 2 | ($t & 0b11) << 4 | ($h & 0b11) << 6)
        as i32;
    let a: m128 = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_permute_ps;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_permute_ps;
    m128(unsafe { _mm_permute_ps(a.0, MASK) })
  }};
}

/// Permutes the lanes around.
///
/// * Different from "shuffle" because there is only one input.
/// * You can't move values between the high and low 128-bit segments.
/// * Each index is `0..=3`, and selects the index only from that 128-bit half
///   of the overall 256 bits involved.
/// * Generally gives better overall performance than shuffle if it can
///   accomplish the movement that you want.
/// * The shuffle pattern must be a const.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
/// //
/// let b = permute_m256!(a, 0, 0, 0, 0).to_array();
/// assert_eq!(b, [1.0, 1.0, 1.0, 1.0, 5.0, 5.0, 5.0, 5.0]);
/// //
/// let b = permute_m256!(a, 0, 1, 0, 3).to_array();
/// assert_eq!(b, [1.0, 2.0, 1.0, 4.0, 5.0, 6.0, 5.0, 8.0]);
/// //
/// let b = permute_m256!(a, 0, 0, 2, 2).to_array();
/// assert_eq!(b, [1.0, 1.0, 3.0, 3.0, 5.0, 5.0, 7.0, 7.0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! permute_m256 {
  ($a:expr, $z:expr, $o:expr, $t:expr, $h:expr) => {{
    const MASK: i32 =
      (($z & 0b11) | ($o & 0b11) << 2 | ($t & 0b11) << 4 | ($h & 0b11) << 6)
        as i32;
    let a: m256 = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm256_permute_ps;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_permute_ps;
    m256(unsafe { _mm256_permute_ps(a.0, MASK) })
  }};
}

/// Permutes the lanes around.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m256d::from_array([5.0, 6.0, 7.0, 8.0]);
/// //
/// let c = permute_f128_in_m256d!(a, b, 2, Clear).to_array();
/// assert_eq!(c, [5.0, 6.0, 0.0, 0.0]);
/// //
/// let c = permute_f128_in_m256d!(a, b, 0, 1).to_array();
/// assert_eq!(c, [1.0, 2.0, 3.0, 4.0]);
/// //
/// let c = permute_f128_in_m256d!(a, b, Clear, 3).to_array();
/// assert_eq!(c, [0.0, 0.0, 7.0, 8.0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! permute_f128_in_m256d {
  ($a:expr, $b:expr, $low:tt, $high:tt) => {{
    const MASK: i32 =
      $crate::permute_f128_in_m256d!(@_convert_tt_to_select $low) |
      ($crate::permute_f128_in_m256d!(@_convert_tt_to_select $high) << 4);
    let a: m256d = $a;
    let b: m256d = $b;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm256_permute2f128_pd;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_permute2f128_pd;
    m256d(unsafe { _mm256_permute2f128_pd(a.0, b.0, MASK) })
  }};
  (@_convert_tt_to_select 0) => {
    0
  };
  (@_convert_tt_to_select 1) => {
    1
  };
  (@_convert_tt_to_select 2) => {
    2
  };
  (@_convert_tt_to_select 3) => {
    3
  };
  (@_convert_tt_to_select Clear) => {
    0b1000
  };
  (@_convert_tt_to_select $unknown:tt) => {
    compile_error!("Illegal select value, must be 0 ..= 3 or Clear.");
  };
}

/// Permutes the lanes around.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
/// let b = m256::from_array([9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
/// //
/// let c = permute_f128_in_m256!(a, b, 2, Clear).to_array();
/// assert_eq!(c, [9.0, 10.0, 11.0, 12.0, 0.0, 0.0, 0.0, 0.0]);
/// //
/// let c = permute_f128_in_m256!(a, b, 0, 1).to_array();
/// assert_eq!(c, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
/// //
/// let c = permute_f128_in_m256!(a, b, Clear, 3).to_array();
/// assert_eq!(c, [0.0, 0.0, 0.0, 0.0, 13.0, 14.0, 15.0, 16.0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! permute_f128_in_m256 {
  ($a:expr, $b:expr, $low:tt, $high:tt) => {{
  const MASK: i32 = $crate::permute_f128_in_m256!(@_convert_tt_to_select $low)
    | ($crate::permute_f128_in_m256!(@_convert_tt_to_select $high) << 4);
    let a: m256 = $a;
    let b: m256 = $b;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm256_permute2f128_ps;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_permute2f128_ps;
    m256(unsafe { _mm256_permute2f128_ps(a.0, b.0, MASK) })
  }};
  (@_convert_tt_to_select 0) => {
    0
  };
  (@_convert_tt_to_select 1) => {
    1
  };
  (@_convert_tt_to_select 2) => {
    2
  };
  (@_convert_tt_to_select 3) => {
    3
  };
  (@_convert_tt_to_select Clear) => {
    0b1000
  };
  (@_convert_tt_to_select $unknown:tt) => {
    compile_error!("Illegal select value, must be 0 ..= 3 or Clear.");
  };
}

/// Permutes the lanes around.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1, 2, 3, 4, 5, 6, 7, 8]);
/// let b = m256i::from([9, 10, 11, 12, 13, 14, 15, 16]);
/// //
/// let c: [i32; 8] = permute_i128_in_m256i!(a, b, 2, Clear).into();
/// assert_eq!(c, [9, 10, 11, 12, 0, 0, 0, 0]);
/// //
/// let c: [i32; 8] = permute_i128_in_m256i!(a, b, 0, 1).into();
/// assert_eq!(c, [1, 2, 3, 4, 5, 6, 7, 8]);
/// //
/// let c: [i32; 8] = permute_i128_in_m256i!(a, b, Clear, 3).into();
/// assert_eq!(c, [0, 0, 0, 0, 13, 14, 15, 16]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! permute_i128_in_m256i {
  ($a:expr, $b:expr, $low:tt, $high:tt) => {{
  const MASK: i32 = $crate::permute_i128_in_m256i!(@_convert_tt_to_select $low)
    | ($crate::permute_i128_in_m256i!(@_convert_tt_to_select $high) << 4);
    let a: m256i = $a;
    let b: m256i = $b;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm256_permute2f128_si256;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_permute2f128_si256;
    m256i(unsafe { _mm256_permute2f128_si256(a.0, b.0, MASK) })
  }};
  (@_convert_tt_to_select 0) => {
    0
  };
  (@_convert_tt_to_select 1) => {
    1
  };
  (@_convert_tt_to_select 2) => {
    2
  };
  (@_convert_tt_to_select 3) => {
    3
  };
  (@_convert_tt_to_select Clear) => {
    0b1000
  };
  (@_convert_tt_to_select $unknown:tt) => {
    compile_error!("Illegal select value, must be 0 ..= 3 or Clear.");
  };
}

/// Permute with a runtime varying pattern.
///
/// For whatever reason, **bit 1** in each `i64` lane is the selection bit.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 3.0]);
/// let b = m128i::from([1_i64 << 1, 0 << 1]);
/// let c = permute_varying_m128d(a, b).to_array();
/// assert_eq!(c, [3.0, 2.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn permute_varying_m128d(a: m128d, b: m128i) -> m128d {
  m128d(unsafe { _mm_permutevar_pd(a.0, b.0) })
}

/// Permute with a runtime varying pattern.
///
/// For whatever reason, **bit 1** in each `i64` lane is the selection bit.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([2.0, 3.0, 7.0, 8.0]);
/// let b = m256i::from([1_i64 << 1, 0 << 1, 1 << 1, 1 << 1]);
/// let c = permute_varying_m256d(a, b).to_array();
/// assert_eq!(c, [3.0, 2.0, 8.0, 8.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn permute_varying_m256d(a: m256d, b: m256i) -> m256d {
  m256d(unsafe { _mm256_permutevar_pd(a.0, b.0) })
}

/// Permute with a runtime varying pattern.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([0.0, 1.0, 2.0, 3.0]);
/// let b = m128i::from([0, 2, 3, 1]);
/// let c = permute_varying_m128(a, b).to_array();
/// assert_eq!(c, [0.0, 2.0, 3.0, 1.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn permute_varying_m128(a: m128, b: m128i) -> m128 {
  m128(unsafe { _mm_permutevar_ps(a.0, b.0) })
}

/// Permute with a runtime varying pattern.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
/// let b = m256i::from([0, 2, 3, 1, 0, 3, 2, 2]);
/// let c = permute_varying_m256(a, b).to_array();
/// assert_eq!(c, [0.0, 2.0, 3.0, 1.0, 4.0, 7.0, 6.0, 6.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn permute_varying_m256(a: m256, b: m256i) -> m256 {
  m256(unsafe { _mm256_permutevar_ps(a.0, b.0) })
}

/// Reciprocal of `f32` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 2.0, 4.0, 8.0, 0.5, 2.0, 8.0, 16.0]);
/// let b = reciprocal_m256(a).to_array();
/// let expected = [1.0, 0.5, 0.25, 0.125, 2.0, 0.5, 0.125, 0.0625];
/// for i in 0..4 {
///   assert!((b[i] - expected[i]).abs() < 0.001);
/// }
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn reciprocal_m256(a: m256) -> m256 {
  m256(unsafe { _mm256_rcp_ps(a.0) })
}

/// Rounds each lane in the style specified.
///
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([-0.1, 1.6, 2.5, 3.1]);
/// //
/// assert_eq!(round_m256d!(a, Nearest).to_array(), [0.0, 2.0, 2.0, 3.0]);
/// //
/// assert_eq!(round_m256d!(a, NegInf).to_array(), [-1.0, 1.0, 2.0, 3.0]);
/// //
/// assert_eq!(round_m256d!(a, PosInf).to_array(), [0.0, 2.0, 3.0, 4.0]);
/// //
/// assert_eq!(round_m256d!(a, Zero).to_array(), [0.0, 1.0, 2.0, 3.0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! round_m256d {
  ($a:expr, Nearest) => {{
    let a: m256d = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm256_round_pd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEAREST_INT,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm256_round_pd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEAREST_INT,
    };
    m256d(unsafe {
      _mm256_round_pd(a.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_NEAREST_INT)
    })
  }};
  ($a:expr, NegInf) => {{
    let a: m256d = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm256_round_pd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEG_INF,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm256_round_pd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEG_INF,
    };
    m256d(unsafe {
      _mm256_round_pd(a.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_NEG_INF)
    })
  }};
  ($a:expr, PosInf) => {{
    let a: m256d = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm256_round_pd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_POS_INF,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm256_round_pd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_POS_INF,
    };
    m256d(unsafe {
      _mm256_round_pd(a.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_POS_INF)
    })
  }};
  ($a:expr, Zero) => {{
    let a: m256d = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm256_round_pd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_ZERO,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm256_round_pd, _MM_FROUND_NO_EXC, _MM_FROUND_TO_ZERO,
    };
    m256d(unsafe {
      _mm256_round_pd(a.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_ZERO)
    })
  }};
}

/// Rounds each lane in the style specified.
///
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([-0.1, 1.6, 3.3, 4.5, 5.1, 6.5, 7.2, 8.0]);
/// //
/// assert_eq!(
///   round_m256!(a, Nearest).to_array(),
///   [0.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
/// );
/// //
/// assert_eq!(
///   round_m256!(a, NegInf).to_array(),
///   [-1.0, 1.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
/// );
/// //
/// assert_eq!(
///   round_m256!(a, PosInf).to_array(),
///   [0.0, 2.0, 4.0, 5.0, 6.0, 7.0, 8.0, 8.0]
/// );
/// //
/// assert_eq!(
///   round_m256!(a, Zero).to_array(),
///   [0.0, 1.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
/// );
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! round_m256 {
  ($a:expr, Nearest) => {{
    let a: m256 = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm256_round_ps, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEAREST_INT,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm256_round_ps, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEAREST_INT,
    };
    m256(unsafe {
      _mm256_round_ps(a.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_NEAREST_INT)
    })
  }};
  ($a:expr, NegInf) => {{
    let a: m256 = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm256_round_ps, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEG_INF,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm256_round_ps, _MM_FROUND_NO_EXC, _MM_FROUND_TO_NEG_INF,
    };
    m256(unsafe {
      _mm256_round_ps(a.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_NEG_INF)
    })
  }};
  ($a:expr, PosInf) => {{
    let a: m256 = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm256_round_ps, _MM_FROUND_NO_EXC, _MM_FROUND_TO_POS_INF,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm256_round_ps, _MM_FROUND_NO_EXC, _MM_FROUND_TO_POS_INF,
    };
    m256(unsafe {
      _mm256_round_ps(a.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_POS_INF)
    })
  }};
  ($a:expr, Zero) => {{
    let a: m256 = $a;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::{
      _mm256_round_ps, _MM_FROUND_NO_EXC, _MM_FROUND_TO_ZERO,
    };
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::{
      _mm256_round_ps, _MM_FROUND_NO_EXC, _MM_FROUND_TO_ZERO,
    };
    m256(unsafe {
      _mm256_round_ps(a.0, _MM_FROUND_NO_EXC | _MM_FROUND_TO_ZERO)
    })
  }};
}

/// Reciprocal of `f32` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([16.0, 9.0, 4.0, 25.0, 16.0, 9.0, 4.0, 25.0]);
/// let b = reciprocal_sqrt_m256(a).to_array();
/// let expected = [0.25, 0.33333, 0.5, 0.2, 0.25, 0.33333, 0.5, 0.2];
/// for i in 0..8 {
///   assert!((b[i] - expected[i]).abs() < 0.001);
/// }
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn reciprocal_sqrt_m256(a: m256) -> m256 {
  m256(unsafe { _mm256_rsqrt_ps(a.0) })
}

/// Set `i8` args into an `m256i` lane.
/// ```
/// # use safe_arch::*;
/// let a: [i8; 32] =
///   set_i8_m256i(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31).into();
/// assert_eq!(a, [31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_i8_m256i(
  e31: i8, e30: i8, e29: i8, e28: i8, e27: i8, e26: i8, e25: i8, e24: i8, e23: i8, e22: i8, e21: i8, e20: i8, e19: i8, e18: i8, e17: i8, e16: i8, e15: i8, e14: i8, e13: i8, e12: i8, e11: i8, e10: i8, e9: i8, e8: i8, e7: i8, e6: i8, e5: i8, e4: i8, e3: i8, e2: i8, e1: i8, e0: i8
) -> m256i {
  m256i(unsafe {
    _mm256_set_epi8(
      e31, e30, e29, e28, e27, e26, e25, e24, e23, e22, e21, e20, e19, e18, e17, e16, e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0
    )
  })
}

/// Set `i16` args into an `m256i` lane.
/// ```
/// # use safe_arch::*;
/// let a: [i16; 16] =
///   set_i16_m256i(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15).into();
/// assert_eq!(a, [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_i16_m256i(
  e15: i16, e14: i16, e13: i16, e12: i16, e11: i16, e10: i16, e9: i16, e8: i16,
  e7: i16, e6: i16, e5: i16, e4: i16, e3: i16, e2: i16, e1: i16, e0: i16,
) -> m256i {
  m256i(unsafe {
    _mm256_set_epi16(
      e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0,
    )
  })
}

/// Set `i32` args into an `m256i` lane.
/// ```
/// # use safe_arch::*;
/// let a: [i32; 8] =
///   set_i32_m256i(0, 1, 2, 3, 4, 5, 6, 7).into();
/// assert_eq!(a, [7, 6, 5, 4, 3, 2, 1, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_i32_m256i(
  e7: i32, e6: i32, e5: i32, e4: i32, e3: i32, e2: i32, e1: i32, e0: i32,
) -> m256i {
  m256i(unsafe {
    _mm256_set_epi32(e7, e6, e5, e4, e3, e2, e1, e0)
  })
}

/// Set `i64` args into an `m256i` lane.
/// ```
/// # use safe_arch::*;
/// let a: [i64; 4] = set_i64_m256i(0, 1, 2, 3).into();
/// assert_eq!(a, [3, 2, 1, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch="x86_86")]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_i64_m256i(
  e3: i64, e2: i64, e1: i64, e0: i64,
) -> m256i {
  m256i(unsafe { _mm256_set_epi64x(e3, e2, e1, e0) })
}

/// Set `m128` args into an `m256`.
/// ```
/// # use safe_arch::*;
/// let a = set_m128_m256(
///   m128::from([7.0, 6.0, 5.0, 4.0]),
///   m128::from([3.0, 2.0, 1.0, 0.0]),
/// ).to_array();
/// assert_eq!(a, [7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch="x86_86")]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_m128_m256(
  hi: m128, lo: m128
) -> m256 {
  m256(unsafe { _mm256_set_m128(hi.0, lo.0) })
}

/// Set `m128d` args into an `m256d`.
/// ```
/// # use safe_arch::*;
/// let a = set_m128d_m256d(
///   set_m128d(3.0, 2.0),
///   set_m128d(1.0, 0.0),
/// ).to_array();
/// assert_eq!(a, [0.0, 1.0, 2.0, 3.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_m128d_m256d(
  hi: m128d, lo: m128d
) -> m256d {
  m256d(unsafe { _mm256_set_m128d(hi.0, lo.0) })
}

/// Set `m128i` args into an `m256i`.
/// ```
/// # use safe_arch::*;
/// let a: [i64; 4] = set_m128i_m256i(
///   set_i64_m128i(3_i64, 2),
///   set_i64_m128i(1_i64, 0),
/// ).into();
/// assert_eq!(a, [0_i64, 1, 2, 3]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_m128i_m256i(
  hi: m128i, lo: m128i
) -> m256i {
  m256i(unsafe { _mm256_set_m128i(hi.0, lo.0) })
}

/// Set `f64` args into an `m256d` lane.
/// ```
/// # use safe_arch::*;
/// let a = set_m256d(0.0, 1.0, 2.0, 3.0).to_array();
/// assert_eq!(a, [3.0, 2.0, 1.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_m256d(
  e3: f64, e2: f64, e1: f64, e0: f64,
) -> m256d {
  m256d(unsafe { _mm256_set_pd(e3, e2, e1, e0) })
}

/// Set `f32` args into an `m256` lane.
/// ```
/// # use safe_arch::*;
/// let a =
///   set_m256(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0).to_array();
/// assert_eq!(a, [7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_m256(
  e7: f32, e6: f32, e5: f32, e4: f32, e3: f32, e2: f32, e1: f32, e0: f32,
) -> m256 {
  m256(unsafe {
    _mm256_set_ps(e7, e6, e5, e4, e3, e2, e1, e0)
  })
}

/// Splat an `i8` arg into an `m256i` lane.
/// ```
/// # use safe_arch::*;
/// let a: [i8; 32] = set_splat_i8_m256i(56).into();
/// assert_eq!(a, [56_i8; 32]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn set_splat_i8_m256i(i: i8) -> m256i {
  m256i(unsafe { _mm256_set1_epi8(i) })
}

/// Splat an `i16` arg into an `m256i` lane.
/// ```
/// # use safe_arch::*;
/// let a: [i16; 16] = set_splat_i16_m256i(56).into();
/// assert_eq!(a, [56_i16; 16]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn set_splat_i16_m256i(i: i16) -> m256i {
  m256i(unsafe { _mm256_set1_epi16(i) })
}

/// Splat an `i32` arg into an `m256i` lane.
/// ```
/// # use safe_arch::*;
/// let a: [i32; 8] = set_splat_i32_m256i(56).into();
/// assert_eq!(a, [56_i32; 8]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn set_splat_i32_m256i(i: i32) -> m256i {
  m256i(unsafe { _mm256_set1_epi32(i) })
}

/// Splat an `i64` arg into an `m256i` lane.
/// ```
/// # use safe_arch::*;
/// let a: [i64; 32] = set_splat_i64_m256i(56).into();
/// assert_eq!(a, [56_i64; 4]);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_86")]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn set_splat_i64_m256i(i: i64) -> m256i {
  m256i(unsafe { _mm256_set1_epi64x(i) })
}

/// Splat an `f64` arg into an `m256d` lane.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m256d(56.0).to_array();
/// assert_eq!(a, [56.0; 4]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn set_splat_m256d(f: f64) -> m256d {
  m256d(unsafe { _mm256_set1_pd(f) })
}

/// Splat an `f32` arg into an `m256` lane.
/// ```
/// # use safe_arch::*;
/// let a =
///   set_splat_m256(56.0).to_array();
/// assert_eq!(a, [56.0; 8]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_splat_m256(
  f: f32,
) -> m256 {
  m256(unsafe {
    _mm256_set1_ps(f)
  })
}

//
//
//

/// Set `i8` args into an `m256i` lane.
/// ```
/// # use safe_arch::*;
/// let a: [i8; 32] =
///   set_reversed_i8_m256i(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31).into();
/// assert_eq!(a, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_reversed_i8_m256i(
  e31: i8, e30: i8, e29: i8, e28: i8, e27: i8, e26: i8, e25: i8, e24: i8, e23: i8, e22: i8, e21: i8, e20: i8, e19: i8, e18: i8, e17: i8, e16: i8, e15: i8, e14: i8, e13: i8, e12: i8, e11: i8, e10: i8, e9: i8, e8: i8, e7: i8, e6: i8, e5: i8, e4: i8, e3: i8, e2: i8, e1: i8, e0: i8
) -> m256i {
  m256i(unsafe {
    _mm256_setr_epi8(
      e31, e30, e29, e28, e27, e26, e25, e24, e23, e22, e21, e20, e19, e18, e17, e16, e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0
    )
  })
}

/// Set `i16` args into an `m256i` lane.
/// ```
/// # use safe_arch::*;
/// let a: [i16; 16] =
///   set_reversed_i16_m256i(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15).into();
/// assert_eq!(a, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_reversed_i16_m256i(
  e15: i16, e14: i16, e13: i16, e12: i16, e11: i16, e10: i16, e9: i16, e8: i16,
  e7: i16, e6: i16, e5: i16, e4: i16, e3: i16, e2: i16, e1: i16, e0: i16,
) -> m256i {
  m256i(unsafe {
    _mm256_setr_epi16(
      e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0,
    )
  })
}

/// Set `i32` args into an `m256i` lane.
/// ```
/// # use safe_arch::*;
/// let a: [i32; 8] =
///   set_reversed_i32_m256i(0, 1, 2, 3, 4, 5, 6, 7).into();
/// assert_eq!(a, [0, 1, 2, 3, 4, 5, 6, 7]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_reversed_i32_m256i(
  e7: i32, e6: i32, e5: i32, e4: i32, e3: i32, e2: i32, e1: i32, e0: i32,
) -> m256i {
  m256i(unsafe {
    _mm256_setr_epi32(e7, e6, e5, e4, e3, e2, e1, e0)
  })
}

/// Set `i64` args into an `m256i` lane.
/// ```
/// # use safe_arch::*;
/// let a: [i64; 4] = set_reversed_i64_m256i(0, 1, 2, 3).into();
/// assert_eq!(a, [0, 1, 2, 3]);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch="x86_86")]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_reversed_i64_m256i(
  e3: i64, e2: i64, e1: i64, e0: i64,
) -> m256i {
  m256i(unsafe { _mm256_setr_epi64x(e3, e2, e1, e0) })
}

/// Set `m128` args into an `m256`.
/// ```
/// # use safe_arch::*;
/// let a = set_reversed_m128_m256(
///   set_reversed_m128(7.0, 6.0, 5.0, 4.0),
///   set_reversed_m128(3.0, 2.0, 1.0, 0.0),
/// ).to_array();
/// assert_eq!(a, [7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch="x86_86")]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_reversed_m128_m256(
  hi: m128, lo: m128
) -> m256 {
  m256(unsafe { _mm256_setr_m128(hi.0, lo.0) })
}

/// Set `m128d` args into an `m256d`.
/// ```
/// # use safe_arch::*;
/// let a = set_reversed_m128d_m256d(
///   set_reversed_m128d(3.0, 2.0),
///   set_reversed_m128d(1.0, 0.0),
/// ).to_array();
/// assert_eq!(a, [3.0, 2.0, 1.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_reversed_m128d_m256d(
  hi: m128d, lo: m128d
) -> m256d {
  m256d(unsafe { _mm256_setr_m128d(hi.0, lo.0) })
}

/// Set `m128i` args into an `m256i`.
/// ```
/// # use safe_arch::*;
/// let a: [i64; 4] = set_reversed_m128i_m256i(
///   m128i::from([0_i64, 1]),
///   m128i::from([2_i64, 3]),
/// ).into();
/// assert_eq!(a, [0_i64, 1, 2, 3]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_reversed_m128i_m256i(
  hi: m128i, lo: m128i
) -> m256i {
  m256i(unsafe { _mm256_setr_m128i(hi.0, lo.0) })
}

/// Set `f64` args into an `m256d` lane.
/// ```
/// # use safe_arch::*;
/// let a = set_reversed_m256d(0.0, 1.0, 2.0, 3.0).to_array();
/// assert_eq!(a, [0.0, 1.0, 2.0, 3.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_reversed_m256d(
  e3: f64, e2: f64, e1: f64, e0: f64,
) -> m256d {
  m256d(unsafe { _mm256_setr_pd(e3, e2, e1, e0) })
}

/// Set `f32` args into an `m256` lane.
/// ```
/// # use safe_arch::*;
/// let a =
///   set_reversed_m256(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0).to_array();
/// assert_eq!(a, [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
#[rustfmt::skip]
pub fn set_reversed_m256(
  e7: f32, e6: f32, e5: f32, e4: f32, e3: f32, e2: f32, e1: f32, e0: f32,
) -> m256 {
  m256(unsafe {
    _mm256_setr_ps(e7, e6, e5, e4, e3, e2, e1, e0)
  })
}

/// A zeroed `m256d`
/// ```
/// # use safe_arch::*;
/// let a = zeroed_m256d().to_array();
/// assert_eq!(a, [0.0; 4]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn zeroed_m256d() -> m256d {
  m256d(unsafe { _mm256_setzero_pd() })
}

/// A zeroed `m256`
/// ```
/// # use safe_arch::*;
/// let a = zeroed_m256().to_array();
/// assert_eq!(a, [0.0; 8]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn zeroed_m256() -> m256 {
  m256(unsafe { _mm256_setzero_ps() })
}

/// A zeroed `m256i`
/// ```
/// # use safe_arch::*;
/// let a: [i32; 8] = zeroed_m256i().into();
/// assert_eq!(a, [0; 8]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn zeroed_m256i() -> m256i {
  m256i(unsafe { _mm256_setzero_si256() })
}

/// Shuffles the `f64` lanes around.
///
/// * args are 0 or 1 each, for "low" or "high" within that pairing.
/// * a 0/1, b 0/1, a 2/3, b 2/3
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m256d::from_array([5.0, 6.0, 7.0, 8.0]);
/// //
/// let c = shuffle_m256d!(a, b, 1, 0, 1, 0).to_array();
/// assert_eq!(c, [2.0, 5.0, 4.0, 7.0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! shuffle_m256d {
  ($a:expr, $b:expr, $z:expr, $o:expr, $t:expr, $e:expr) => {{
    const MASK: i32 =
      (($z & 0b1) | ($o & 0b1) << 1 | ($t & 0b1) << 2 | ($e & 0b1) << 3) as i32;
    let a: m256d = $a;
    let b: m256d = $b;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm256_shuffle_pd;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_shuffle_pd;
    m256d(unsafe { _mm256_shuffle_pd(a.0, b.0, MASK) })
  }};
}

/// Shuffles the `f32` lanes around.
///
/// * args are 0, 1, 2, 3 for which lane to use in the lower or upper half.
/// * the same pattern is used for the four low lanes and the four high lanes.
/// * a low, a low, b low, b low, a high, a high, b high, b high
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
/// let b = m256::from_array([9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
/// //
/// let c = shuffle_m256!(a, b, 1, 3, 2, 0).to_array();
/// assert_eq!(c, [2.0, 4.0, 11.0, 9.0, 6.0, 8.0, 15.0, 13.0]);
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
macro_rules! shuffle_m256 {
  ($a:expr, $b:expr, $z:expr, $o:expr, $t:expr, $e:expr) => {{
    const MASK: i32 =
      (($z & 0b11) | ($o & 0b11) << 2 | ($t & 0b11) << 4 | ($e & 0b11) << 6)
        as i32;
    let a: m256 = $a;
    let b: m256 = $b;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm256_shuffle_ps;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_shuffle_ps;
    m256(unsafe { _mm256_shuffle_ps(a.0, b.0, MASK) })
  }};
}

/// Lanewise `sqrt` on `f64` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 4.0, 9.0, 16.0]);
/// let b = sqrt_m256d(a).to_array();
/// assert_eq!(b, [1.0, 2.0, 3.0, 4.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn sqrt_m256d(a: m256d) -> m256d {
  m256d(unsafe { _mm256_sqrt_pd(a.0) })
}

/// Lanewise `sqrt` on `f64` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 4.0, 9.0, 16.0, 25.0, 36.0, 0.0, 49.0]);
/// let b = sqrt_m256(a).to_array();
/// assert_eq!(b, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 0.0, 7.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn sqrt_m256(a: m256) -> m256 {
  m256(unsafe { _mm256_sqrt_ps(a.0) })
}

/// Store data from a register into memory.
///
/// ```
/// # use safe_arch::*;
/// let mut addr = m256d::from([0.0; 4]);
/// store_m256d(&mut addr, m256d::from([1.0, 2.0, 3.0, 4.0]));
/// assert_eq!(addr.to_array(), [1.0, 2.0, 3.0, 4.0]);
/// ```
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn store_m256d(addr: &mut m256d, a: m256d) {
  unsafe { _mm256_store_pd(addr as *mut m256d as *mut f64, a.0) }
}

/// Store data from a register into memory.
///
/// ```
/// # use safe_arch::*;
/// let mut addr = m256::from([0.0; 8]);
/// store_m256(&mut addr, m256::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]));
/// assert_eq!(addr.to_array(), [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
/// ```
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn store_m256(addr: &mut m256, a: m256) {
  unsafe { _mm256_store_ps(addr as *mut m256 as *mut f32, a.0) }
}

/// Store data from a register into memory.
///
/// ```
/// # use safe_arch::*;
/// let mut addr = m256i::from([0_i32; 8]);
/// store_m256i(&mut addr, m256i::from([1, 2, 3, 4, 5, 6, 7, 8]));
/// assert_eq!(<[i32; 8]>::from(addr), [1, 2, 3, 4, 5, 6, 7, 8]);
/// ```
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn store_m256i(addr: &mut m256i, a: m256i) {
  unsafe { _mm256_store_si256(addr as *mut m256i as *mut __m256i, a.0) }
}

/// Store data from a register into memory.
///
/// ```
/// # use safe_arch::*;
/// let mut addr = [0.0; 4];
/// store_unaligned_m256d(&mut addr, m256d::from([1.0, 2.0, 3.0, 4.0]));
/// assert_eq!(addr, [1.0, 2.0, 3.0, 4.0]);
/// ```
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn store_unaligned_m256d(addr: &mut [f64; 4], a: m256d) {
  unsafe { _mm256_storeu_pd(addr.as_mut_ptr(), a.0) }
}

/// Store data from a register into memory.
///
/// ```
/// # use safe_arch::*;
/// let mut addr = [0.0; 8];
/// store_unaligned_m256(
///   &mut addr,
///   m256::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
/// );
/// assert_eq!(addr, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
/// ```
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn store_unaligned_m256(addr: &mut [f32; 8], a: m256) {
  unsafe { _mm256_storeu_ps(addr.as_mut_ptr(), a.0) }
}

/// Store data from a register into memory.
///
/// ```
/// # use safe_arch::*;
/// let mut addr = [0_i8; 32];
/// store_unaligned_m256i(&mut addr, m256i::from([12_i8; 32]));
/// assert_eq!(addr, [12_i8; 32]);
/// ```
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn store_unaligned_m256i(addr: &mut [i8; 32], a: m256i) {
  unsafe { _mm256_storeu_si256(addr as *mut [i8; 32] as *mut __m256i, a.0) }
}

/// Store data from a register into memory.
///
/// ```
/// # use safe_arch::*;
/// let mut hi_addr = [0.0; 2];
/// let mut lo_addr = [0.0; 2];
/// store_unaligned_hi_lo_m256d(
///   &mut hi_addr,
///   &mut lo_addr,
///   m256d::from([1.0, 2.0, 3.0, 4.0]),
/// );
/// assert_eq!(hi_addr, [3.0, 4.0]);
/// assert_eq!(lo_addr, [1.0, 2.0]);
/// ```
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn store_unaligned_hi_lo_m256d(
  hi_addr: &mut [f64; 2],
  lo_addr: &mut [f64; 2],
  a: m256d,
) {
  unsafe {
    _mm256_storeu2_m128d(hi_addr.as_mut_ptr(), lo_addr.as_mut_ptr(), a.0)
  }
}

/// Store data from a register into memory.
///
/// ```
/// # use safe_arch::*;
/// let mut hi_addr = [0.0; 4];
/// let mut lo_addr = [0.0; 4];
/// store_unaligned_hi_lo_m256(
///   &mut hi_addr,
///   &mut lo_addr,
///   m256::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
/// );
/// assert_eq!(hi_addr, [5.0, 6.0, 7.0, 8.0]);
/// assert_eq!(lo_addr, [1.0, 2.0, 3.0, 4.0]);
/// ```
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn store_unaligned_hi_lo_m256(
  hi_addr: &mut [f32; 4],
  lo_addr: &mut [f32; 4],
  a: m256,
) {
  unsafe {
    _mm256_storeu2_m128(hi_addr.as_mut_ptr(), lo_addr.as_mut_ptr(), a.0)
  }
}

/// Store data from a register into memory.
///
/// ```
/// # use safe_arch::*;
/// let mut hi_addr = [0_i8; 16];
/// let mut lo_addr = [0_i8; 16];
/// store_unaligned_hi_lo_m256i(
///   &mut hi_addr,
///   &mut lo_addr,
///   m256i::from([56_i8; 32]),
/// );
/// assert_eq!(hi_addr, [56_i8; 16]);
/// assert_eq!(lo_addr, [56_i8; 16]);
/// ```
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn store_unaligned_hi_lo_m256i(
  hi_addr: &mut [i8; 16],
  lo_addr: &mut [i8; 16],
  a: m256i,
) {
  unsafe {
    _mm256_storeu2_m128i(
      hi_addr.as_mut_ptr().cast(),
      lo_addr.as_mut_ptr().cast(),
      a.0,
    )
  }
}

/// Lanewise `a - b` with `f64` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m256d::from_array([5.0, 60.0, 712.0, 8.5]);
/// let c = sub_m256d(a, b).to_array();
/// assert_eq!(c, [-4.0, -58.0, -709.0, -4.5]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn sub_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_sub_pd(a.0, b.0) })
}

/// Lanewise `a - b` with `f32` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 20.0, 30.0, 40.0, 50.0]);
/// let b = m256::from_array([59.0, 61.0, 79.0, 81.5, 90.0, 100.0, 110.0, 51.0]);
/// let c = sub_m256(a, b).to_array();
/// assert_eq!(c, [-58.0, -59.0, -76.0, -77.5, -70.0, -70.0, -70.0, -1.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn sub_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_sub_ps(a.0, b.0) })
}

/// Unpack and interleave the high lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m256d::from_array([59.0, 61.0, 79.0, 81.5]);
/// let c = unpack_hi_m256d(a, b).to_array();
/// assert_eq!(c, [2.0, 61.0, 4.0, 81.5]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn unpack_hi_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_unpackhi_pd(a.0, b.0) })
}

/// Unpack and interleave the high lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 20.0, 30.0, 40.0, 50.0]);
/// let b = m256::from_array([59.0, 61.0, 79.0, 81.5, 90.0, 100.0, 110.0, 51.0]);
/// let c = unpack_hi_m256(a, b).to_array();
/// assert_eq!(c, [3.0, 79.0, 4.0, 81.5, 40.0, 110.0, 50.0, 51.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn unpack_hi_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_unpackhi_ps(a.0, b.0) })
}

/// Unpack and interleave the high lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m256d::from_array([59.0, 61.0, 79.0, 81.5]);
/// let c = unpack_lo_m256d(a, b).to_array();
/// assert_eq!(c, [1.0, 59.0, 3.0, 79.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn unpack_lo_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_unpacklo_pd(a.0, b.0) })
}

/// Unpack and interleave the high lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 2.0, 3.0, 4.0, 20.0, 30.0, 40.0, 50.0]);
/// let b = m256::from_array([59.0, 61.0, 79.0, 81.5, 90.0, 100.0, 110.0, 51.0]);
/// let c = unpack_lo_m256(a, b).to_array();
/// assert_eq!(c, [1.0, 59.0, 2.0, 61.0, 20.0, 90.0, 30.0, 100.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn unpack_lo_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_unpacklo_ps(a.0, b.0) })
}

/// Bitwise `a ^ b`.
/// ```
/// # use safe_arch::*;
/// let a = m256d::from_array([1.0, 0.0, 1.0, 0.0]);
/// let b = m256d::from_array([1.0, 1.0, 0.0, 0.0]);
/// let c = bitxor_m256d(a, b).to_array();
/// assert_eq!(c, [0.0, 1.0, 1.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn bitxor_m256d(a: m256d, b: m256d) -> m256d {
  m256d(unsafe { _mm256_xor_pd(a.0, b.0) })
}

/// Bitwise `a ^ b`.
/// ```
/// # use safe_arch::*;
/// let a = m256::from_array([1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
/// let b = m256::from_array([1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0]);
/// let c = bitxor_m256(a, b).to_array();
/// assert_eq!(c, [0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn bitxor_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_xor_ps(a.0, b.0) })
}

/// Zero extend an `m128d` to `m256d`
///
/// ```
/// # use safe_arch::*;
/// let a = zero_extend_m128d(m128d::from_array([1.0, 2.0])).to_array();
/// assert_eq!(a, [1.0, 2.0, 0.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn zero_extend_m128d(a: m128d) -> m256d {
  m256d(unsafe { _mm256_zextpd128_pd256(a.0) })
}

/// Zero extend an `m128` to `m256`
///
/// ```
/// # use safe_arch::*;
/// let a = zero_extend_m128(m128::from_array([1.0, 2.0, 3.0, 4.0])).to_array();
/// assert_eq!(a, [1.0, 2.0, 3.0, 4.0, 0.0, 0.0, 0.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn zero_extend_m128(a: m128) -> m256 {
  m256(unsafe { _mm256_zextps128_ps256(a.0) })
}

/// Zero extend an `m128i` to `m256i`
///
/// ```
/// # use safe_arch::*;
/// let a: [i32; 8] = zero_extend_m128i(m128i::from([1, 2, 3, 4])).into();
/// assert_eq!(a, [1, 2, 3, 4, 0, 0, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx")))]
pub fn zero_extend_m128i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_zextsi128_si256(a.0) })
}

impl Add for m256d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Self) -> Self {
    add_m256d(self, rhs)
  }
}
impl AddAssign for m256d {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl BitAnd for m256d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitand(self, rhs: Self) -> Self {
    bitand_m256d(self, rhs)
  }
}
impl BitAndAssign for m256d {
  #[inline(always)]
  fn bitand_assign(&mut self, rhs: Self) {
    *self = *self & rhs;
  }
}

impl BitOr for m256d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitor(self, rhs: Self) -> Self {
    bitor_m256d(self, rhs)
  }
}
impl BitOrAssign for m256d {
  #[inline(always)]
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}

impl BitXor for m256d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitxor(self, rhs: Self) -> Self {
    bitxor_m256d(self, rhs)
  }
}
impl BitXorAssign for m256d {
  #[inline(always)]
  fn bitxor_assign(&mut self, rhs: Self) {
    *self = *self ^ rhs;
  }
}

impl Div for m256d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn div(self, rhs: Self) -> Self {
    div_m256d(self, rhs)
  }
}
impl DivAssign for m256d {
  #[inline(always)]
  fn div_assign(&mut self, rhs: Self) {
    *self = *self / rhs;
  }
}

impl Mul for m256d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: Self) -> Self {
    mul_m256d(self, rhs)
  }
}
impl MulAssign for m256d {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl Neg for m256d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn neg(self) -> Self {
    sub_m256d(zeroed_m256d(), self)
  }
}

impl Not for m256d {
  type Output = Self;
  /// Not a direct intrinsic, but it's very useful and the implementation is
  /// simple enough.
  ///
  /// Negates the bits by performing an `xor` with an all-1s bit pattern.
  #[must_use]
  #[inline(always)]
  fn not(self) -> Self {
    let all_bits = set_splat_m256d(f64::from_bits(u64::MAX));
    self ^ all_bits
  }
}

impl Sub for m256d {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Self) -> Self {
    sub_m256d(self, rhs)
  }
}
impl SubAssign for m256d {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl PartialEq for m256d {
  /// ```
  /// # use safe_arch::*;
  /// let a = m256d::from([1.0, 2.0, 3.0, 4.0]);
  /// let b = m256d::from([5.0, 6.0, 7.0, 8.0]);
  /// assert!(a == a);
  /// assert!(a != b);
  /// ```
  #[must_use]
  #[inline(always)]
  fn eq(&self, other: &Self) -> bool {
    let mask = m256d(unsafe { _mm256_cmp_pd(self.0, other.0, _CMP_EQ_OQ) });
    move_mask_m256d(mask) == 0b1111
  }
}

impl Add for m256 {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn add(self, rhs: Self) -> Self {
    add_m256(self, rhs)
  }
}
impl AddAssign for m256 {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl BitAnd for m256 {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitand(self, rhs: Self) -> Self {
    bitand_m256(self, rhs)
  }
}
impl BitAndAssign for m256 {
  #[inline(always)]
  fn bitand_assign(&mut self, rhs: Self) {
    *self = *self & rhs;
  }
}

impl BitOr for m256 {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitor(self, rhs: Self) -> Self {
    bitor_m256(self, rhs)
  }
}
impl BitOrAssign for m256 {
  #[inline(always)]
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}

impl BitXor for m256 {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitxor(self, rhs: Self) -> Self {
    bitxor_m256(self, rhs)
  }
}
impl BitXorAssign for m256 {
  #[inline(always)]
  fn bitxor_assign(&mut self, rhs: Self) {
    *self = *self ^ rhs;
  }
}

impl Div for m256 {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn div(self, rhs: Self) -> Self {
    div_m256(self, rhs)
  }
}
impl DivAssign for m256 {
  #[inline(always)]
  fn div_assign(&mut self, rhs: Self) {
    *self = *self / rhs;
  }
}

impl Mul for m256 {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn mul(self, rhs: Self) -> Self {
    mul_m256(self, rhs)
  }
}
impl MulAssign for m256 {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl Neg for m256 {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn neg(self) -> Self {
    sub_m256(zeroed_m256(), self)
  }
}

impl Not for m256 {
  type Output = Self;
  /// Not a direct intrinsic, but it's very useful and the implementation is
  /// simple enough.
  ///
  /// Negates the bits by performing an `xor` with an all-1s bit pattern.
  #[must_use]
  #[inline(always)]
  fn not(self) -> Self {
    let all_bits = set_splat_m256(f32::from_bits(u32::MAX));
    self ^ all_bits
  }
}

impl Sub for m256 {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn sub(self, rhs: Self) -> Self {
    sub_m256(self, rhs)
  }
}
impl SubAssign for m256 {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl PartialEq for m256 {
  /// ```
  /// # use safe_arch::*;
  /// let a = m256::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  /// let b = m256::from([9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
  /// assert!(a == a);
  /// assert!(a != b);
  /// ```
  #[must_use]
  #[inline(always)]
  fn eq(&self, other: &Self) -> bool {
    let mask = m256(unsafe { _mm256_cmp_ps(self.0, other.0, _CMP_EQ_OQ) });
    move_mask_m256(mask) == 0b1111_1111
  }
}
