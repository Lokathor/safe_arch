#![cfg(target_feature = "avx2")]

use super::*;

/*

TODO: uncomment these and call the correct functions before finalizing this PR.

impl Not for m256i {
  type Output = Self;
  /// Not a direct intrinsic, but it's very useful and the implementation is
  /// simple enough.
  ///
  /// Negates the bits by performing an `xor` with an all-1s bit pattern.
  #[must_use]
  #[inline(always)]
  fn not(self) -> Self {
    let all_bits = set_splat_m256(f32::from_bits(u32::MAX));
    let result = cast_from_m256i_to_m256(self) ^ all_bits;
    cast_from_m256_to_m256i(result)
  }
}

impl BitAnd for m256i {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitand(self, rhs: Self) -> Self {
    let rhs = cast_from_m256i_to_m256(rhs);
    let result = and_m256(cast_from_m256i_to_m256(self), rhs);
    cast_from_m256_to_m256i(result)
  }
}
impl BitAndAssign for m256i {
  #[inline(always)]
  fn bitand_assign(&mut self, rhs: Self) {
    *self = *self & rhs;
  }
}

impl BitOr for m256i {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitor(self, rhs: Self) -> Self {
    let rhs = cast_from_m256i_to_m256(rhs);
    let result = or_m256(cast_from_m256i_to_m256(self), rhs);
    cast_from_m256_to_m256i(result)
  }
}
impl BitOrAssign for m256i {
  #[inline(always)]
  fn bitor_assign(&mut self, rhs: Self) {
      *self = *self | rhs;
  }
}

impl BitXor for m256i {
  type Output = Self;
  #[must_use]
  #[inline(always)]
  fn bitxor(self, rhs: Self) -> Self {
    let rhs = cast_from_m256i_to_m256(rhs);
    let result = xor_m256(cast_from_m256i_to_m256(self), rhs);
    cast_from_m256_to_m256i(result)
  }
}
impl BitXorAssign for m256i {
  #[inline(always)]
  fn bitxor_assign(&mut self, rhs: Self) {
    *self = *self ^ rhs;
  }
}
*/

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

/// Loads the reference given and zeroes any `i32` lanes not in the mask.
///
/// * A lane is "in" the mask if that lane's mask value is set in the high bit
///   (aka "if the lane's value is negative").
/// ```
/// # use safe_arch::*;
/// let a = splat_i32_m128i(5);
/// let b = load_masked_i32_m128i(&a, m128i::from([-1_i32, 0, 0, -1]));
/// assert_eq!(<[i32; 4]>::from(b), [5, 0, 0, 5]);
/// ```
/// * **Intrinsic:** [`_mm_maskload_epi32`]
/// * **Assembly:** `vpmaskmovd xmm, xmm, m128`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn load_masked_i32_m128i(a: &m128i, mask: m128i) -> m128i {
  m128i(unsafe { _mm_maskload_epi32(a as *const m128i as *const i32, mask.0) })
}

/// Loads the reference given and zeroes any `i64` lanes not in the mask.
///
/// * A lane is "in" the mask if that lane's mask value is set in the high bit
///   (aka "if the lane's value is negative").
/// ```
/// # use safe_arch::*;
/// let a = splat_i64_m128i(5);
/// let b = load_masked_i64_m128i(&a, m128i::from([0_i64, -1]));
/// assert_eq!(<[i64; 2]>::from(b), [0_i64, 5]);
/// ```
/// * **Intrinsic:** [`_mm_maskload_epi64`]
/// * **Assembly:** `vpmaskmovq xmm, xmm, m128`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn load_masked_i64_m128i(a: &m128i, mask: m128i) -> m128i {
  m128i(unsafe { _mm_maskload_epi64(a as *const m128i as *const i64, mask.0) })
}

/// Stores the `i32` masked lanes given to the reference.
///
/// * A lane is "in" the mask if that lane's mask value is set in the high bit
///   (aka "if the lane's value is negative").
/// * Lanes not in the mask are not modified.
/// ```
/// # use safe_arch::*;
/// let mut a = m128i::default();
/// store_masked_i32_m128i(
///   &mut a,
///   m128i::from([-1_i32, 0, 0, -1]),
///   splat_i32_m128i(5),
/// );
/// assert_eq!(<[i32; 4]>::from(a), [5, 0, 0, 5]);
/// ```
/// * **Intrinsic:** [`_mm_maskstore_epi32`]
/// * **Assembly:** `vpmaskmovd m128, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn store_masked_i32_m128i(addr: &mut m128i, mask: m128i, a: m128i) {
  unsafe { _mm_maskstore_epi32(addr as *mut m128i as *mut i32, mask.0, a.0) };
}

/// Stores the `i32` masked lanes given to the reference.
///
/// * A lane is "in" the mask if that lane's mask value is set in the high bit
///   (aka "if the lane's value is negative").
/// * Lanes not in the mask are not modified.
/// ```
/// # use safe_arch::*;
/// let mut a = m128i::default();
/// store_masked_i64_m128i(&mut a, m128i::from([0_i64, -1]), splat_i64_m128i(5));
/// assert_eq!(<[i64; 2]>::from(a), [0, 5]);
/// ```
/// * **Intrinsic:** [`_mm_maskstore_epi64`]
/// * **Assembly:** `vpmaskmovq m128, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn store_masked_i64_m128i(addr: &mut m128i, mask: m128i, a: m128i) {
  unsafe { _mm_maskstore_epi64(addr as *mut m128i as *mut i64, mask.0, a.0) };
}

