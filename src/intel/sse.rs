#![cfg(target_feature = "sse")]

use super::*;

// TODO: load and store m128i values. The operations are technically part of
// `sse`, but all actual usage of `m128i` is from `sse2`, so we won't tackle
// that right away.

/// Lanewise `a + b`.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([5.0, 6.0, 7.0, 8.5]);
/// let c = add_m128(a, b).to_array();
/// assert_eq!(c, [6.0, 8.0, 10.0, 12.5]);
/// ```
#[must_use]
#[inline(always)]
pub fn add_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_add_ps(a.0, b.0) })
}

/// Low lane `a + b`, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([5.0, 6.0, 7.0, 8.5]);
/// let c = add_m128_s(a, b).to_array();
/// assert_eq!(c, [6.0, 2.0, 3.0, 4.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn add_m128_s(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_add_ss(a.0, b.0) })
}

/// Bitwise `a & b`.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 0.0, 1.0, 0.0]);
/// let b = m128::from_array([1.0, 1.0, 0.0, 0.0]);
/// let c = and_m128(a, b).to_array();
/// assert_eq!(c, [1.0, 0.0, 0.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn and_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_and_ps(a.0, b.0) })
}

/// Bitwise `(!a) & b`.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 0.0, 1.0, 0.0]);
/// let b = m128::from_array([1.0, 1.0, 0.0, 0.0]);
/// let c = andnot_m128(a, b).to_array();
/// assert_eq!(c, [0.0, 1.0, 0.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn andnot_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_andnot_ps(a.0, b.0) })
}

/// Lanewise `a == b`.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 0.0, 1.0, 0.0]);
/// let b = m128::from_array([1.0, 1.0, 0.0, 0.0]);
/// let c = cmp_eq_m128_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [u32::MAX, 0, 0, u32::MAX]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_eq_m128_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpeq_ps(a.0, b.0) })
}

/// Low lane `a == b`, other lanes unchanged.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 0.0, 1.0, 0.0]);
/// let b = m128::from_array([1.0, 1.0, 0.0, 0.0]);
/// let c = cmp_eq_m128_s_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [u32::MAX, 0, 1_f32.to_bits(), 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_eq_m128_s_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpeq_ss(a.0, b.0) })
}

/// Lanewise `a >= b`.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([2.0, 2.0, 2.0, 2.0]);
/// let c = cmp_ge_m128_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [0, u32::MAX, u32::MAX, u32::MAX]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_ge_m128_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpge_ps(a.0, b.0) })
}

/// Low lane `a >= b`, other lanes unchanged.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([2.0, 2.0, 2.0, 2.0]);
/// let c = cmp_ge_m128_s_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [u32::MAX, 2_f32.to_bits(), 3_f32.to_bits(), 4_f32.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_ge_m128_s_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpge_ss(a.0, b.0) })
}

/// Lanewise `a > b`.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([2.0, 2.0, 2.0, 2.0]);
/// let c = cmp_gt_m128_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [0, 0, u32::MAX, u32::MAX]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_gt_m128_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpgt_ps(a.0, b.0) })
}

/// Low lane `a > b`, other lanes unchanged.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.5, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([2.0, 2.0, 2.0, 2.0]);
/// let c = cmp_gt_m128_s_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [u32::MAX, 2_f32.to_bits(), 3_f32.to_bits(), 4_f32.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_gt_m128_s_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpgt_ss(a.0, b.0) })
}

/// Lanewise `a <= b`.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([2.0, 2.0, 2.0, 2.0]);
/// let c = cmp_le_m128_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [u32::MAX, u32::MAX, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_le_m128_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmple_ps(a.0, b.0) })
}

/// Low lane `a <= b`, other lanes unchanged.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([2.0, 2.0, 2.0, 2.0]);
/// let c = cmp_le_m128_s_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [u32::MAX, 2_f32.to_bits(), 3_f32.to_bits(), 4_f32.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_le_m128_s_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmple_ss(a.0, b.0) })
}

/// Lanewise `a < b`.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([2.0, 2.0, 2.0, 2.0]);
/// let c = cmp_lt_m128_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [u32::MAX, 0, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_lt_m128_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmplt_ps(a.0, b.0) })
}

/// Low lane `a < b`, other lanes unchanged.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([2.0, 2.0, 2.0, 2.0]);
/// let c = cmp_lt_m128_s_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [u32::MAX, 2_f32.to_bits(), 3_f32.to_bits(), 4_f32.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_lt_m128_s_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmplt_ss(a.0, b.0) })
}

