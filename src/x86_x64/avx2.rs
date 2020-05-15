#![cfg(target_feature = "avx2")]

use super::*;

/// Blends the `i32` lanes in `$a` and `$b` into a single value.
///
/// * The blend is controlled by an immediate mask value (an `i32`).
/// * For each lane `0..=3`, use `0` if you want that lane of the output to be
///   from `$a` and use `1` if you want that lane of the output to be from `$b`.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([10, 20, 30, 40]);
/// let b = m128i::from([100, 200, 300, 400]);
/// //
/// let c: [i32; 4] = blend_i32_m128i!(a, b, 0b0110).into();
/// assert_eq!(c, [10, 200, 300, 40]);
/// ```
/// * **Intrinsic:** [`_mm_blend_epi32`]
/// * **Assembly:** `vpblendd xmm, xmm, xmm, imm8`
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
macro_rules! blend_i32_m128i {
  ($a:expr, $b:expr, $imm:expr) => {{
    let a: $crate::m128i = $a;
    let b: $crate::m128i = $b;
    const IMM: ::core::primitive::i32 = $imm;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_blend_epi32;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_blend_epi32;
    $crate::m128i(unsafe { _mm_blend_epi32(a.0, b.0, IMM) })
  }};
}

/// Splat the lowest 8-bit lane across the entire 128 bits.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(0x77_i128);
/// let b: [i8; 16] = splat_i8_m128i_s_m128i(a).into();
/// assert_eq!(b, [0x77_i8; 16]);
/// ```
/// * **Intrinsic:** [`_mm_broadcastb_epi8`]
/// * **Assembly:** `vpbroadcastb xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn splat_i8_m128i_s_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_broadcastb_epi8(a.0) })
}

/// Splat the lowest 16-bit lane across the entire 128 bits.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(0x77_i128);
/// let b: [i16; 8] = splat_i16_m128i_s_m128i(a).into();
/// assert_eq!(b, [0x77_i16; 8]);
/// ```
/// * **Intrinsic:** [`_mm_broadcastw_epi16`]
/// * **Assembly:** `vpbroadcastw xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn splat_i16_m128i_s_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_broadcastw_epi16(a.0) })
}

/// Splat the lowest 32-bit lane across the entire 128 bits.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(0x77_i128);
/// let b: [i32; 4] = splat_i32_m128i_s_m128i(a).into();
/// assert_eq!(b, [0x77; 4]);
/// ```
/// * **Intrinsic:** [`_mm_broadcastd_epi32`]
/// * **Assembly:** `vpbroadcastd xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn splat_i32_m128i_s_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_broadcastd_epi32(a.0) })
}

/// Splat the lowest 64-bit lane across the entire 128 bits.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(0x77_i128);
/// let b: [i64; 2] = splat_i64_m128i_s_m128i(a).into();
/// assert_eq!(b, [0x77_i64; 2]);
/// ```
/// * **Intrinsic:** [`_mm_broadcastq_epi64`]
/// * **Assembly:** `vpbroadcastq xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn splat_i64_m128i_s_m128i(a: m128i) -> m128i {
  m128i(unsafe { _mm_broadcastq_epi64(a.0) })
}

/// Splat the lower `f64` across both lanes of `m128d`.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from([1.0, 2.0]);
/// let b = splat_m128d_s_m128d(a).to_array();
/// assert_eq!(b, [1.0, 1.0]);
/// ```
/// * **Intrinsic:** [`_mm_broadcastsd_pd`]
/// * **Assembly:** `movddup xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn splat_m128d_s_m128d(a: m128d) -> m128d {
  m128d(unsafe { _mm_broadcastsd_pd(a.0) })
}

/// Splat the 128-bits across 256-bits.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(1_i128);
/// let b: [i128; 2] = splat_m128i_m256i(a).into();
/// assert_eq!(b, [1_i128, 1]);
/// ```
/// * **Intrinsic:** [`_mm256_broadcastsi128_si256`]
/// * **Assembly:** `vbroadcasti128 ymm, m128`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn splat_m128i_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_broadcastsi128_si256(a.0) })
}

/// Splat the lowest `f32` across all four lanes.
/// ```
/// # use safe_arch::*;
/// let a = set_m128_s(1.0);
/// let b = splat_m128_s_m128(a).to_array();
/// assert_eq!(b, [1.0, 1.0, 1.0, 1.0]);
/// ```
/// * **Intrinsic:** [`_mm_broadcastss_ps`]
/// * **Assembly:** `vbroadcastss xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn splat_m128_s_m128(a: m128) -> m128 {
  m128(unsafe { _mm_broadcastss_ps(a.0) })
}

