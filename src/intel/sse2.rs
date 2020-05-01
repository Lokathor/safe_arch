#![cfg(target_feature = "sse2")]

use super::*;

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
///     u8::MAX, 0, 5, 6, 135, 14, 20, 26,
///     5, 6, 8, 14, 8, 14, 83, 26
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
/// assert_eq!(c, [u16::MAX, 0, 5, 6, 8, 14, 20, 26]);
/// ```
#[must_use]
#[inline(always)]
pub fn average_u16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_avg_epu16(a.0, b.0) })
}

/// Shifts all bits in the entire register left by a number of **bytes**.
///
/// * **Shift left logical:** New bits at the bottom are all 0s.
/// * **Immediate:** The amount to shift by must be a compile time const.
///
/// Remember that the register overall is using a little-endian design, so
/// however many lanes you choose to think of the the register as, the top the
/// bytes of each lane will shift "off the top" of one lane and then appear at
/// the bottom of the next higher indexed lane.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0x11111111, 0xF, 0xA, 0xB]);
/// //
/// let c: [i32; 4] = byte_shift_left_logical_immediate_m128i!(a, 1).into();
/// assert_eq!(c, [0x11111100, 0xF11, 0xA00, 0xB00]);
/// //
/// let d: u128 = byte_shift_left_logical_immediate_m128i!(a, 1).into();
/// assert_eq!(d, 0xB00_00000A00_00000F11_11111100);
/// ```
#[macro_export]
macro_rules! byte_shift_left_logical_immediate_m128i {
  ($a:expr, $imm:expr) => {{
    let a: m128i = $a;
    const imm: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_bslli_si128;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_bslli_si128;
    m128i(unsafe { _mm_bslli_si128(a.0, imm) })
  }};
}

/// Shifts all bits in the entire register right by a number of **bytes**.
///
/// * **Shift right logical:** New bits at the top are all 0s.
/// * **Immediate:** The amount to shift by must be a compile time const.
///
/// Remember that the register overall is using a little-endian design, so
/// however many lanes you choose to think of the the register as, the bottom
/// bytes of each lane will shift "off the bottom" of one lane and then appear
/// at the top of the next lower indexed lane.
///
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([0x11111111, 0xF, 0xA, 0xB]);
/// //
/// let c: [i32; 4] = byte_shift_right_logical_immediate_m128i!(a, 1).into();
/// assert_eq!(c, [0x0F111111, 0x0A000000, 0x0B000000, 0]);
/// //
/// let d: u128 = byte_shift_right_logical_immediate_m128i!(a, 1).into();
/// assert_eq!(d, 0x0_0B000000_0A000000_0F111111);
/// ```
#[macro_export]
macro_rules! byte_shift_right_logical_immediate_m128i {
  ($a:expr, $imm:expr) => {{
    let a: m128i = $a;
    const imm: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_bsrli_si128;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_bsrli_si128;
    m128i(unsafe { _mm_bsrli_si128(a.0, imm) })
  }};
}

/// Bit-preserving cast to `m128` from `m128d`
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 2.0]);
/// let c: [u32; 4] = cast_to_m128_from_m128d(a).to_bits();
/// assert_eq!(c, [0, 0x3FF00000, 0, 0x40000000]);
/// ```
#[must_use]
#[inline(always)]
pub fn cast_to_m128_from_m128d(a: m128d) -> m128 {
  m128(unsafe { _mm_castpd_ps(a.0) })
}

/// Bit-preserving cast to `m128i` from `m128d`
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 2.0]);
/// let c: [u32; 4] = cast_to_m128i_from_m128d(a).into();
/// assert_eq!(c, [0, 0x3FF00000, 0, 0x40000000]);
/// ```
#[must_use]
#[inline(always)]
pub fn cast_to_m128i_from_m128d(a: m128d) -> m128i {
  m128i(unsafe { _mm_castpd_si128(a.0) })
}

/// Bit-preserving cast to `m128d` from `m128`
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let c: [u64; 2] = cast_to_m128d_from_m128(a).to_bits();
/// assert_eq!(c, [0x400000003F800000, 0x4080000040400000]);
/// ```
#[must_use]
#[inline(always)]
pub fn cast_to_m128d_from_m128(a: m128) -> m128d {
  m128d(unsafe { _mm_castps_pd(a.0) })
}