/// Lanewise `a != b`.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 0.0, 1.0, 0.0]);
/// let b = m128::from_array([1.0, 1.0, 0.0, 0.0]);
/// let c = cmp_neq_m128_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [0, u32::MAX, u32::MAX, 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_neq_m128_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpneq_ps(a.0, b.0) })
}

/// Low lane `a != b`, other lanes unchanged.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 0.0, 1.0, 0.0]);
/// let b = m128::from_array([1.0, 1.0, 0.0, 0.0]);
/// let c = cmp_neq_m128_s_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [0, 0, 1_f32.to_bits(), 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_neq_m128_s_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpneq_ss(a.0, b.0) })
}

/// Lanewise `!(a >= b)`.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([2.0, 2.0, 2.0, 2.0]);
/// let c = cmp_nge_m128_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [u32::MAX, 0, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_nge_m128_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpnge_ps(a.0, b.0) })
}

/// Low lane `!(a >= b)`, other lanes unchanged.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([2.0, 2.0, 2.0, 2.0]);
/// let c = cmp_nge_m128_s_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [0, 2_f32.to_bits(), 3_f32.to_bits(), 4_f32.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_nge_m128_s_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpnge_ss(a.0, b.0) })
}

/// Lanewise `!(a > b)`.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([2.0, 2.0, 2.0, 2.0]);
/// let c = cmp_ngt_m128_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [u32::MAX, u32::MAX, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_ngt_m128_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpngt_ps(a.0, b.0) })
}

/// Low lane `!(a > b)`, other lanes unchanged.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.5, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([2.0, 2.0, 2.0, 2.0]);
/// let c = cmp_ngt_m128_s_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [0, 2_f32.to_bits(), 3_f32.to_bits(), 4_f32.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_ngt_m128_s_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpngt_ss(a.0, b.0) })
}

/// Lanewise `!(a <= b)`.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([2.0, 2.0, 2.0, 2.0]);
/// let c = cmp_nle_m128_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [0, 0, u32::MAX, u32::MAX]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_nle_m128_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpnle_ps(a.0, b.0) })
}

/// Low lane `!(a <= b)`, other lanes unchanged.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([2.0, 2.0, 2.0, 2.0]);
/// let c = cmp_nle_m128_s_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [0, 2_f32.to_bits(), 3_f32.to_bits(), 4_f32.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_nle_m128_s_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpnle_ss(a.0, b.0) })
}

/// Lanewise `!(a < b)`.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([2.0, 2.0, 2.0, 2.0]);
/// let c = cmp_nlt_m128_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [0, u32::MAX, u32::MAX, u32::MAX]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_nlt_m128_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpnlt_ps(a.0, b.0) })
}

/// Low lane `!(a < b)`, other lanes unchanged.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([2.0, 2.0, 2.0, 2.0]);
/// let c = cmp_nlt_m128_s_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [0, 2_f32.to_bits(), 3_f32.to_bits(), 4_f32.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_nlt_m128_s_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpnlt_ss(a.0, b.0) })
}

/// Lanewise `(!a.is_nan()) & (!b.is_nan())`.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([0.0, f32::NAN, 0.0, f32::NAN]);
/// let b = m128::from_array([0.0, 0.0, f32::NAN, f32::NAN]);
/// let c = cmp_ord_m128_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [u32::MAX, 0, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_ord_m128_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpord_ps(a.0, b.0) })
}

/// Low lane `(!a.is_nan()) & (!b.is_nan())`, other lanes unchanged.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([0.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([0.0, f32::NAN, f32::NAN, f32::NAN]);
/// let c = cmp_ord_m128_s_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [u32::MAX, 2_f32.to_bits(), 3_f32.to_bits(), 4_f32.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_ord_m128_s_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpord_ss(a.0, b.0) })
}

/// Lanewise `a.is_nan() | b.is_nan()`.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([0.0, f32::NAN, 0.0, f32::NAN]);
/// let b = m128::from_array([0.0, 0.0, f32::NAN, f32::NAN]);
/// let c = cmp_unord_m128_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [0, u32::MAX, u32::MAX, u32::MAX]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_unord_m128_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpunord_ps(a.0, b.0) })
}

/// Low lane `a.is_nan() | b.is_nan()`, other lanes unchanged.
///
/// All bits 1 for true, all bit 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([0.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([0.0, f32::NAN, f32::NAN, f32::NAN]);
/// let c = cmp_unord_m128_s_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [0, 2_f32.to_bits(), 3_f32.to_bits(), 4_f32.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_unord_m128_s_mask(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_cmpunord_ss(a.0, b.0) })
}