/// Shift `u32` values to the left by `count` bits.
///
/// * Each `u32` lane in `a` is shifted by the same indexed `u32` lane in
///   `count`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let count = m128i::from([5, 6, 7, 8]);
/// let out: [u32; 4] = shl_u32_each_m128i(a, count).into();
/// assert_eq!(out, [1 << 5, 2 << 6, 3 << 7, 4 << 8]);
/// ```
/// * **Intrinsic:** [`_mm_sllv_epi32`]
/// * **Assembly:** `vpsllvd xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn shl_u32_each_m128i(a: m128i, count: m128i) -> m128i {
  m128i(unsafe { _mm_sllv_epi32(a.0, count.0) })
}

/// Shift `u64` values to the left by `count` bits.
///
/// * Each `u64` lane in `a` is shifted by the same indexed `u64` lane in
///   `count`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_u64, 2]);
/// let count = m128i::from([3_u64, 4]);
/// let out: [u64; 2] = shl_u64_each_m128i(a, count).into();
/// assert_eq!(out, [1_u64 << 3, 2 << 4]);
/// ```
/// * **Intrinsic:** [`_mm_sllv_epi64`]
/// * **Assembly:** `vpsllvq xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn shl_u64_each_m128i(a: m128i, count: m128i) -> m128i {
  m128i(unsafe { _mm_sllv_epi64(a.0, count.0) })
}

/// Shift `i32` values to the right by `count` bits.
///
/// * Each `i32` lane in `a` is shifted by the same indexed `u32` lane in
///   `count`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([100, 110, 120, -130]);
/// let count = m128i::from([1, 2, 3, 4]);
/// let out: [i32; 4] = shr_i32_each_m128i(a, count).into();
/// assert_eq!(out, [100 >> 1, 110 >> 2, 120 >> 3, (-130) >> 4]);
/// ```
/// * **Intrinsic:** [`_mm_srav_epi32`]
/// * **Assembly:** `vpsravd xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn shr_i32_each_m128i(a: m128i, count: m128i) -> m128i {
  m128i(unsafe { _mm_srav_epi32(a.0, count.0) })
}

/// Shift `u32` values to the left by `count` bits.
///
/// * Each `u32` lane in `a` is shifted by the same indexed `u32` lane in
///   `count`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([100, 110, 120, 130]);
/// let count = m128i::from([1, 2, 3, 4]);
/// let out: [u32; 4] = shr_u32_each_m128i(a, count).into();
/// assert_eq!(out, [100 >> 1, 110 >> 2, 120 >> 3, 130 >> 4]);
/// ```
/// * **Intrinsic:** [`_mm_srlv_epi32`]
/// * **Assembly:** `vpsrlvd xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn shr_u32_each_m128i(a: m128i, count: m128i) -> m128i {
  m128i(unsafe { _mm_srlv_epi32(a.0, count.0) })
}

/// Shift `u64` values to the left by `count` bits.
///
/// * Each `u64` lane in `a` is shifted by the same indexed `u64` lane in
///   `count`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([100_u64, 110]);
/// let count = m128i::from([1_u64, 2]);
/// let out: [u64; 2] = shr_u64_each_m128i(a, count).into();
/// assert_eq!(out, [100_u64 >> 1, 110 >> 2]);
/// ```
/// * **Intrinsic:** [`_mm_srlv_epi64`]
/// * **Assembly:** `vpsrlvq xmm, xmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn shr_u64_each_m128i(a: m128i, count: m128i) -> m128i {
  m128i(unsafe { _mm_srlv_epi64(a.0, count.0) })
}

/// Absolute value of `i8` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([-7_i8; 32]);
/// let b: [i8; 32] = abs_i8_m256i(a).into();
/// assert_eq!(b, [7_i8; 32]);
/// ```
/// * **Intrinsic:** [`_mm256_abs_epi8`]
/// * **Assembly:** `vpabsb ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn abs_i8_m256i(a: m256i) -> m256i {
  m256i(unsafe { _mm256_abs_epi8(a.0) })
}

/// Absolute value of `i16` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([-7_i16; 16]);
/// let b: [i16; 16] = abs_i16_m256i(a).into();
/// assert_eq!(b, [7_i16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_abs_epi16`]
/// * **Assembly:** `vpabsw ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn abs_i16_m256i(a: m256i) -> m256i {
  m256i(unsafe { _mm256_abs_epi16(a.0) })
}

/// Absolute value of `i32` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([-7_i32; 8]);
/// let b: [i32; 8] = abs_i32_m256i(a).into();
/// assert_eq!(b, [7_i32; 8]);
/// ```
/// * **Intrinsic:** [`_mm256_abs_epi32`]
/// * **Assembly:** `vpabsd ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn abs_i32_m256i(a: m256i) -> m256i {
  m256i(unsafe { _mm256_abs_epi32(a.0) })
}

