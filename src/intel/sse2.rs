#![cfg(target_feature = "sse2")]

use super::*;

/// Returns if the f64 array and u64 array are the same bit pattern.
#[doc(hidden)]
pub fn debug_check_same_bits_f64(f: [f64; 2], u: [u64; 2]) -> bool {
  let x: [u64; 2] = unsafe { core::mem::transmute(f) };
  x == u
}

/// Lanewise `a + b` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(
///   [0_i8, 1, 2, 3, 4, 5, 6, 7,
///   8, 9, 10, 11, 12, 13, 14, 15]
/// );
/// let b = m128i::from(
///   [0_i8, 11, 2, 13, 4, 15, 6, 17,
///   8, 19, -20, 21, 22, -23, 24, 127]
/// );
/// let c: [i8; 16] = add_i8_m128i(a, b).into();
/// assert_eq!(
///   c,
///   [0, 12, 4, 16, 8, 20, 12, 24, 16,
///   28, -10, 32, 34, -10, 38, -114]
/// );
/// ```
#[must_use]
#[inline(always)]
#[rustfmt::skip]
pub fn add_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_add_epi8(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `i16`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 2, 3, 4, -1, -2, -3, -4]);
/// let b = m128i::from([5_i16, 6, 7, 8, -15, -26, -37, 48]);
/// let c: [i16; 8] = add_i16_m128i(a, b).into();
/// assert_eq!(c, [6, 8, 10, 12, -16, -28, -40, 44]);
/// ```
#[must_use]
#[inline(always)]
pub fn add_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_add_epi16(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `i32`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let b = m128i::from([5, 6, 7, 8]);
/// let c: [i32; 4] = add_i32_m128i(a, b).into();
/// assert_eq!(c, [6, 8, 10, 12]);
/// ```
#[must_use]
#[inline(always)]
pub fn add_i32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_add_epi32(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `i64`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([92_i64, 87]);
/// let b = m128i::from([-9001_i64, 1]);
/// let c: [i64; 2] = add_i64_m128i(a, b).into();
/// assert_eq!(c, [-8909, 88]);
/// ```
#[must_use]
#[inline(always)]
pub fn add_i64_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_add_epi64(a.0, b.0) })
}

/// Lanewise `a + b`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([92.0, 87.5]);
/// let b = m128d::from_array([100.0, -6.0]);
/// let c = add_m128d(a, b).to_array();
/// assert_eq!(c, [192.0, 81.5]);
/// ```
#[must_use]
#[inline(always)]
pub fn add_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_add_pd(a.0, b.0) })
}

/// Lowest lane `a + b`, high lane unchanged.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([92.0, 87.5]);
/// let b = m128d::from_array([100.0, -600.0]);
/// let c = add_m128d_s(a, b).to_array();
/// assert_eq!(c, [192.0, 87.5]);
/// ```
#[must_use]
#[inline(always)]
pub fn add_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_add_sd(a.0, b.0) })
}

/// Lanewise saturating `a + b` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([
///   i8::MAX, i8::MIN, 3, 4, -1, -2, -3, -4,
///   3, 4, -1, -2, -1, -2, -3, -4,
/// ]);
/// let b = m128i::from([
///   i8::MAX, i8::MIN, 7, 8, -15, -26, -37, 48,
///   7, 8, -15, -26, -15, -26, -37, 48,
/// ]);
/// let c: [i8; 16] = add_saturating_i8_m128i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     i8::MAX, i8::MIN, 10, 12, -16, -28, -40, 44,
///     10, 12, -16, -28, -16, -28, -40, 44
///   ]
/// );
/// ```
#[must_use]
#[inline(always)]
#[rustfmt::skip]
pub fn add_saturating_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_adds_epi8(a.0, b.0) })
}

/// Lanewise saturating `a + b` with lanes as `i16`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([i16::MAX, i16::MIN, 3, 4, -1, -2, -3, -4]);
/// let b = m128i::from([i16::MAX, i16::MIN, 7, 8, -15, -26, -37, 48]);
/// let c: [i16; 8] = add_saturating_i16_m128i(a, b).into();
/// assert_eq!(c, [i16::MAX, i16::MIN, 10, 12, -16, -28, -40, 44]);
/// ```
#[must_use]
#[inline(always)]
pub fn add_saturating_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_adds_epi16(a.0, b.0) })
}

/// Lanewise saturating `a + b` with lanes as `u8`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([
///   u8::MAX, 0, 3, 4, 254, 2, 3, 4,
///   3, 4, 1, 2, 1, 2, 128, 4,
/// ]);
/// let b = m128i::from([
///   u8::MAX, 0, 7, 8, 15, 26, 37, 48,
///   7, 8, 15, 26, 15, 26, 37, 48,
/// ]);
/// let c: [u8; 16] = add_saturating_u8_m128i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     u8::MAX, 0, 10, 12, 255, 28, 40, 52,
///     10, 12, 16, 28, 16, 28, 165, 52
///   ]
/// );
/// ```
#[must_use]
#[inline(always)]
#[rustfmt::skip]
pub fn add_saturating_u8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_adds_epu8(a.0, b.0) })
}