/// Low lane `i32` equality.
///
/// 1 for true, 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([1.0, 1.0, 1.0, 1.0]);
/// assert_eq!(1_i32, cmp_i32eq_m128_s_mask(a, b));
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_i32eq_m128_s_mask(a: m128, b: m128) -> i32 {
  unsafe { _mm_comieq_ss(a.0, b.0) }
}

/// Low lane `i32` greater than or equal to.
///
/// 1 for true, 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([1.0, 1.0, 1.0, 1.0]);
/// assert_eq!(1_i32, cmp_i32ge_m128_s_mask(a, b));
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_i32ge_m128_s_mask(a: m128, b: m128) -> i32 {
  unsafe { _mm_comige_ss(a.0, b.0) }
}

/// Low lane `i32` greater than.
///
/// 1 for true, 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([2.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([1.0, 1.0, 1.0, 1.0]);
/// assert_eq!(1_i32, cmp_i32gt_m128_s_mask(a, b));
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_i32gt_m128_s_mask(a: m128, b: m128) -> i32 {
  unsafe { _mm_comigt_ss(a.0, b.0) }
}

/// Low lane `i32` less than or equal to.
///
/// 1 for true, 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([0.5, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([1.0, 1.0, 1.0, 1.0]);
/// assert_eq!(1_i32, cmp_i32le_m128_s_mask(a, b));
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_i32le_m128_s_mask(a: m128, b: m128) -> i32 {
  unsafe { _mm_comile_ss(a.0, b.0) }
}

/// Low lane `i32` less than.
///
/// 1 for true, 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([0.5, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([1.0, 1.0, 1.0, 1.0]);
/// assert_eq!(1_i32, cmp_i32lt_m128_s_mask(a, b));
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_i32lt_m128_s_mask(a: m128, b: m128) -> i32 {
  unsafe { _mm_comilt_ss(a.0, b.0) }
}

/// Low lane `i32` not equal to.
///
/// 1 for true, 0 for false.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([1.0, 1.0, 1.0, 1.0]);
/// assert_eq!(0_i32, cmp_i32neq_m128_s_mask(a, b));
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_i32neq_m128_s_mask(a: m128, b: m128) -> i32 {
  unsafe { _mm_comineq_ss(a.0, b.0) }
}

/// Convert `i32` to `f32` and replace the low lane of the input.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = convert_replace_i32_m128_s(a, 5_i32).to_array();
/// assert_eq!(b, [5.0, 2.0, 3.0, 4.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn convert_replace_i32_m128_s(a: m128, i: i32) -> m128 {
  m128(unsafe { _mm_cvtsi32_ss(a.0, i) })
}

/// Convert `i64` to `f32` and replace the low lane of the input.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = convert_replace_i64_m128_s(a, 5_i64).to_array();
/// assert_eq!(b, [5.0, 2.0, 3.0, 4.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg(arch = "x86_64")]
pub fn convert_replace_i64_m128_s(a: m128, i: i64) -> m128 {
  m128(unsafe { _mm_cvtsi64_ss(a.0, i) })
}

/// Gets the low lane as an individual `f32` value.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// assert_eq!(1_f32, get_f32_m128_s(a));
/// ```
#[must_use]
#[inline(always)]
pub fn get_f32_m128_s(a: m128) -> f32 {
  unsafe { _mm_cvtss_f32(a.0) }
}

/// Converts the low lane to `i32` and extracts as an individual value.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// assert_eq!(1_i32, convert_get_i32_m128_s(a));
/// ```
#[must_use]
#[inline(always)]
pub fn convert_get_i32_m128_s(a: m128) -> i32 {
  unsafe { _mm_cvtss_si32(a.0) }
}

/// Converts the low lane to `i64` and extracts as an individual value.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// assert_eq!(1_i64, convert_get_i64_m128_s(a));
/// ```
#[must_use]
#[inline(always)]
#[cfg(arch = "x86_64")]
pub fn convert_get_i64_m128_s(a: m128) -> i64 {
  unsafe { _mm_cvttss_si64(a.0) }
}

/// Lanewise `a / b`.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([10.0, 12.0, 13.0, 14.0]);
/// let b = m128::from_array([2.0, 6.0, 13.0, 2.0]);
/// let c = div_m128(a, b).to_array();
/// assert_eq!(c, [5.0, 2.0, 1.0, 7.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn div_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_div_ps(a.0, b.0) })
}

/// Low lane `a / b`, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([10.0, 12.0, 13.0, 14.0]);
/// let b = m128::from_array([2.0, 6.0, 13.0, 2.0]);
/// let c = div_m128_s(a, b).to_array();
/// assert_eq!(c, [5.0, 12.0, 13.0, 14.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn div_m128_s(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_div_ss(a.0, b.0) })
}