// _mm_maskload_epi32
// _mm_maskload_epi64
// _mm_maskstore_epi32
// _mm_maskstore_epi64
// _mm_sllv_epi32
// _mm_sllv_epi64
// _mm_srav_epi32
// _mm_srlv_epi32
// _mm_srlv_epi64
// _mm256_abs_epi16
// _mm256_abs_epi32
// _mm256_abs_epi8
// _mm256_add_epi16
// _mm256_add_epi32
// _mm256_add_epi64
// _mm256_add_epi8
// _mm256_adds_epi16
// _mm256_adds_epi8
// _mm256_adds_epu16
// _mm256_adds_epu8
// _mm256_alignr_epi8
// _mm256_and_si256
// _mm256_andnot_si256
// _mm256_avg_epu16
// _mm256_avg_epu8
// _mm256_blend_epi16
// _mm256_blend_epi32
// _mm256_blendv_epi8
// _mm256_broadcastb_epi8
// _mm256_broadcastd_epi32
// _mm256_broadcastq_epi64
// _mm256_broadcastsd_pd
// _mm256_broadcastss_ps
// _mm256_broadcastw_epi16
// _mm256_bslli_epi128
// _mm256_bsrli_epi128
// _mm256_cmpeq_epi16
// _mm256_cmpeq_epi32
// _mm256_cmpeq_epi64
// _mm256_cmpeq_epi8
// _mm256_cmpgt_epi16
// _mm256_cmpgt_epi32
// _mm256_cmpgt_epi64
// _mm256_cmpgt_epi8
// _mm256_cvtepi16_epi32
// _mm256_cvtepi16_epi64
// _mm256_cvtepi32_epi64
// _mm256_cvtepi8_epi16
// _mm256_cvtepi8_epi32
// _mm256_cvtepi8_epi64
// _mm256_cvtepu16_epi32
// _mm256_cvtepu16_epi64
// _mm256_cvtepu32_epi64
// _mm256_cvtepu8_epi16
// _mm256_cvtepu8_epi32
// _mm256_cvtepu8_epi64
// _mm256_extract_epi16
// _mm256_extract_epi8
// _mm256_extracti128_si256
// _mm256_hadd_epi16
// _mm256_hadd_epi32
// _mm256_hadds_epi16
// _mm256_hsub_epi16
// _mm256_hsub_epi32
// _mm256_hsubs_epi16
// _mm256_inserti128_si256
// _mm256_madd_epi16
// _mm256_maddubs_epi16
// _mm256_maskload_epi32
// _mm256_maskload_epi64
// _mm256_maskstore_epi32
// _mm256_maskstore_epi64
// _mm256_max_epi16
// _mm256_max_epi32
// _mm256_max_epi8
// _mm256_max_epu16
// _mm256_max_epu32
// _mm256_max_epu8
// _mm256_min_epi16
// _mm256_min_epi32
// _mm256_min_epi8
// _mm256_min_epu16
// _mm256_min_epu32
// _mm256_min_epu8
// _mm256_movemask_epi8
// _mm256_mpsadbw_epu8
// _mm256_mul_epi32
// _mm256_mul_epu32
// _mm256_mulhi_epi16
// _mm256_mulhi_epu16
// _mm256_mulhrs_epi16
// _mm256_mullo_epi16
// _mm256_mullo_epi32
// _mm256_or_si256
// _mm256_packs_epi16
// _mm256_packs_epi32
// _mm256_packus_epi16
// _mm256_packus_epi32
// _mm256_permute2x128_si256
// _mm256_permute4x64_epi64
// _mm256_permute4x64_pd
// _mm256_permutevar8x32_epi32
// _mm256_permutevar8x32_ps
// _mm256_sad_epu8
// _mm256_shuffle_epi32
// _mm256_shuffle_epi8
// _mm256_shufflehi_epi16
// _mm256_shufflelo_epi16
// _mm256_sign_epi16
// _mm256_sign_epi32
// _mm256_sign_epi8
// _mm256_sll_epi16
// _mm256_sll_epi32
// _mm256_sll_epi64
// _mm256_slli_epi16
// _mm256_slli_epi32
// _mm256_slli_epi64
// _mm256_slli_si256
// _mm256_sllv_epi32
// _mm256_sllv_epi64
// _mm256_sra_epi16
// _mm256_sra_epi32
// _mm256_srai_epi16
// _mm256_srai_epi32
// _mm256_srav_epi32
// _mm256_srl_epi16
// _mm256_srl_epi32
// _mm256_srl_epi64
// _mm256_srli_epi16
// _mm256_srli_epi32
// _mm256_srli_epi64
// _mm256_srli_si256
// _mm256_srlv_epi32
// _mm256_srlv_epi64
// _mm256_stream_load_si256
// _mm256_sub_epi16
// _mm256_sub_epi32
// _mm256_sub_epi64
// _mm256_sub_epi8
// _mm256_subs_epi16
// _mm256_subs_epi8
// _mm256_subs_epu16
// _mm256_subs_epu8
// _mm256_unpackhi_epi16
// _mm256_unpackhi_epi32
// _mm256_unpackhi_epi64
// _mm256_unpackhi_epi8
// _mm256_unpacklo_epi16
// _mm256_unpacklo_epi32
// _mm256_unpacklo_epi64
// _mm256_unpacklo_epi8
// _mm256_xor_si256