/// Lanewise saturating `a + b` with lanes as `u16`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([u16::MAX, 0, 3, 4, 1, 2, 3, 4]);
/// let b = m128i::from([u16::MAX, 0, 7, 8, 15, 26, 37, 48]);
/// let c: [u16; 8] = add_saturating_u16_m128i(a, b).into();
/// assert_eq!(c, [u16::MAX, 0, 10, 12, 16, 28, 40, 52]);
/// ```
#[must_use]
#[inline(always)]
pub fn add_saturating_u16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_adds_epu16(a.0, b.0) })
}

/// Bitwise `a & b`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = and_m128d(a, b).to_array();
/// assert_eq!(c, [1.0, 0.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn and_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_and_pd(a.0, b.0) })
}

/// Bitwise `a & b`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 0, 1, 0]);
/// let b = m128i::from([1, 1, 0, 0]);
/// let c: [i32; 4] = and_m128i(a, b).into();
/// assert_eq!(c, [1, 0, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn and_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_and_si128(a.0, b.0) })
}

/// Bitwise `(!a) & b`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = andnot_m128d(a, b).to_array();
/// assert_eq!(c, [0.0, 1.0]);
/// ```
#[must_use]
#[inline(always)]
pub fn andnot_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_andnot_pd(a.0, b.0) })
}

/// Bitwise `(!a) & b`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 0, 1, 0]);
/// let b = m128i::from([1, 1, 0, 0]);
/// let c: [i32; 4] = andnot_m128i(a, b).into();
/// assert_eq!(c, [0, 1, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn andnot_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_andnot_si128(a.0, b.0) })
}

/// Lanewise saturating `a + b` with lanes as `u8`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([
///   u8::MAX, 0, 3, 4, 254, 2, 3, 4,
///   3, 4, 1, 2, 1, 2, 128, 4,
/// ]);
/// let b = m128i::from([
///   u8::MAX, 0, 7, 8, 15, 26, 37, 48,
///   7, 8, 15, 26, 15, 26, 37, 48,
/// ]);
/// let c: [u8; 16] = average_u8_m128i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     u8::MAX, 0, 10, 12, 255, 28, 40, 52,
///     10, 12, 16, 28, 16, 28, 165, 52
///   ]
/// );
/// ```
#[must_use]
#[inline(always)]
#[rustfmt::skip]
pub fn average_u8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_avg_epu8(a.0, b.0) })
}

/// Lanewise saturating `a + b` with lanes as `u16`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([u16::MAX, 0, 3, 4, 1, 2, 3, 4]);
/// let b = m128i::from([u16::MAX, 0, 7, 8, 15, 26, 37, 48]);
/// let c: [u16; 8] = average_u16_m128i(a, b).into();
/// assert_eq!(c, [u16::MAX, 0, 10, 12, 16, 28, 40, 52]);
/// ```
#[must_use]
#[inline(always)]
pub fn average_u16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_avg_epu16(a.0, b.0) })
}

// _mm_bslli_si128

// _mm_bsrli_si128

// _mm_castpd_ps

// _mm_castpd_si128

// _mm_castps_pd

// _mm_castps_si128

// _mm_castsi128_pd

// _mm_castsi128_ps

// _mm_clflush

// _mm_cmpeq_epi16

// _mm_cmpeq_epi32

// _mm_cmpeq_epi8

// _mm_cmpeq_pd

// _mm_cmpeq_sd

// _mm_cmpge_pd

// _mm_cmpge_sd

// _mm_cmpgt_epi16

// _mm_cmpgt_epi32

// _mm_cmpgt_epi8

// _mm_cmpgt_pd

// _mm_cmpgt_sd

// _mm_cmple_pd

// _mm_cmple_sd

// _mm_cmplt_epi16

// _mm_cmplt_epi32

// _mm_cmplt_epi8

// _mm_cmplt_pd

// _mm_cmplt_sd

// _mm_cmpneq_pd

// _mm_cmpneq_sd

// _mm_cmpnge_pd

// _mm_cmpnge_sd

// _mm_cmpngt_pd

// _mm_cmpngt_sd

// _mm_cmpnle_pd

// _mm_cmpnle_sd

// _mm_cmpnlt_pd

// _mm_cmpnlt_sd

// _mm_cmpord_pd

// _mm_cmpord_sd

// _mm_cmpunord_pd

// _mm_cmpunord_sd

// _mm_comieq_sd

// _mm_comige_sd

// _mm_comigt_sd

// _mm_comile_sd

// _mm_comilt_sd

// _mm_comineq_sd

// _mm_cvtepi32_pd

// _mm_cvtepi32_ps

// _mm_cvtpd_epi32

// _mm_cvtpd_pi32