/// Bit-preserving cast to `m128i` from `m128`
/// ```
/// # use safe_arch::*;
/// let a = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// let c: [u32; 4] = cast_to_m128i_from_m128(a).into();
/// assert_eq!(c, [0x3F800000, 0x40000000, 0x40400000, 0x40800000]);
/// ```
#[must_use]
#[inline(always)]
pub fn cast_to_m128i_from_m128(a: m128) -> m128i {
  m128i(unsafe { _mm_castps_si128(a.0) })
}

/// Bit-preserving cast to `m128d` from `m128i`
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let c: [u64; 2] = cast_to_m128d_from_m128i(a).to_bits();
/// assert_eq!(c, [0x200000001, 0x400000003]);
/// ```
#[must_use]
#[inline(always)]
pub fn cast_to_m128d_from_m128i(a: m128i) -> m128d {
  m128d(unsafe { _mm_castsi128_pd(a.0) })
}

/// Bit-preserving cast to `m128` from `m128i`
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let c: [u32; 4] = cast_to_m128_from_m128i(a).to_bits();
/// assert_eq!(c, [1, 2, 3, 4]);
/// ```
#[must_use]
#[inline(always)]
pub fn cast_to_m128_from_m128i(a: m128i) -> m128 {
  m128(unsafe { _mm_castsi128_ps(a.0) })
}

/// Lanewise `a == b` with lanes as `i8`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(
///   [0_i8, 1, 2, 3, 4, 5, 6, 7,
///   8, 9, 10, 11, 12, 13, 14, 127]
/// );
/// let b = m128i::from(
///   [0_i8, 11, 2, 13, 4, 15, 6, 17,
///   8, 19, -20, 21, 22, -23, 24, 127]
/// );
/// let c: [i8; 16] = cmp_eq_mask_i8_m128i(a, b).into();
/// assert_eq!(
///   c,
///   [-1, 0, -1, 0,-1, 0, -1, 0, -1,
///   0, 0, 0, 0, 0, 0, -1]
/// );
/// ```
#[must_use]
#[inline(always)]
#[rustfmt::skip]
pub fn cmp_eq_mask_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmpeq_epi8(a.0, b.0) })
}

/// Lanewise `a == b` with lanes as `i16`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 2, 3, 4, -1, -2, -3, -4]);
/// let b = m128i::from([5_i16, 2, 7, 4, -15, -26, -37, -4]);
/// let c: [i16; 8] = cmp_eq_mask_i16_m128i(a, b).into();
/// assert_eq!(c, [0, -1, 0, -1, 0, 0, 0, -1]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_eq_mask_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmpeq_epi16(a.0, b.0) })
}

/// Lanewise `a == b` with lanes as `i32`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let b = m128i::from([5, 2, 7, 4]);
/// let c: [i32; 4] = cmp_eq_mask_i32_m128i(a, b).into();
/// assert_eq!(c, [0, -1, 0, -1]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_eq_mask_i32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmpeq_epi32(a.0, b.0) })
}

/// Lanewise `a == b`, mask output.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_eq_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_eq_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpeq_pd(a.0, b.0) })
}

/// Low lane `a == b`, other lanes unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_eq_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_eq_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpeq_sd(a.0, b.0) })
}

/// Lanewise `a >= b`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([3.0, 1.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_ge_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, u64::MAX]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_ge_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpge_pd(a.0, b.0) })
}

/// Low lane `a >= b`, other lanes unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_ge_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_ge_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpge_sd(a.0, b.0) })
}

/// Lanewise `a > b` with lanes as `i8`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(
///   [1_i8, 1, 20, 3, 40, 5, 60, 7, 80,
///   9, 10, 11, 12, 13, 14, 127]
/// );
/// let b = m128i::from(
///   [0_i8, 11, 2, 13, 4, 15, 6, 17,
///   8, 19, -20, 21, 22, -23, 24, 120]
/// );
/// let c: [i8; 16] = cmp_gt_mask_i8_m128i(a, b).into();
/// assert_eq!(
///   c,
///   [-1, 0, -1, 0,-1, 0, -1, 0, -1,
///   0, -1, 0, 0, -1, 0, -1]
/// );
/// ```
#[must_use]
#[inline(always)]
#[rustfmt::skip]
pub fn cmp_gt_mask_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmpgt_epi8(a.0, b.0) })
}