/// Lanewise `a + b` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i8; 32]);
/// let b = m256i::from([10_i8; 32]);
/// let c: [i8; 32] = add_i8_m256i(a, b).into();
/// assert_eq!(c, [15_i8; 32]);
/// ```
/// * **Intrinsic:** [`_mm256_add_epi8`]
/// * **Assembly:** `vpaddb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn add_i8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_add_epi8(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `i16`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i16; 16]);
/// let b = m256i::from([10_i16; 16]);
/// let c: [i16; 16] = add_i16_m256i(a, b).into();
/// assert_eq!(c, [15_i16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_add_epi16`]
/// * **Assembly:** `vpaddw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn add_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_add_epi16(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `i32`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i32; 8]);
/// let b = m256i::from([10_i32; 8]);
/// let c: [i32; 8] = add_i32_m256i(a, b).into();
/// assert_eq!(c, [15_i32; 8]);
/// ```
/// * **Intrinsic:** [`_mm256_add_epi32`]
/// * **Assembly:** `vpaddd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn add_i32_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_add_epi32(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `i64`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i64; 4]);
/// let b = m256i::from([10_i64; 4]);
/// let c: [i64; 4] = add_i64_m256i(a, b).into();
/// assert_eq!(c, [15_i64; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_add_epi64`]
/// * **Assembly:** `vpaddq ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn add_i64_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_add_epi64(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([126_i8; 32]);
/// let b = m256i::from([125_i8; 32]);
/// let c: [i8; 32] = add_saturating_i8_m256i(a, b).into();
/// assert_eq!(c, [127_i8; 32]);
/// ```
/// * **Intrinsic:** [`_mm256_adds_epi8`]
/// * **Assembly:** `vpaddsb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn add_saturating_i8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_adds_epi8(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `i16`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([32700_i16; 16]);
/// let b = m256i::from([32000_i16; 16]);
/// let c: [i16; 16] = add_saturating_i16_m256i(a, b).into();
/// assert_eq!(c, [32767_i16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_adds_epi16`]
/// * **Assembly:** `vpaddsw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn add_saturating_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_adds_epi16(a.0, b.0) })
}

/// Works like [`combined_byte_shr_imm_m128i`], but twice as wide.
///
/// The low half of the bytes and high half of the bytes are both processed
/// separately.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i8; 32]);
/// let b = m256i::from([12_i8; 32]);
/// // `a` bytes come in to the _high_ indexes because these are LE bytes.
/// // Also note that the three 5 values at the low half and high half.
/// let c: [i8; 32] = combined_byte_shr_imm_m256i!(a, b, 3).into();
/// assert_eq!(
///   c,
///   [
///     12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 5, 5, 5,
///     12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 5, 5, 5_i8
///   ]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_alignr_epi8`]
/// * **Assembly:** `vpalignr ymm, ymm, ymm, imm8`
#[macro_export]
#[rustfmt::skip]
macro_rules! combined_byte_shr_imm_m256i {
  ($a:expr, $b:expr, $imm:expr) => {{
    let a: $crate::m256i = $a;
    let b: $crate::m256i = $b;
    const IMM: ::core::primitive::i32 = $imm as ::core::primitive::i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm256_alignr_epi8;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_alignr_epi8;
    $crate::m256i(unsafe { _mm256_alignr_epi8(a.0, b.0, IMM) })
  }};
}

/// Bitwise `a & b`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i64, 0, 1, 1]);
/// let b = m256i::from([0_i64, 1, 0, 1]);
/// let c: [i64; 4] = and_m256i(a, b).into();
/// assert_eq!(c, [0_i64, 0, 0, 1]);
/// ```
/// * **Intrinsic:** [`_mm256_and_si256`]
/// * **Assembly:** `vpand ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn and_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_and_si256(a.0, b.0) })
}

/// Bitwise `(!a) & b`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0_i64, 0, 1, 1]);
/// let b = m256i::from([0_i64, 1, 0, 1]);
/// let c: [i64; 4] = andnot_m256i(a, b).into();
/// assert_eq!(c, [0_i64, 1, 0, 0]);
/// ```
/// * **Intrinsic:** [`_mm256_andnot_si256`]
/// * **Assembly:** `vpandn ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn andnot_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_andnot_si256(a.0, b.0) })
}

/// Average `u8` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([100_u8; 32]);
/// let b = m256i::from([120_u8; 32]);
/// let c: [u8; 32] = average_u8_m256i(a, b).into();
/// assert_eq!(c, [110_u8; 32]);
/// ```
/// * **Intrinsic:** [`_mm256_avg_epu8`]
/// * **Assembly:** `vpavgb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn average_u8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_avg_epu8(a.0, b.0) })
}

/// Average `u16` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([100_u16; 16]);
/// let b = m256i::from([120_u16; 16]);
/// let c: [u16; 16] = average_u16_m256i(a, b).into();
/// assert_eq!(c, [110_u16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_avg_epu16`]
/// * **Assembly:** `vpavgw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn average_u16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_avg_epu16(a.0, b.0) })
}

/// Blends the `i16` lanes according to the immediate value.
///
/// * The low 8 lanes and high 8 lanes both use the same immediate.
/// * Each bit in `0..=7` should be set for `$b` and unset for `$a` within that
///   half of the `i16` values.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i16; 16]);
/// let b = m256i::from([10_i16; 16]);
/// //
/// let c: [i16; 16] = blend_imm_i16_m256i!(a, b, 0b11001000).into();
/// assert_eq!(c, [5_i16, 5, 5, 10, 5, 5, 10, 10, 5, 5, 5, 10, 5, 5, 10, 10]);
/// ```
/// * **Intrinsic:** [`_mm256_blend_epi16`]
/// * **Assembly:** `vpblendw ymm, ymm, ymm, imm8`
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
macro_rules! blend_imm_i16_m256i {
  ($a:expr, $b:expr, $imm:expr) => {{
    let a: $crate::m256i = $a;
    let b: $crate::m256i = $b;
    const IMM: ::core::primitive::i32 = $imm as ::core::primitive::i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm256_blend_epi16;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_blend_epi16;
    $crate::m256i(unsafe { _mm256_blend_epi16(a.0, b.0, IMM) })
  }};
}