/// Loads the reference into a register.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([10.0, 12.0, 13.0, 14.0]);
/// let b = load_m128(&a);
/// let c = cmp_eq_m128_mask(a, b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [u32::MAX, u32::MAX, u32::MAX, u32::MAX]);
/// ```
#[must_use]
#[inline(always)]
pub fn load_m128(a: &m128) -> m128 {
  m128(unsafe { _mm_load_ps(a as *const m128 as *const f32) })
}

/// Loads the reference into all lanes of a register.
/// ```
/// # use safe_arch::*;
/// let a = 1.0;
/// let b = load_splat_m128(&a);
/// let c =
///   cmp_eq_m128_mask(m128::from_array([1.0, 1.0, 1.0, 1.0]), b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [u32::MAX, u32::MAX, u32::MAX, u32::MAX]);
/// ```
#[must_use]
#[inline(always)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn load_splat_m128(a: &f32) -> m128 {
  m128(unsafe { _mm_load_ps1(a) })
}

/// Loads the reference into all lanes of a register.
/// ```
/// # use safe_arch::*;
/// let a = 1.0;
/// let b = load_f32_m128_s(&a);
/// let c =
///   cmp_eq_m128_mask(m128::from_array([1.0, 0.0, 0.0, 0.0]), b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [u32::MAX, u32::MAX, u32::MAX, u32::MAX]);
/// ```
#[must_use]
#[inline(always)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn load_f32_m128_s(a: &f32) -> m128 {
  m128(unsafe { _mm_load_ss(a) })
}

/// Loads the reference into a register with reversed order.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([10.0, 12.0, 13.0, 14.0]);
/// let b = load_reverse_m128(&a);
/// let c =
///   cmp_eq_m128_mask(m128::from_array([14.0, 13.0, 12.0, 10.0]), b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [u32::MAX, u32::MAX, u32::MAX, u32::MAX]);
/// ```
#[must_use]
#[inline(always)]
pub fn load_reverse_m128(a: &m128) -> m128 {
  m128(unsafe { _mm_loadr_ps(a as *const m128 as *const f32) })
}

/// Loads the reference into a register.
///
/// This generally has no speed penalty if the reference happens to be 16-byte
/// aligned, but there is a slight speed penalty if the reference is only 4-byte
/// aligned.
/// ```
/// # use safe_arch::*;
/// let a = [10.0, 12.0, 13.0, 14.0];
/// let b = load_unaligned_m128(&a);
/// let c =
///   cmp_eq_m128_mask(m128::from_array([10.0, 12.0, 13.0, 14.0]), b).to_array();
/// let d: [u32; 4] = unsafe { core::mem::transmute(c) };
/// assert_eq!(d, [u32::MAX, u32::MAX, u32::MAX, u32::MAX]);
/// ```
#[must_use]
#[inline(always)]
pub fn load_unaligned_m128(a: &[f32; 4]) -> m128 {
  m128(unsafe { _mm_loadu_ps(a as *const [f32; 4] as *const f32) })
}

/// Lanewise `max(a, b)`.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 12.0, 3.0, 4.0]);
/// let b = m128::from_array([5.0, 6.0, 7.0, 8.5]);
/// let c = max_m128(a, b).to_array();
/// assert_eq!(c, [5.0, 12.0, 7.0, 8.5]);
/// ```
#[must_use]
#[inline(always)]
pub fn max_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_max_ps(a.0, b.0) })
}

/// Low lane `max(a, b)`, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 12.0, 3.0, 4.0]);
/// let b = m128::from_array([5.0, 6.0, 7.0, 8.5]);
/// let c = max_m128_s(a, b).to_array();
/// assert_eq!(c, [5.0, 12.0, 3.0, 4.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn max_m128_s(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_max_ss(a.0, b.0) })
}

/// Lanewise `min(a, b)`.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 12.0, 3.0, 4.0]);
/// let b = m128::from_array([5.0, 6.0, 7.0, 8.5]);
/// let c = min_m128(a, b).to_array();
/// assert_eq!(c, [1.0, 6.0, 3.0, 4.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn min_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_min_ps(a.0, b.0) })
}

/// Low lane `min(a, b)`, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 12.0, 3.0, 4.0]);
/// let b = m128::from_array([0.0, 6.0, 7.0, 8.5]);
/// let c = min_m128_s(a, b).to_array();
/// assert_eq!(c, [0.0, 12.0, 3.0, 4.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn min_m128_s(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_min_ss(a.0, b.0) })
}