/// Lanewise `a > b` with lanes as `i16`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 20, 3, 40, -1, -2, -3, 0]);
/// let b = m128i::from([5_i16, 2, 7, 4, -15, -26, -37, -4]);
/// let c: [i16; 8] = cmp_gt_mask_i16_m128i(a, b).into();
/// assert_eq!(c, [0, -1, 0, -1, -1, -1, -1, -1]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_gt_mask_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmpgt_epi16(a.0, b.0) })
}

/// Lanewise `a > b` with lanes as `i32`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 20, 7, 40]);
/// let b = m128i::from([5, 2, 7, 4]);
/// let c: [i32; 4] = cmp_gt_mask_i32_m128i(a, b).into();
/// assert_eq!(c, [0, -1, 0, -1]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_gt_mask_i32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmpgt_epi32(a.0, b.0) })
}

/// Lanewise `a > b`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_gt_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_gt_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpgt_pd(a.0, b.0) })
}

/// Low lane `a > b`, other lanes unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_gt_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_gt_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpgt_sd(a.0, b.0) })
}

/// Lanewise `a <= b`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([0.0, 1.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_le_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, u64::MAX]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_le_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmple_pd(a.0, b.0) })
}

/// Low lane `a <= b`, other lanes unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([0.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_le_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_le_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmple_sd(a.0, b.0) })
}

/// Lanewise `a < b` with lanes as `i8`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(
///   [1_i8, 1, 20, 3, 40, 5, 60, 7, 80,
///   9, 10, 11, 12, 13, 14, 127]
/// );
/// let b = m128i::from(
///   [0_i8, 11, 2, 13, 4, 15, 6, 17,
///   8, 19, -20, 21, 22, -23, 24, 120]
/// );
/// let c: [i8; 16] = cmp_lt_mask_i8_m128i(a, b).into();
/// assert_eq!(
///   c,
///   [0, -1, 0,-1,0, -1, 0, -1, 0,
///   -1, 0, -1, -1, 0, -1, 0]
/// );
/// ```
#[must_use]
#[inline(always)]
#[rustfmt::skip]
pub fn cmp_lt_mask_i8_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmplt_epi8(a.0, b.0) })
}

/// Lanewise `a < b` with lanes as `i16`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i16, 20, 3, 40, -1, -2, -3, 0]);
/// let b = m128i::from([5_i16, 2, 7, 4, -15, -26, -37, -4]);
/// let c: [i16; 8] = cmp_lt_mask_i16_m128i(a, b).into();
/// assert_eq!(c, [-1, 0, -1, 0, 0, 0, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_lt_mask_i16_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmplt_epi16(a.0, b.0) })
}

/// Lanewise `a < b` with lanes as `i32`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 20, 7, 40]);
/// let b = m128i::from([5, 2, 7, 4]);
/// let c: [i32; 4] = cmp_lt_mask_i32_m128i(a, b).into();
/// assert_eq!(c, [-1, 0, 0, 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_lt_mask_i32_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmplt_epi32(a.0, b.0) })
}

/// Lanewise `a < b`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([0.0, 7.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_lt_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_lt_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmplt_pd(a.0, b.0) })
}

/// Low lane `a < b`, other lane unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([0.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_lt_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_lt_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmplt_sd(a.0, b.0) })
}

/// Lanewise `a != b`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([3.0, 1.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_neq_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_neq_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpneq_pd(a.0, b.0) })
}

/// Low lane `a != b`, other lane unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_neq_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_neq_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpneq_sd(a.0, b.0) })
}

/// Lanewise `!(a >= b)`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([3.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_nge_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [0, u64::MAX]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_nge_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpnge_pd(a.0, b.0) })
}

/// Low lane `!(a >= b)`, other lane unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_nge_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [0, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_nge_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpnge_sd(a.0, b.0) })
}

/// Lanewise `!(a > b)`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([3.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_ngt_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [0, u64::MAX]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_ngt_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpngt_pd(a.0, b.0) })
}

/// Low lane `!(a > b)`, other lane unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_ngt_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [0, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_ngt_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpngt_sd(a.0, b.0) })
}

/// Lanewise `!(a <= b)`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([3.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_nle_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_nle_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpnle_pd(a.0, b.0) })
}

/// Low lane `!(a <= b)`, other lane unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_nle_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_nle_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpnle_sd(a.0, b.0) })
}