/// Blends the `i32` lanes according to the immediate value.
///
/// * Each bit in `0..=7` should be set for `$b` and unset for `$a`
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i32; 8]);
/// let b = m256i::from([10_i32; 8]);
/// //
/// let c: [i32; 8] = blend_imm_i32_m256i!(a, b, 0b11001000).into();
/// assert_eq!(c, [5, 5, 5, 10, 5, 5, 10, 10]);
/// ```
/// * **Intrinsic:** [`_mm256_blend_epi32`]
/// * **Assembly:** `vpblendd ymm, ymm, ymm, imm8`
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
macro_rules! blend_imm_i32_m256i {
  ($a:expr, $b:expr, $imm:expr) => {{
    let a: $crate::m256i = $a;
    let b: $crate::m256i = $b;
    const IMM: ::core::primitive::i32 = $imm as ::core::primitive::i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm256_blend_epi32;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_blend_epi32;
    $crate::m256i(unsafe { _mm256_blend_epi32(a.0, b.0, IMM) })
  }};
}

/// Blend `i8` lanes according to a runtime varying mask.
///
/// * Mask lanes should be non-negative for `a` and negative for `b`.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i8; 32]);
/// let b = m256i::from([10_i8; 32]);
/// let mask = m256i::from([
///   0_i8, 0, 0, -1, -1, -1, 0, 0, 0, -1, -1, -1, 0, 0, 0, -1, -1, -1, 0, 0, 0,
///   -1, -1, -1, 0, 0, 0, -1, -1, -1, 0, 0,
/// ]);
/// let c: [i8; 32] = blend_varying_i8_m256i(a, b, mask).into();
/// assert_eq!(
///   c,
///   [
///     5, 5, 5, 10, 10, 10, 5, 5, 5, 10, 10, 10, 5, 5, 5, 10, 10, 10, 5, 5, 5,
///     10, 10, 10, 5, 5, 5, 10, 10, 10, 5, 5
///   ]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_avg_epu16`]
/// * **Assembly:** `vpavgw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn blend_varying_i8_m256i(a: m256i, b: m256i, mask: m256i) -> m256i {
  m256i(unsafe { _mm256_blendv_epi8(a.0, b.0, mask.0) })
}

/// Sets the lowest `i8` lane of an `m128i` as all lanes of an `m256i`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(5_i8 as i128);
/// let b: [i8; 32] = set_splat_i8_m128i_s_m256i(a).into();
/// assert_eq!(b, [5_i8; 32]);
/// ```
/// * **Intrinsic:** [`_mm256_broadcastb_epi8`]
/// * **Assembly:** `vpbroadcastb ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn set_splat_i8_m128i_s_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_broadcastb_epi8(a.0) })
}

/// Sets the lowest `i16` lane of an `m128i` as all lanes of an `m256i`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(5_i16 as i128);
/// let b: [i16; 16] = set_splat_i16_m128i_s_m256i(a).into();
/// assert_eq!(b, [5_i16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_broadcastw_epi16`]
/// * **Assembly:** `vpbroadcastw ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn set_splat_i16_m128i_s_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_broadcastw_epi16(a.0) })
}

/// Sets the lowest `i32` lane of an `m128i` as all lanes of an `m256i`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(5_i32 as i128);
/// let b: [i32; 8] = set_splat_i32_m128i_s_m256i(a).into();
/// assert_eq!(b, [5_i32; 8]);
/// ```
/// * **Intrinsic:** [`_mm256_broadcastd_epi32`]
/// * **Assembly:** `vpbroadcastd ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn set_splat_i32_m128i_s_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_broadcastd_epi32(a.0) })
}

/// Sets the lowest `i64` lane of an `m128i` as all lanes of an `m256i`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(5_i64 as i128);
/// let b: [i64; 4] = set_splat_i64_m128i_s_m256i(a).into();
/// assert_eq!(b, [5_i64; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_broadcastq_epi64`]
/// * **Assembly:** `vpbroadcastq ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn set_splat_i64_m128i_s_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_broadcastq_epi64(a.0) })
}

/// Sets the lowest lane of an `m128d` as all lanes of an `m256d`.
/// ```
/// # use safe_arch::*;
/// let a = set_m128d_s(5.0);
/// let b = set_splat_m128d_s_m256d(a).to_array();
/// assert_eq!(b, [5.0; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_broadcastsd_pd`]
/// * **Assembly:** `vbroadcastsd ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn set_splat_m128d_s_m256d(a: m128d) -> m256d {
  m256d(unsafe { _mm256_broadcastsd_pd(a.0) })
}

/// Sets the lowest lane of an `m128` as all lanes of an `m256`.
/// ```
/// # use safe_arch::*;
/// let a = set_m128_s(5.0);
/// let b = set_splat_m128_s_m256(a).to_array();
/// assert_eq!(b, [5.0; 8]);
/// ```
/// * **Intrinsic:** [`_mm256_broadcastss_ps`]
/// * **Assembly:** `vbroadcastss ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn set_splat_m128_s_m256(a: m128) -> m256 {
  m256(unsafe { _mm256_broadcastss_ps(a.0) })
}