/// Move the low lane of `b` to `a`, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 12.0, 3.0, 4.0]);
/// let b = m128::from_array([8.0, 6.0, 7.0, 8.5]);
/// let c = move_m128_s(a, b).to_array();
/// assert_eq!(c, [8.0, 12.0, 3.0, 4.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn move_m128_s(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_move_ss(a.0, b.0) })
}

/// Move the high lanes of `b` to the low lanes of `a`, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 12.0, 3.0, 4.0]);
/// let b = m128::from_array([8.0, 6.0, 7.0, 8.5]);
/// let c = move_high_low_m128(a, b).to_array();
/// assert_eq!(c, [7.0, 8.5, 3.0, 4.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn move_high_low_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_movehl_ps(a.0, b.0) })
}

/// Move the low lanes of `b` to the high lanes of `a`, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 12.0, 3.0, 4.0]);
/// let b = m128::from_array([8.0, 6.0, 7.0, 8.5]);
/// let c = move_low_high_m128(a, b).to_array();
/// assert_eq!(c, [1.0, 12.0, 8.0, 6.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn move_low_high_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_movelh_ps(a.0, b.0) })
}

/// Gathers the sign bit of each lane as an `i32`.
///
/// The output has lane 0 as bit 0, lane 1 as bit 1, and so on.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([-1.0, 12.0, -3.0, -4.0]);
/// let i = move_mask_m128(a);
/// assert_eq!(i, 0b1101);
/// ```
#[must_use]
#[inline(always)]
pub fn move_mask_m128(a: m128) -> i32 {
  unsafe { _mm_movemask_ps(a.0) }
}

/// Lanewise `a * b`.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([5.0, 6.0, 7.0, 8.5]);
/// let c = mul_m128(a, b).to_array();
/// assert_eq!(c, [5.0, 12.0, 21.0, 34.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn mul_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_mul_ps(a.0, b.0) })
}

/// Low lane `a * b`, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([5.0, 6.0, 7.0, 8.5]);
/// let c = mul_m128_s(a, b).to_array();
/// assert_eq!(c, [5.0, 2.0, 3.0, 4.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn mul_m128_s(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_mul_ss(a.0, b.0) })
}

/// Bitwise `a | b`.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 0.0, 1.0, 0.0]);
/// let b = m128::from_array([1.0, 1.0, 0.0, 0.0]);
/// let c = or_m128(a, b).to_array();
/// assert_eq!(c, [1.0, 1.0, 1.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn or_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_or_ps(a.0, b.0) })
}

/// Lanewise `1.0 / a` approximation.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 4.0, 8.0]);
/// let b = reciprocal_m128(a).to_array();
/// let expected = [1.0, 0.5, 0.25, 0.125];
/// for i in 0..4 {
///   assert!((b[i] - expected[i]).abs() < 0.001);
/// }
/// ```
#[must_use]
#[inline(always)]
pub fn reciprocal_m128(a: m128) -> m128 {
  m128(unsafe { _mm_rcp_ps(a.0) })
}

/// Low lane `1.0 / a` approximation, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 4.0, 8.0]);
/// let b = reciprocal_m128_s(a).to_array();
/// let expected = [1.0, 2.0, 4.0, 8.0];
/// for i in 0..4 {
///   assert!((b[i] - expected[i]).abs() < 0.001);
/// }
/// ```
#[must_use]
#[inline(always)]
pub fn reciprocal_m128_s(a: m128) -> m128 {
  m128(unsafe { _mm_rcp_ss(a.0) })
}

/// Lanewise `1.0 / sqrt(a)` approximation.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([16.0, 9.0, 4.0, 25.0]);
/// let b = reciprocal_sqrt_m128(a).to_array();
/// let expected = [0.25, 0.33333, 0.5, 0.2];
/// for i in 0..4 {
///   assert!((b[i] - expected[i]).abs() < 0.001);
/// }
/// ```
#[must_use]
#[inline(always)]
pub fn reciprocal_sqrt_m128(a: m128) -> m128 {
  m128(unsafe { _mm_rsqrt_ps(a.0) })
}

/// Low lane `1.0 / sqrt(a)` approximation, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([16.0, 8.0, 9.0, 10.0]);
/// let b = reciprocal_sqrt_m128_s(a).to_array();
/// let expected = [0.25, 8.0, 9.0, 10.0];
/// for i in 0..4 {
///   assert!((b[i] - expected[i]).abs() < 0.001);
/// }
/// ```
#[must_use]
#[inline(always)]
pub fn reciprocal_sqrt_m128_s(a: m128) -> m128 {
  m128(unsafe { _mm_rsqrt_ss(a.0) })
}

