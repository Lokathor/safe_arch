#![cfg(target_feature = "sse3")]

use super::*;

/// Add the high lane and subtract the low lane.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([10.0, 50.0]);
/// let b = m128d::from_array([100.0, 500.0]);
/// let c = add_sub_m128d(a, b).to_array();
/// assert_eq!(c, [-90.0, 550.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse3")))]
pub fn add_sub_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_addsub_pd(a.0, b.0) })
}

/// Alternately, from the top, add a lane and then subtract a lane.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([10.0, 20.0, 30.0, 40.0]);
/// let b = m128::from_array([100.0, 200.0, 300.0, 400.0]);
/// let c = add_sub_m128(a, b).to_array();
/// assert_eq!(c, [-90.0, 220.0, -270.0, 440.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse3")))]
pub fn add_sub_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_addsub_ps(a.0, b.0) })
}

/// Add each lane horizontally, pack the outputs as `a` then `b`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([10.0, 50.0]);
/// let b = m128d::from_array([100.0, 500.0]);
/// let c = add_horizontal_m128d(a, b).to_array();
/// assert_eq!(c, [60.0, 600.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse3")))]
pub fn add_horizontal_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_hadd_pd(a.0, b.0) })
}

/// Add each lane horizontally, pack the outputs as `a` then `b`.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([10.0, 20.0, 30.0, 40.0]);
/// let b = m128::from_array([100.0, 200.0, 300.0, 400.0]);
/// let c = add_horizontal_m128(a, b).to_array();
/// assert_eq!(c, [30.0, 70.0, 300.0, 700.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse3")))]
pub fn add_horizontal_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_hadd_ps(a.0, b.0) })
}

/// Subtract each lane horizontally, pack the outputs as `a` then `b`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([10.0, 50.0]);
/// let b = m128d::from_array([100.0, 500.0]);
/// let c = sub_horizontal_m128d(a, b).to_array();
/// assert_eq!(c, [-40.0, -400.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse3")))]
pub fn sub_horizontal_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_hsub_pd(a.0, b.0) })
}

/// Subtract each lane horizontally, pack the outputs as `a` then `b`.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([10.0, 20.0, 30.0, 45.0]);
/// let b = m128::from_array([100.0, 200.0, 300.0, 450.0]);
/// let c = sub_horizontal_m128(a, b).to_array();
/// assert_eq!(c, [-10.0, -15.0, -100.0, -150.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse3")))]
pub fn sub_horizontal_m128(a: m128, b: m128) -> m128 {
  m128(unsafe { _mm_hsub_ps(a.0, b.0) })
}

/// Copy the low lane of the input to both lanes of the output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 2.0]);
/// let b = duplicate_low_lane_m128d_s(a);
/// assert_eq!(b.to_array(), [1.0, 1.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse3")))]
pub fn duplicate_low_lane_m128d_s(a: m128d) -> m128d {
  m128d(unsafe { _mm_movedup_pd(a.0) })
}

/// Duplicate the odd lanes to the even lanes.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([0.0, 1.0, 2.0, 3.0]);
/// let b = duplicate_odd_lanes_m128(a);
/// assert_eq!(b.to_array(), [1.0, 1.0, 3.0, 3.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse3")))]
pub fn duplicate_odd_lanes_m128(a: m128) -> m128 {
  m128(unsafe { _mm_movehdup_ps(a.0) })
}

/// Duplicate the odd lanes to the even lanes.
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([0.0, 1.0, 2.0, 3.0]);
/// let b = duplicate_even_lanes_m128(a);
/// assert_eq!(b.to_array(), [0.0, 0.0, 2.0, 2.0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse3")))]
pub fn duplicate_even_lanes_m128(a: m128) -> m128 {
  m128(unsafe { _mm_moveldup_ps(a.0) })
}