/// Shifts each `u128` lane left by a number of **bytes**.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0x0000000B_0000000A_0000000F_11111111_u128; 2]);
/// //
/// let b: [u128; 2] = byte_shl_u128_imm_m256i!(a, 1).into();
/// assert_eq!(b, [0x00000B00_00000A00_00000F11_11111100_u128; 2]);
/// ```
/// * **Intrinsic:** [`_mm256_bslli_epi128`]
/// * **Assembly:** `vpslldq ymm, ymm, imm8`
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
macro_rules! byte_shl_u128_imm_m256i {
  ($a:expr, $imm:expr) => {{
    let a: $crate::m256i = $a;
    const IMM: ::core::primitive::i32 = $imm as ::core::primitive::i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm256_bslli_epi128;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_bslli_epi128;
    $crate::m256i(unsafe { _mm256_bslli_epi128(a.0, IMM) })
  }};
}

/// Shifts each `u128` lane right by a number of **bytes**.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([0x0000000B_0000000A_0000000F_11111111_u128; 2]);
/// //
/// let b: [u128; 2] = byte_shr_u128_imm_m256i!(a, 1).into();
/// assert_eq!(b, [0x00000000_0B000000_0A000000_0F111111; 2]);
/// ```
/// * **Intrinsic:** [`_mm256_bsrli_epi128`]
/// * **Assembly:** `vpsrldq ymm, ymm, imm8`
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
macro_rules! byte_shr_u128_imm_m256i {
  ($a:expr, $imm:expr) => {{
    let a: $crate::m256i = $a;
    const IMM: ::core::primitive::i32 = $imm as ::core::primitive::i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm256_bsrli_epi128;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_bsrli_epi128;
    $crate::m256i(unsafe { _mm256_bsrli_epi128(a.0, IMM) })
  }};
}

