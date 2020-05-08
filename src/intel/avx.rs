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
pub fn andnot_m256(a: m256, b: m256) -> m256 {
  m256(unsafe { _mm256_andnot_ps(a.0, b.0) })
}

// _mm256_blend_pd
// _mm256_blend_ps
// _mm256_blendv_pd
// _mm256_blendv_ps
// _mm256_broadcast_pd
// _mm256_broadcast_ps
// _mm256_broadcast_sd
// _mm_broadcast_ss
// _mm256_broadcast_ss
// _mm256_castpd_ps
// _mm256_castpd_si256
// _mm256_castpd128_pd256
// _mm256_castpd256_pd128
// _mm256_castps_pd
// _mm256_castps_si256
// _mm256_castps128_ps256
// _mm256_castps256_ps128
// _mm256_castsi128_si256
// _mm256_castsi256_pd
// _mm256_castsi256_ps
// _mm256_castsi256_si128
// _mm256_ceil_pd
// _mm256_ceil_ps
// _mm_cmp_pd
// _mm256_cmp_pd
// _mm_cmp_ps
// _mm256_cmp_ps
// _mm_cmp_sd
// _mm_cmp_ss
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