/// Lanewise `!(a < b)`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([3.0, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_nlt_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_nlt_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpnlt_pd(a.0, b.0) })
}

/// Low lane `!(a < b)`, other lane unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_nlt_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_nlt_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpnlt_sd(a.0, b.0) })
}

/// Lanewise `(!a.is_nan()) & (!b.is_nan())`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([3.0, f64::NAN]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_ord_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_ord_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpord_pd(a.0, b.0) })
}

/// Low lane `(!a.is_nan()) & (!b.is_nan())`, other lane unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([2.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_ord_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_ord_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpord_sd(a.0, b.0) })
}

/// Lanewise `a.is_nan() | b.is_nan()`.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([f64::NAN, 0.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_unord_mask_m128d(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 0]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_unord_mask_m128d(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpunord_pd(a.0, b.0) })
}

/// Low lane `a.is_nan() | b.is_nan()`, other lane unchanged.
///
/// Mask output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([f64::NAN, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// let c = cmp_unord_mask_m128d_s(a, b).to_bits();
/// assert_eq!(c, [u64::MAX, 5_f64.to_bits()]);
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_unord_mask_m128d_s(a: m128d, b: m128d) -> m128d {
  m128d(unsafe { _mm_cmpunord_sd(a.0, b.0) })
}

/// Low lane `f64` equal to.
///
/// `i32` output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// assert_eq!(1_i32, cmp_eq_i32_m128d_s(a, b));
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_eq_i32_m128d_s(a: m128d, b: m128d) -> i32 {
  unsafe { _mm_comieq_sd(a.0, b.0) }
}

/// Low lane `f64` greater than or equal to.
///
/// `i32` output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// assert_eq!(1_i32, cmp_ge_i32_m128d_s(a, b));
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_ge_i32_m128d_s(a: m128d, b: m128d) -> i32 {
  unsafe { _mm_comige_sd(a.0, b.0) }
}

/// Low lane `f64` greater than.
///
/// `i32` output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// assert_eq!(1_i32, cmp_ge_i32_m128d_s(a, b));
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_gt_i32_m128d_s(a: m128d, b: m128d) -> i32 {
  unsafe { _mm_comigt_sd(a.0, b.0) }
}

/// Low lane `f64` less than or equal to.
///
/// `i32` output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([1.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// assert_eq!(1_i32, cmp_le_i32_m128d_s(a, b));
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_le_i32_m128d_s(a: m128d, b: m128d) -> i32 {
  unsafe { _mm_comile_sd(a.0, b.0) }
}

/// Low lane `f64` less than.
///
/// `i32` output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([0.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// assert_eq!(1_i32, cmp_lt_i32_m128d_s(a, b));
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_lt_i32_m128d_s(a: m128d, b: m128d) -> i32 {
  unsafe { _mm_comilt_sd(a.0, b.0) }
}

/// Low lane `f64` less than.
///
/// `i32` output.
/// ```
/// # use safe_arch::*;
/// let a = m128d::from_array([0.0, 5.0]);
/// let b = m128d::from_array([1.0, 1.0]);
/// assert_eq!(1_i32, cmp_neq_i32_m128d_s(a, b));
/// ```
#[must_use]
#[inline(always)]
pub fn cmp_neq_i32_m128d_s(a: m128d, b: m128d) -> i32 {
  unsafe { _mm_comineq_sd(a.0, b.0) }
}

/// Rounds the lower two `i32` lanes to two `f64` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let b = convert_to_m128d_from_m128i(a);
/// let c = m128d::from_array([1.0, 2.0]);
/// assert_eq!(b.to_bits(), c.to_bits());
/// ```
#[must_use]
#[inline(always)]
pub fn convert_to_m128d_from_m128i(a: m128i) -> m128d {
  m128d(unsafe { _mm_cvtepi32_pd(a.0) })
}

/// Rounds the four `i32` lanes to four `f32` lanes.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1, 2, 3, 4]);
/// let b = convert_to_m128_from_m128i(a);
/// let c = m128::from_array([1.0, 2.0, 3.0, 4.0]);
/// assert_eq!(b.to_bits(), c.to_bits());
/// ```
#[must_use]
#[inline(always)]
pub fn convert_to_m128_from_m128i(a: m128i) -> m128 {
  m128(unsafe { _mm_cvtepi32_ps(a.0) })
}

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