/// Compare `i8` lanes for equality, mask output.
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   <[i8; 32]>::from(cmp_eq_mask_i8_m256i(
///     m256i::from([1_i8; 32]),
///     m256i::from([1_i8; 32])
///   )),
///   [-1_i8; 32]
/// );
/// assert_eq!(
///   <[i8; 32]>::from(cmp_eq_mask_i8_m256i(
///     m256i::from([5_i8; 32]),
///     m256i::from([6_i8; 32])
///   )),
///   [0_i8; 32]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_cmpeq_epi8`]
/// * **Assembly:** `vpcmpeqb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn cmp_eq_mask_i8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_cmpeq_epi8(a.0, b.0) })
}

/// Compare `i16` lanes for equality, mask output.
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   <[i16; 16]>::from(cmp_eq_mask_i16_m256i(
///     m256i::from([1_i16; 16]),
///     m256i::from([1_i16; 16])
///   )),
///   [-1_i16; 16]
/// );
/// assert_eq!(
///   <[i16; 16]>::from(cmp_eq_mask_i16_m256i(
///     m256i::from([5_i16; 16]),
///     m256i::from([6_i16; 16])
///   )),
///   [0_i16; 16]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_cmpeq_epi16`]
/// * **Assembly:** `vpcmpeqw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn cmp_eq_mask_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_cmpeq_epi16(a.0, b.0) })
}

/// Compare `i32` lanes for equality, mask output.
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   <[i32; 8]>::from(cmp_eq_mask_i32_m256i(
///     m256i::from([1_i32; 8]),
///     m256i::from([1_i32; 8])
///   )),
///   [-1_i32; 8]
/// );
/// assert_eq!(
///   <[i32; 8]>::from(cmp_eq_mask_i32_m256i(
///     m256i::from([5_i32; 8]),
///     m256i::from([6_i32; 8])
///   )),
///   [0_i32; 8]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_cmpeq_epi32`]
/// * **Assembly:** `vpcmpeqd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn cmp_eq_mask_i32_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_cmpeq_epi32(a.0, b.0) })
}

/// Compare `i64` lanes for equality, mask output.
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   <[i64; 4]>::from(cmp_eq_mask_i64_m256i(
///     m256i::from([1_i64; 4]),
///     m256i::from([1_i64; 4])
///   )),
///   [-1_i64; 4]
/// );
/// assert_eq!(
///   <[i64; 4]>::from(cmp_eq_mask_i64_m256i(
///     m256i::from([5_i64; 4]),
///     m256i::from([6_i64; 4])
///   )),
///   [0_i64; 4]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_cmpeq_epi64`]
/// * **Assembly:** `vpcmpeqq ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn cmp_eq_mask_i64_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_cmpeq_epi64(a.0, b.0) })
}

/// Compare `i8` lanes for `a > b`, mask output.
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   <[i8; 32]>::from(cmp_gt_mask_i8_m256i(
///     m256i::from([1_i8; 32]),
///     m256i::from([0_i8; 32])
///   )),
///   [-1_i8; 32]
/// );
/// assert_eq!(
///   <[i8; 32]>::from(cmp_gt_mask_i8_m256i(
///     m256i::from([5_i8; 32]),
///     m256i::from([5_i8; 32])
///   )),
///   [0_i8; 32]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_cmpgt_epi8`]
/// * **Assembly:** `vpcmpgtb ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn cmp_gt_mask_i8_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_cmpgt_epi8(a.0, b.0) })
}

/// Compare `i16` lanes for `a > b`, mask output.
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   <[i16; 16]>::from(cmp_gt_mask_i16_m256i(
///     m256i::from([1_i16; 16]),
///     m256i::from([0_i16; 16])
///   )),
///   [-1_i16; 16]
/// );
/// assert_eq!(
///   <[i16; 16]>::from(cmp_gt_mask_i16_m256i(
///     m256i::from([5_i16; 16]),
///     m256i::from([5_i16; 16])
///   )),
///   [0_i16; 16]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_cmpgt_epi16`]
/// * **Assembly:** `vpcmpgtw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn cmp_gt_mask_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_cmpgt_epi16(a.0, b.0) })
}

/// Compare `i32` lanes for `a > b`, mask output.
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   <[i32; 8]>::from(cmp_gt_mask_i32_m256i(
///     m256i::from([1_i32; 8]),
///     m256i::from([0_i32; 8])
///   )),
///   [-1_i32; 8]
/// );
/// assert_eq!(
///   <[i32; 8]>::from(cmp_gt_mask_i32_m256i(
///     m256i::from([5_i32; 8]),
///     m256i::from([5_i32; 8])
///   )),
///   [0_i32; 8]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_cmpgt_epi32`]
/// * **Assembly:** `vpcmpgtd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn cmp_gt_mask_i32_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_cmpgt_epi32(a.0, b.0) })
}

/// Compare `i64` lanes for `a > b`, mask output.
/// ```
/// # use safe_arch::*;
/// assert_eq!(
///   <[i64; 4]>::from(cmp_gt_mask_i64_m256i(
///     m256i::from([1_i64; 4]),
///     m256i::from([0_i64; 4])
///   )),
///   [-1_i64; 4]
/// );
/// assert_eq!(
///   <[i64; 4]>::from(cmp_gt_mask_i64_m256i(
///     m256i::from([5_i64; 4]),
///     m256i::from([5_i64; 4])
///   )),
///   [0_i64; 4]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_cmpgt_epi64`]
/// * **Assembly:** `vpcmpgtq ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn cmp_gt_mask_i64_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_cmpgt_epi64(a.0, b.0) })
}

/// Sign extend `i16` values to `i32` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([-5_i16; 8]);
/// let b: [i32; 8] = convert_i16_m128i_m256i(a).into();
/// assert_eq!(b, [-5_i32; 8]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepi16_epi32`]
/// * **Assembly:** `vpmovsxwd ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn convert_i16_m128i_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepi16_epi32(a.0) })
}

/// Sign extend `i16` values to `i64` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([-5_i16; 8]);
/// let b: [i64; 4] = convert_i16_m128i_lower4_m256i(a).into();
/// assert_eq!(b, [-5_i64; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepi16_epi64`]
/// * **Assembly:** `vpmovsxwq ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn convert_i16_m128i_lower4_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepi16_epi64(a.0) })
}

/// Sign extend `i32` values to `i64` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([-5_i32; 4]);
/// let b: [i64; 4] = convert_i32_m128i_m256i(a).into();
/// assert_eq!(b, [-5_i64; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepi32_epi64`]
/// * **Assembly:** `vpmovsxdq ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn convert_i32_m128i_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepi32_epi64(a.0) })
}

/// Sign extend `i8` values to `i16` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([-5_i8; 16]);
/// let b: [i16; 16] = convert_i8_m128i_m256i(a).into();
/// assert_eq!(b, [-5_i16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepi8_epi16`]
/// * **Assembly:** `vpmovsxbw ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn convert_i8_m128i_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepi8_epi16(a.0) })
}

/// Sign extend the lower 8 `i8` values to `i32` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([-5_i8; 16]);
/// let b: [i32; 8] = convert_i8_m128i_lower8_m256i(a).into();
/// assert_eq!(b, [-5_i32; 8]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepi8_epi32`]
/// * **Assembly:** `vpmovsxbd ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn convert_i8_m128i_lower8_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepi8_epi32(a.0) })
}

/// Sign extend the lower 4 `i8` values to `i64` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([-5_i8; 16]);
/// let b: [i64; 4] = convert_i8_m128i_lower4_m256i(a).into();
/// assert_eq!(b, [-5_i64; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepi8_epi64`]
/// * **Assembly:** `vpmovsxbq ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn convert_i8_m128i_lower4_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepi8_epi64(a.0) })
}

/// Zero extend `u16` values to `i32` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5_u16; 8]);
/// let b: [i32; 8] = convert_u16_m128i_m256i(a).into();
/// assert_eq!(b, [5_i32; 8]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepu16_epi32`]
/// * **Assembly:** `vpmovzxwd ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn convert_u16_m128i_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepu16_epi32(a.0) })
}

/// Zero extend lower 4 `u16` values to `i64` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5_u16; 8]);
/// let b: [i64; 4] = convert_u16_m128i_lower4_m256i(a).into();
/// assert_eq!(b, [5_i64; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepu16_epi64`]
/// * **Assembly:** `vpmovzxwq ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn convert_u16_m128i_lower4_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepu16_epi64(a.0) })
}

/// Zero extend `u32` values to `i64` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5_u32; 4]);
/// let b: [i64; 4] = convert_u32_m128i_m256i(a).into();
/// assert_eq!(b, [5_i64; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepu32_epi64`]
/// * **Assembly:** `vpmovzxdq ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn convert_u32_m128i_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepu32_epi64(a.0) })
}

/// Zero extend `u8` values to `i16` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5_u8; 16]);
/// let b: [i16; 16] = convert_u8_m128i_m256i(a).into();
/// assert_eq!(b, [5_i16; 16]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepu8_epi16`]
/// * **Assembly:** `vpmovzxbw ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn convert_u8_m128i_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepu8_epi16(a.0) })
}