/// Sets the args into an `m128`, first arg is the high lane.
/// ```
/// # use safe_arch::*;
/// let a = set_m128(1.0, 2.0, 3.0, 4.0).to_array();
/// let b = m128::from_array([4.0, 3.0, 2.0, 1.0]).to_array();
/// assert_eq!(a, b);
/// ```
#[must_use]
#[inline(always)]
pub fn set_m128(three: f32, two: f32, one: f32, zero: f32) -> m128 {
  m128(unsafe { _mm_set_ps(three, two, one, zero) })
}

/// Sets the args into an `m128`, first arg is the high lane.
/// ```
/// # use safe_arch::*;
/// let a = set_m128_s(1.0).to_array();
/// let b = m128::from_array([1.0, 0.0, 0.0, 0.0]).to_array();
/// assert_eq!(a, b);
/// ```
#[must_use]
#[inline(always)]
pub fn set_m128_s(low: f32) -> m128 {
  m128(unsafe { _mm_set_ss(low) })
}

/// Splats the value to all lanes.
/// ```
/// # use safe_arch::*;
/// let a = splat_m128(1.0).to_array();
/// let b = m128::from_array([1.0, 1.0, 1.0, 1.0]).to_array();
/// assert_eq!(a, b);
/// ```
#[must_use]
#[inline(always)]
pub fn splat_m128(all: f32) -> m128 {
  m128(unsafe { _mm_set1_ps(all) })
}

/// Sets the args into an `m128`, first arg is the low lane.
/// ```
/// # use safe_arch::*;
/// let a = set_reversed_m128(1.0, 2.0, 3.0, 4.0).to_array();
/// let b = m128::from_array([1.0, 2.0, 3.0, 4.0]).to_array();
/// assert_eq!(a, b);
/// ```
#[must_use]
#[inline(always)]
pub fn set_reversed_m128(zero: f32, one: f32, two: f32, three: f32) -> m128 {
  m128(unsafe { _mm_setr_ps(zero, one, two, three) })
}

/// All lanes zero.
/// ```
/// # use safe_arch::*;
/// let a = zeroed_m128().to_array();
/// assert_eq!(a, [0.0, 0.0, 0.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn zeroed_m128() -> m128 {
  m128(unsafe { _mm_setzero_ps() })
}

/// Shuffles the lanes around.
///
/// This is a macro because the shuffle pattern must be a compile time constant,
/// and Rust doesn't currently support that for functions.
///
/// ## Two `m128` Inputs
/// You can provide two `m128` arguments, in which case:
/// * The low lanes of the output come from `$a`, as picked by `$z` and `$o`
///   (Zero and One)
/// * The high lanes of the output come from `$b`, as picked by `$t` and `$e`
///   (Two and threE).
/// * `$a` and `$b` must obviously be `m128` expressions.
/// * Each of the lane selection values is a lane index (`0..4`). They can be
///   any integer type as long as all four lane indexes are the same type. Out
///   of bounds index values are wrapped to just the low 2 bits.
/// * The lane selection values are combined into a private `const` which is
///   computed at compile time and then used at runtime. This means that you can
///   use literals, but you can also use the names of other constants or even a
///   `const fn` expression, if that is somehow is useful to you.
///
/// ```
/// # use safe_arch::*;
/// # use safe_arch::shuffle_m128;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([5.0, 6.0, 7.0, 8.0]);
/// //
/// let c = shuffle_m128!(a, b, 0, 0, 0, 0).to_array();
/// assert_eq!(c, [1.0, 1.0, 5.0, 5.0]);
/// //
/// let c = shuffle_m128!(a, b, 0, 1, 2, 3).to_array();
/// assert_eq!(c, [1.0, 2.0, 7.0, 8.0]);
/// //
/// let c = shuffle_m128!(a, b, 0, 2, 2, 1).to_array();
/// assert_eq!(c, [1.0, 3.0, 7.0, 6.0]);
/// ```
///
/// ## One `m128` Input
/// You can provide one `m128` argument, in which case the above variant is
/// called with `$a` as the input to both sides of the shuffle (note that any
/// potential side effects of evaluating `$a` are executed only once).
///
/// ```no_run
/// # use safe_arch::*;
/// # use safe_arch::shuffle_m128;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// //
/// let c = shuffle_m128!(a, 0, 0, 0, 0).to_array();
/// assert_eq!(c, [1.0, 1.0, 1.0, 1.0]);
/// //
/// let c = shuffle_m128!(a, 0, 1, 2, 3).to_array();
/// assert_eq!(c, [1.0, 2.0, 3.0, 4.0]);
/// //
/// let c = shuffle_m128!(a, 0, 2, 2, 1).to_array();
/// assert_eq!(c, [1.0, 3.0, 3.0, 2.0]);
/// ```
#[macro_export]
macro_rules! shuffle_m128 {
  ($a:expr, $b:expr, $z:expr, $o:expr, $t:expr, $e:expr) => {{
    const MASK: i32 =
      (($z & 0b11) | ($o & 0b11) << 2 | ($t & 0b11) << 4 | ($e & 0b11) << 6)
        as i32;
    let a: m128 = $a;
    let b: m128 = $b;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_shuffle_ps;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_shuffle_ps;
    m128(unsafe { _mm_shuffle_ps(a.0, b.0, MASK) })
  }};
  ($a:expr, $z:expr, $o:expr, $t:expr, $e:expr) => {{
    // Note(Lokathor): this makes sure that any side-effecting expressions we
    // get as input are only executed once, then that expression output goes
    // into both sides of the shuffle.
    let a: m128 = $a;
    shuffle_m128!(a, a, $z, $o, $t, $e)
  }};
}