// _mm_cvtpd_ps

// _mm_cvtpi32_pd

// _mm_cvtps_epi32

// _mm_cvtps_pd

// _mm_cvtsd_f64

// _mm_cvtsd_si32

// _mm_cvtsd_si64

// _mm_cvtsd_si64x

// _mm_cvtsd_ss

// _mm_cvtsi128_si32

// _mm_cvtsi128_si64

// _mm_cvtsi128_si64x

// _mm_cvtsi32_sd

// _mm_cvtsi32_si128

// _mm_cvtsi64_sd

// _mm_cvtsi64_si128

// _mm_cvtsi64x_sd

// _mm_cvtsi64x_si128

// _mm_cvtss_sd

// _mm_cvttpd_epi32

// _mm_cvttpd_pi32

// _mm_cvttps_epi32

// _mm_cvttsd_si32

// _mm_cvttsd_si64

// _mm_cvttsd_si64x

// _mm_div_pd

// _mm_div_sd

// _mm_extract_epi16

// _mm_insert_epi16

// _mm_lfence

// _mm_load_pd

// _mm_load_pd1

// _mm_load_sd

// _mm_load_si128

// _mm_load1_pd

// _mm_loadh_pd

// _mm_loadl_epi64

// _mm_loadl_pd

// _mm_loadr_pd

// _mm_loadu_pd

// _mm_loadu_si128

// _mm_loadu_si32

// _mm_madd_epi16

// _mm_maskmoveu_si128

// _mm_max_epi16

// _mm_max_epu8

// _mm_max_pd

// _mm_max_sd

// _mm_mfence

// _mm_min_epi16

// _mm_min_epu8

// _mm_min_pd

// _mm_min_sd

// _mm_move_epi64

// _mm_move_sd

// _mm_movemask_epi8

// _mm_movemask_pd

// _mm_movepi64_pi64

// _mm_movpi64_epi64

// _mm_mul_epu32

// _mm_mul_pd

// _mm_mul_sd

// _mm_mul_su32

// _mm_mulhi_epi16

// _mm_mulhi_epu16

// _mm_mullo_epi16

// _mm_or_pd

// _mm_or_si128

// _mm_packs_epi16

// _mm_packs_epi32

// _mm_packus_epi16

// _mm_pause

// _mm_sad_epu8

// _mm_set_epi16

// _mm_set_epi32

// _mm_set_epi64

// _mm_set_epi64x

// _mm_set_epi8

// _mm_set_pd

// _mm_set_pd1

// _mm_set_sd

// _mm_set1_epi16

// _mm_set1_epi32

// _mm_set1_epi64

// _mm_set1_epi64x

// _mm_set1_epi8

// _mm_set1_pd

// _mm_setr_epi16

// _mm_setr_epi32

// _mm_setr_epi64

// _mm_setr_epi8

// _mm_setr_pd

// _mm_setzero_pd

// _mm_setzero_si128

// _mm_shuffle_epi32

// _mm_shuffle_pd

// _mm_shufflehi_epi16

// _mm_shufflelo_epi16

// _mm_sll_epi16

// _mm_sll_epi32

// _mm_sll_epi64

// _mm_slli_epi16

// _mm_slli_epi32

// _mm_slli_epi64

// _mm_slli_si128

// _mm_sqrt_pd

// _mm_sqrt_sd

// _mm_sra_epi16

// _mm_sra_epi32

// _mm_srai_epi16

// _mm_srai_epi32

// _mm_srl_epi16

// _mm_srl_epi32

// _mm_srl_epi64

// _mm_srli_epi16

// _mm_srli_epi32

// _mm_srli_epi64

// _mm_srli_si128

// _mm_store_pd

// _mm_store_pd1

// _mm_store_sd

// _mm_store_si128

// _mm_store1_pd

// _mm_storeh_pd

// _mm_storel_epi64

// _mm_storel_pd

// _mm_storer_pd

// _mm_storeu_pd

// _mm_storeu_si128

// _mm_storeu_si32

// _mm_sub_epi16

// _mm_sub_epi32

// _mm_sub_epi64

// _mm_sub_epi8

// _mm_sub_pd

// _mm_sub_sd

// _mm_sub_si64

// _mm_subs_epi16

// _mm_subs_epi8

// _mm_subs_epu16

// _mm_subs_epu8

// _mm_ucomieq_sd

// _mm_ucomige_sd

// _mm_ucomigt_sd

// _mm_ucomile_sd

// _mm_ucomilt_sd

// _mm_ucomineq_sd

// _mm_unpackhi_epi16

// _mm_unpackhi_epi32

// _mm_unpackhi_epi64

// _mm_unpackhi_epi8

// _mm_unpackhi_pd

// _mm_unpacklo_epi16

// _mm_unpacklo_epi32

// _mm_unpacklo_epi64

// _mm_unpacklo_epi8

// _mm_unpacklo_pd

// _mm_xor_pd

// _mm_xor_si128