/// Zero extend lower 8 `u8` values to `i16` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5_u8; 16]);
/// let b: [i32; 8] = convert_u8_m128i_lower8_m256i(a).into();
/// assert_eq!(b, [5_i32; 8]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepu8_epi32`]
/// * **Assembly:** `vpmovzxbd ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn convert_u8_m128i_lower8_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepu8_epi32(a.0) })
}

/// Zero extend lower 4 `u8` values to `i16` values.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([5_u8; 16]);
/// let b: [i64; 4] = convert_u8_m128i_lower4_m256i(a).into();
/// assert_eq!(b, [5_i64; 4]);
/// ```
/// * **Intrinsic:** [`_mm256_cvtepu8_epi64`]
/// * **Assembly:** `vpmovzxbq ymm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn convert_u8_m128i_lower4_m256i(a: m128i) -> m256i {
  m256i(unsafe { _mm256_cvtepu8_epi64(a.0) })
}

/// Gets an `i16` value out of an `m256i`, returns as `i32`.
///
/// The lane to get must be a constant. If you select outside the range `0..16`
/// then the selection is wrapped to be in range (only the lowest 4 bits of the
/// input are used).
///
/// ```
/// # use safe_arch::*;
/// let a =
///   m256i::from([0xA_i16, 0xB, 0xC, 0xD, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// //
/// assert_eq!(extract_i16_as_i32_m256i!(a, 0), 0xA);
/// assert_eq!(extract_i16_as_i32_m256i!(a, 1), 0xB);
/// // the lane requested is "wrapped" to be a valid index.
/// assert_eq!(0b1_0010 & 0b1111, 2);
/// assert_eq!(extract_i16_as_i32_m256i!(a, 0b1_0010), 0xC);
/// ```
/// * **Intrinsic:** [`_mm256_extract_epi16`]
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
macro_rules! extract_i16_as_i32_m256i {
  ($a:expr, $imm:expr) => {{
    let a: $crate::m256i = $a;
    const LANE: ::core::primitive::i32 =
      ($imm & 0b1111) as ::core::primitive::i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm256_extract_epi16;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_extract_epi16;
    unsafe { _mm256_extract_epi16(a.0, LANE) }
  }};
}

/// Gets an `i8` value out of an `m256i`, returns as `i32`.
///
/// The lane to get must be a constant. If you select outside the range `0..32`
/// then the selection is wrapped to be in range (only the lowest 5 bits of the
/// input are used).
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([
///   0xA_i8, 0xB, 0xC, 0xD, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
///   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
/// ]);
/// //
/// assert_eq!(extract_i8_as_i32_m256i!(a, 0), 0xA);
/// assert_eq!(extract_i8_as_i32_m256i!(a, 1), 0xB);
/// // the lane requested is "wrapped" to be a valid index.
/// assert_eq!(0b10_0010 & 0b1_1111, 2);
/// assert_eq!(extract_i8_as_i32_m256i!(a, 0b10_0010), 0xC);
/// ```
/// * **Intrinsic:** [`_mm256_extract_epi8`]
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
macro_rules! extract_i8_as_i32_m256i {
  ($a:expr, $imm:expr) => {{
    let a: $crate::m256i = $a;
    const LANE: ::core::primitive::i32 =
      ($imm & 0b11111) as ::core::primitive::i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm256_extract_epi8;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_extract_epi8;
    unsafe { _mm256_extract_epi8(a.0, LANE) }
  }};
}

/// Gets an `m128i` value out of an `m256i`.
///
/// The lane to get must be a constant. Only the lowest bit of the value is
/// used.
///
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_u128, 6_u128]);
/// //
/// assert_eq!(extract_m128i_m256i!(a, 0), m128i::from(5_u128));
/// assert_eq!(extract_m128i_m256i!(a, 1), m128i::from(6_u128));
/// // the index is "wrapped" to be in bounds.
/// assert_eq!(extract_m128i_m256i!(a, 2), m128i::from(5_u128));
/// ```
/// * **Intrinsic:** [`_mm256_extract_epi8`]
/// * **Assembly:** `vextracti128 xmm, ymm, imm8`
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
macro_rules! extract_m128i_m256i {
  ($a:expr, $imm:expr) => {{
    let a: $crate::m256i = $a;
    const LANE: ::core::primitive::i32 = ($imm & 0b1) as ::core::primitive::i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm256_extracti128_si256;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm256_extracti128_si256;
    $crate::m128i(unsafe { _mm256_extracti128_si256(a.0, LANE) })
  }};
}

/// Horizontal `a + b` with lanes as `i16`.
///
/// * The results are interleaved 128-bits at a time: a.low, b.low, a.high,
///   b.high
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i16; 16]);
/// let b = m256i::from([6_i16; 16]);
/// let c: [i16; 16] = add_horizontal_i16_m256i(a, b).into();
/// assert_eq!(
///   c,
///   [10_i16, 10, 10, 10, 12, 12, 12, 12, 10, 10, 10, 10, 12, 12, 12, 12]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_hadd_epi16`]
/// * **Assembly:** `vphaddw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn add_horizontal_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_hadd_epi16(a.0, b.0) })
}