/// Lanewise `sqrt(a)`.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([25.0, 16.0, 9.0, 4.0]);
/// let b = sqrt_m128(a).to_array();
/// assert_eq!(b, [5.0, 4.0, 3.0, 2.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn sqrt_m128(a: m128) -> m128 {
  m128(unsafe { _mm_sqrt_ps(a.0) })
}

/// Low lane `sqrt(a)`, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([4.0, 8.0, 7.0, 6.0]);
/// let b = sqrt_m128_s(a).to_array();
/// assert_eq!(b, [2.0, 8.0, 7.0, 6.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn sqrt_m128_s(a: m128) -> m128 {
  m128(unsafe { _mm_sqrt_ss(a.0) })
}

/// Stores the value to the reference given.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([10.0, 12.0, 13.0, 14.0]);
/// let mut b = zeroed_m128();
/// store_m128(&mut b, a);
/// let c = b.to_array();
/// assert_eq!(c, [10.0, 12.0, 13.0, 14.0]);
/// ```
#[inline(always)]
pub fn store_m128(r: &mut m128, a: m128) {
  unsafe { _mm_store_ps(r as *mut m128 as *mut f32, a.0) }
}

/// Stores the low lane value to the reference given.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([10.0, 12.0, 13.0, 14.0]);
/// let mut f = 0.0;
/// store_m128_s(&mut f, a);
/// assert_eq!(f, 10.0);
/// ```
#[inline(always)]
pub fn store_m128_s(r: &mut f32, a: m128) {
  unsafe { _mm_store_ss(r as *mut f32, a.0) }
}

/// Stores the low lane value to all lanes of the reference given.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([10.0, 12.0, 13.0, 14.0]);
/// let mut b = zeroed_m128();
/// store_splat_m128(&mut b, a);
/// let c = b.to_array();
/// assert_eq!(c, [10.0, 10.0, 10.0, 10.0]);
/// ```
#[inline(always)]
pub fn store_splat_m128(r: &mut m128, a: m128) {
  unsafe { _mm_store1_ps(r as *mut m128 as *mut f32, a.0) }
}

/// Stores the value to the reference given in reverse order.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([10.0, 12.0, 13.0, 14.0]);
/// let mut b = zeroed_m128();
/// store_reverse_m128(&mut b, a);
/// let c = b.to_array();
/// assert_eq!(c, [14.0, 13.0, 12.0, 10.0]);
/// ```
#[inline(always)]
pub fn store_reverse_m128(r: &mut m128, a: m128) {
  unsafe { _mm_storer_ps(r as *mut m128 as *mut f32, a.0) }
}

/// Stores the value to the reference given.
///
/// This generally has no speed penalty if the reference happens to be 16-byte
/// aligned, but there is a slight speed penalty if the reference is only 4-byte
/// aligned.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([10.0, 12.0, 13.0, 14.0]);
/// let mut b = [0.0; 4];
/// store_unaligned_m128(&mut b, a);
/// assert_eq!(b, [10.0, 12.0, 13.0, 14.0]);
/// ```
#[inline(always)]
pub fn store_unaligned_m128(r: &mut [f32; 4], a: m128) {
  unsafe { _mm_store_ps(r as *mut [f32; 4] as *mut f32, a.0) }
}

/// Lanewise `a - b`.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([5.0, 8.0, 12.0, 3.0]);
/// let c = sub_m128(a, b).to_array();
/// assert_eq!(c, [-4.0, -6.0, -9.0, 1.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn sub_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_sub_ps(a.0, b.0) })
}

/// Low lane `a - b`, other lanes unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([5.0, 8.0, 12.0, 3.0]);
/// let c = sub_m128_s(a, b).to_array();
/// assert_eq!(c, [-4.0, 2.0, 3.0, 4.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn sub_m128_s(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_sub_ss(a.0, b.0) })
}

/// Transpose four `m128` as if they were a 4x4 matrix.
/// ```
/// # use safe_arch::*;
/// let mut a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let mut b = m128::from_array([5.0, 6.0, 7.0, 8.0]);
/// let mut c = m128::from_array([9.0, 10.0, 11.0, 12.0]);
/// let mut d = m128::from_array([13.0, 14.0, 15.0, 16.0]);
/// transpose_four_m128(&mut a, &mut b, &mut c, &mut d);
/// assert_eq!(a.to_array(), [1.0, 5.0, 9.0, 13.0]);
/// assert_eq!(b.to_array(), [2.0, 6.0, 10.0, 14.0]);
/// assert_eq!(c.to_array(), [3.0, 7.0, 11.0, 15.0]);
/// assert_eq!(d.to_array(), [4.0, 8.0, 12.0, 16.0]);
/// ```
#[inline(always)]
pub fn transpose_four_m128(
  a: &mut m128,
  b: &mut m128,
  c: &mut m128,
  d: &mut m128,
) {
  unsafe { _MM_TRANSPOSE4_PS(&mut a.0, &mut b.0, &mut c.0, &mut d.0) }
}

/// Unpack and interleave high lanes of `a` and `b`.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([5.0, 6.0, 7.0, 8.0]);
/// let c = unpack_high_m128(a, b).to_array();
/// assert_eq!(c, [3.0, 7.0, 4.0, 8.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn unpack_high_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_unpackhi_ps(a.0, b.0) })
}

/// Unpack and interleave low lanes of `a` and `b`.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let b = m128::from_array([5.0, 6.0, 7.0, 8.0]);
/// let c = unpack_low_m128(a, b).to_array();
/// assert_eq!(c, [1.0, 5.0, 2.0, 6.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn unpack_low_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_unpacklo_ps(a.0, b.0) })
}

/// Bitwise `a ^ b`.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 0.0, 1.0, 0.0]);
/// let b = m128::from_array([1.0, 1.0, 0.0, 0.0]);
/// let c = xor_m128(a, b).to_array();
/// assert_eq!(c, [0.0, 1.0, 1.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn xor_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_xor_ps(a.0, b.0) })
}

//
// Here we define the Operator Overloads for `m128`. Each one just calls the
// correct function from above. By putting the impls here and not with the
// `m128` type we theoretically would be able to build the crate safely even if
// there's no `sse` feature enabled. You'd just have a `m128` type without the
// operator overloads is all. Not that the standard Rust distribution can build
// properly without `sse` enabled, but maybe you're using a custom target or
// something. It doesn't really put us out of our way, so it doesn't hurt to try
// and accommodate the potential use case.
//

impl Add for m128 {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    add_m128(self, rhs)
  }
}
impl AddAssign for m128 {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl BitAnd for m128 {
  type Output = Self;
  fn bitand(self, rhs: Self) -> Self {
    and_m128(self, rhs)
  }
}
impl BitAndAssign for m128 {
  fn bitand_assign(&mut self, rhs: Self) {
    *self = *self & rhs;
  }
}

impl BitOr for m128 {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self {
    or_m128(self, rhs)
  }
}
impl BitOrAssign for m128 {
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}

impl BitXor for m128 {
  type Output = Self;
  fn bitxor(self, rhs: Self) -> Self {
    xor_m128(self, rhs)
  }
}
impl BitXorAssign for m128 {
  fn bitxor_assign(&mut self, rhs: Self) {
    *self = *self ^ rhs;
  }
}

impl Div for m128 {
  type Output = Self;
  fn div(self, rhs: Self) -> Self {
    div_m128(self, rhs)
  }
}
impl DivAssign for m128 {
  fn div_assign(&mut self, rhs: Self) {
    *self = *self / rhs;
  }
}

impl Mul for m128 {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self {
    mul_m128(self, rhs)
  }
}
impl MulAssign for m128 {
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl Neg for m128 {
  type Output = Self;
  fn neg(self) -> Self {
    sub_m128(zeroed_m128(), self)
  }
}

impl Not for m128 {
  type Output = Self;
  /// Not a direct intrinsic, but it's useful and the implementation is simple
  /// enough.
  ///
  /// This performs an `xor` with an all-1s bit pattern.
  fn not(self) -> Self {
    let all_bits = splat_m128(f32::from_bits(u32::MAX));
    self ^ all_bits
  }
}

impl Sub for m128 {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    sub_m128(self, rhs)
  }
}
impl SubAssign for m128 {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}