/// Horizontal saturating `a + b` with lanes as `i16`.
///
/// * The results are interleaved 128-bits at a time: a.low, b.low, a.high,
///   b.high
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([i16::MAX; 16]);
/// let b = m256i::from([i16::MIN; 16]);
/// let c: [i16; 16] = add_horizontal_saturating_i16_m256i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     i16::MAX, i16::MAX, i16::MAX, i16::MAX,
///     i16::MIN, i16::MIN, i16::MIN, i16::MIN,
///     i16::MAX, i16::MAX, i16::MAX, i16::MAX,
///     i16::MIN, i16::MIN, i16::MIN, i16::MIN,
///   ]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_hadds_epi16`]
/// * **Assembly:** `vphaddsw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
#[rustfmt::skip]
pub fn add_horizontal_saturating_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_hadds_epi16(a.0, b.0) })
}

/// Horizontal `a + b` with lanes as `i32`.
///
/// * The results are interleaved 128-bits at a time: a.low, b.low, a.high,
///   b.high
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_i32; 8]);
/// let b = m256i::from([6_i32; 8]);
/// let c: [i32; 8] = add_horizontal_i32_m256i(a, b).into();
/// assert_eq!(c, [10, 10, 12, 12, 10, 10, 12, 12]);
/// ```
/// * **Intrinsic:** [`_mm256_hadd_epi32`]
/// * **Assembly:** `vphaddd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn add_horizontal_i32_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_hadd_epi32(a.0, b.0) })
}

/// Horizontal `a - b` with lanes as `i16`.
///
/// * The results are interleaved 128-bits at a time: a.low, b.low, a.high,
///   b.high
/// ```
/// # use safe_arch::*;
/// let a =
///   m256i::from([5_i16, 6, 2, 5, 4, 3, 1, 0, -12, 13, 56, 21, 8, 7, 6, 5]);
/// let b = m256i::from([
///   12000_i16, 13000, -2, -8, 0, 1, 2, 3, 8, 7, 6, 5, 234, 654, 123, 978,
/// ]);
/// let c: [i16; 16] = add_horizontal_i16_m256i(a, b).into();
/// assert_eq!(
///   c,
///   [11_i16, 7, 7, 1, 25000, -10, 1, 5, 1, 77, 15, 11, 15, 11, 888, 1101]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_hsub_epi16`]
/// * **Assembly:** `vphsubw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn sub_horizontal_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_hsub_epi16(a.0, b.0) })
}

/// Horizontal `a - b` with lanes as `i32`.
///
/// * The results are interleaved 128-bits at a time: a.low, b.low, a.high,
///   b.high
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5, 6, 2, 5, 4, 3, 1, 0]);
/// let b = m256i::from([-12, 13, 56, 21, 8, 7, 6, 5]);
/// let c: [i32; 8] = sub_horizontal_i32_m256i(a, b).into();
/// assert_eq!(c, [-1, -3, -25, 35, 1, 1, 1, 1]);
/// ```
/// * **Intrinsic:** [`_mm256_hsub_epi32`]
/// * **Assembly:** `vphsubd ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
pub fn sub_horizontal_i32_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_hsub_epi32(a.0, b.0) })
}

/// Horizontal saturating `a - b` with lanes as `i16`.
///
/// * The results are interleaved 128-bits at a time: a.low, b.low, a.high,
///   b.high
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([i16::MAX; 16]);
/// let b = m256i::from([i16::MIN; 16]);
/// let c: [i16; 16] = add_horizontal_saturating_i16_m256i(a, b).into();
/// assert_eq!(
///   c,
///   [
///     i16::MAX, i16::MAX, i16::MAX, i16::MAX,
///     i16::MIN, i16::MIN, i16::MIN, i16::MIN,
///     i16::MAX, i16::MAX, i16::MAX, i16::MAX,
///     i16::MIN, i16::MIN, i16::MIN, i16::MIN,
///   ]
/// );
/// ```
/// * **Intrinsic:** [`_mm256_hsubs_epi16`]
/// * **Assembly:** `vphsubsw ymm, ymm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "avx2")))]
#[rustfmt::skip]
pub fn sub_horizontal_saturating_i16_m256i(a: m256i, b: m256i) -> m256i {
  m256i(unsafe { _mm256_hsubs_epi16(a.0, b.0) })
}

// _mm256_inserti128_si256

// _mm256_madd_epi16

// _mm256_maddubs_epi16

// _mm256_maskload_epi32
// _mm256_maskload_epi64

// _mm256_maskstore_epi32
// _mm256_maskstore_epi64

// _mm256_max_epi8
// _mm256_max_epi16
// _mm256_max_epi32
// _mm256_max_epu8
// _mm256_max_epu16
// _mm256_max_epu32

// _mm256_min_epi8
// _mm256_min_epi16
// _mm256_min_epi32
// _mm256_min_epu8
// _mm256_min_epu16
// _mm256_min_epu32

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

// _mm256_sign_epi8
// _mm256_sign_epi16
// _mm256_sign_epi32

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

// _mm256_sub_epi8
// _mm256_sub_epi16
// _mm256_sub_epi32
// _mm256_sub_epi64

// _mm256_subs_epi8
// _mm256_subs_epi16
// _mm256_subs_epu8
// _mm256_subs_epu16

// _mm256_unpackhi_epi8
// _mm256_unpackhi_epi16
// _mm256_unpackhi_epi32
// _mm256_unpackhi_epi64

// _mm256_unpacklo_epi8
// _mm256_unpacklo_epi16
// _mm256_unpacklo_epi32
// _mm256_unpacklo_epi64

// _mm256_xor_si256
