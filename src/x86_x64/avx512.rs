#![cfg(target_feature = "avx512f")]
#![allow(non_camel_case_types)]
use super::*;

/// Mask type for 8-element operations
pub type mmask8 = u8;
/// Mask type for 16-element operations
pub type mmask16 = __mmask16;
/// Mask type for 32-element operations
pub type mmask32 = __mmask32;
/// Mask type for 64-element operations
pub type mmask64 = __mmask64;

// Constructors and basic operations

/// Zeroed `m512i`
/// ```
/// # use safe_arch::*;
/// let a = zeroed_m512i();
/// let b: [i32; 16] = a.into();
/// assert_eq!(b, [0; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_setzero_si512`]
/// * **Assembly:** `vpxorq zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn zeroed_m512i() -> m512i {
  m512i(unsafe { _mm512_setzero_si512() })
}

/// Zeroed `m512d`
/// ```
/// # use safe_arch::*;
/// let a = zeroed_m512d();
/// let b: [f64; 8] = a.into();
/// assert_eq!(b, [0.0; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_setzero_pd`]
/// * **Assembly:** `vxorpd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn zeroed_m512d() -> m512d {
    m512d(unsafe { _mm512_setzero_pd() })
}

/// Zeroed `m512`
/// ```
/// # use safe_arch::*;
/// let a = zeroed_m512();
/// let b: [f32; 16] = a.into();
/// assert_eq!(b, [0.0; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_setzero_ps`]
/// * **Assembly:** `vxorps zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn zeroed_m512() -> m512 {
    m512(unsafe { _mm512_setzero_ps() })
}

/// Sets all `i8` lanes to the value given.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i8_m512i(5);
/// let b: [i8; 64] = a.into();
/// assert_eq!(b, [5_i8; 64]);
/// ```
/// * **Intrinsic:** [`_mm512_set1_epi8`]
/// * **Assembly:** `vpbroadcastb zmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn set_splat_i8_m512i(i: i8) -> m512i {
  m512i(unsafe { _mm512_set1_epi8(i) })
}

/// Sets all `i16` lanes to the value given.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i16_m512i(5);
/// let b: [i16; 32] = a.into();
/// assert_eq!(b, [5_i16; 32]);
/// ```
/// * **Intrinsic:** [`_mm512_set1_epi16`]
/// * **Assembly:** `vpbroadcastw zmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn set_splat_i16_m512i(i: i16) -> m512i {
  m512i(unsafe { _mm512_set1_epi16(i) })
}

/// Sets all `i32` lanes to the value given.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i32_m512i(5);
/// let b: [i32; 16] = a.into();
/// assert_eq!(b, [5_i32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_set1_epi32`]
/// * **Assembly:** `vpbroadcastd zmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn set_splat_i32_m512i(i: i32) -> m512i {
  m512i(unsafe { _mm512_set1_epi32(i) })
}

/// Splat an `i64` value into all 8 lanes of an `m512i`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i64_m512i(5);
/// let b: [i64; 8] = a.into();
/// assert_eq!(b, [5_i64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_set1_epi64`]
/// * **Assembly:** `vpbroadcastq zmm, r/m64`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn set_splat_i64_m512i(i: i64) -> m512i {
    m512i(unsafe { _mm512_set1_epi64(i) })
}

/// Splat an `f64` value into all 8 lanes of an `m512d`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(5.0);
/// let b: [f64; 8] = a.into();
/// assert_eq!(b, [5.0_f64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_set1_pd`]
/// * **Assembly:** `vbroadcastsd zmm, r/m64`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn set_splat_m512d(f: f64) -> m512d {
    m512d(unsafe { _mm512_set1_pd(f) })
}

/// Sets all `f32` lanes to the value given.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(5.0);
/// let b: [f32; 16] = a.into();
/// assert_eq!(b, [5.0_f32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_set1_ps`]
/// * **Assembly:** `vbroadcastss zmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn set_splat_m512(f: f32) -> m512 {
  m512(unsafe { _mm512_set1_ps(f) })
}

/// Load data from memory into a register.
/// ```
/// # use safe_arch::*;
/// let a = [1.0_f32; 16];
/// let b = load_m512(&a);
/// let c: [f32; 16] = b.into();
/// assert_eq!(c, [1.0_f32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_loadu_ps`]
/// * **Assembly:** `vmovups zmm, m512`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn load_m512(a: &[f32; 16]) -> m512 {
  m512(unsafe { _mm512_loadu_ps(a.as_ptr()) })
}

/// Load `f64` data from memory into a register.
/// ```
/// # use safe_arch::*;
/// let a = [1.0_f64; 8];
/// let b = load_m512d(&a);
/// let c: [f64; 8] = b.into();
/// assert_eq!(c, [1.0_f64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_loadu_pd`]
/// * **Assembly:** `vmovupd zmm, m512`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn load_m512d(a: &[f64; 8]) -> m512d {
    m512d(unsafe { _mm512_loadu_pd(a.as_ptr()) })
}

/// Load data from memory into a register.
/// ```
/// # use safe_arch::*;
/// let a = [1_i32; 16];
/// let b = load_m512i(&a);
/// let c: [i32; 16] = b.into();
/// assert_eq!(c, [1_i32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_loadu_si512`]
/// * **Assembly:** `vmovdqu64 zmm, m512`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn load_m512i(a: &[i32; 16]) -> m512i {
  m512i(unsafe { _mm512_loadu_si512(a.as_ptr() as *const __m512i) })
}

/// Store a register into memory.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(5.0);
/// let mut b = [0.0_f32; 16];
/// store_m512(&mut b, a);
/// assert_eq!(b, [5.0_f32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_storeu_ps`]
/// * **Assembly:** `vmovups m512, zmm`
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn store_m512(addr: &mut [f32; 16], a: m512) {
  unsafe { _mm512_storeu_ps(addr.as_mut_ptr(), a.0) }
}

/// Store a `m512d` register into memory.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(5.0);
/// let mut b = [0.0_f64; 8];
/// store_m512d(&mut b, a);
/// assert_eq!(b, [5.0_f64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_storeu_pd`]
/// * **Assembly:** `vmovupd m512, zmm`
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn store_m512d(addr: &mut [f64; 8], a: m512d) {
    unsafe { _mm512_storeu_pd(addr.as_mut_ptr(), a.0) }
}

/// Store a register into memory.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i32_m512i(5);
/// let mut b = m512i::default();
/// store_m512i(&mut b, a);
/// let c: [i32; 16] = b.into();
/// assert_eq!(c, [5_i32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_storeu_si512`]
/// * **Assembly:** `vmovdqu64 m512, zmm`
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn store_m512i(addr: &mut m512i, a: m512i) {
  unsafe { _mm512_storeu_si512(addr as *mut m512i as *mut __m512i, a.0) }
}

// Arithmetic operations

/// Lanewise `a + b` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i8_m512i(5);
/// let b = set_splat_i8_m512i(10);
/// let c: [i8; 64] = add_i8_m512i(a, b).into();
/// assert_eq!(c, [15_i8; 64]);
/// ```
/// * **Intrinsic:** [`_mm512_add_epi8`]
/// * **Assembly:** `vpaddb zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn add_i8_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_add_epi8(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `i16`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i16_m512i(5);
/// let b = set_splat_i16_m512i(10);
/// let c: [i16; 32] = add_i16_m512i(a, b).into();
/// assert_eq!(c, [15_i16; 32]);
/// ```
/// * **Intrinsic:** [`_mm512_add_epi16`]
/// * **Assembly:** `vpaddw zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn add_i16_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_add_epi16(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `i32`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i32_m512i(5);
/// let b = set_splat_i32_m512i(10);
/// let c: [i32; 16] = add_i32_m512i(a, b).into();
/// assert_eq!(c, [15_i32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_add_epi32`]
/// * **Assembly:** `vpaddd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn add_i32_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_add_epi32(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `i64`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i64_m512i(5);
/// let b = set_splat_i64_m512i(10);
/// let c: [i64; 8] = add_i64_m512i(a, b).into();
/// assert_eq!(c, [15_i64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_add_epi64`]
/// * **Assembly:** `vpaddd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn add_i64_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_add_epi64(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `f32`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(5.0);
/// let b = set_splat_m512(10.0);
/// let c: [f32; 16] = add_m512(a, b).into();
/// assert_eq!(c, [15.0_f32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_add_ps`]
/// * **Assembly:** `vaddps zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn add_m512(a: m512, b: m512) -> m512 {
  m512(unsafe { _mm512_add_ps(a.0, b.0) })
}

/// Lanewise `a + b` with lanes as `f64`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(5.0);
/// let b = set_splat_m512d(10.0);
/// let c: [f64; 8] = add_m512d(a, b).into();
/// assert_eq!(c, [15.0_f64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_add_pd`]
/// * **Assembly:** `vaddpd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn add_m512d(a: m512d, b: m512d) -> m512d {
  m512d(unsafe { _mm512_add_pd(a.0, b.0) })
}

/// Lanewise `a - b` with lanes as `i8`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i8_m512i(5);
/// let b = set_splat_i8_m512i(10);
/// let c: [i8; 64] = sub_i8_m512i(a, b).into();
/// assert_eq!(c, [-5_i8; 64]);
/// ```
/// * **Intrinsic:** [`_mm512_sub_epi8`]
/// * **Assembly:** `vpsubb zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn sub_i8_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_sub_epi8(a.0, b.0) })
}

/// Lanewise `a - b` with lanes as `i16`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i16_m512i(5);
/// let b = set_splat_i16_m512i(10);
/// let c: [i16; 32] = sub_i16_m512i(a, b).into();
/// assert_eq!(c, [-5_i16; 32]);
/// ```
/// * **Intrinsic:** [`_mm512_sub_epi16`]
/// * **Assembly:** `vpsubw zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn sub_i16_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_sub_epi16(a.0, b.0) })
}

/// Lanewise `a - b` with lanes as `i32`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i32_m512i(5);
/// let b = set_splat_i32_m512i(10);
/// let c: [i32; 16] = sub_i32_m512i(a, b).into();
/// assert_eq!(c, [-5_i32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_sub_epi32`]
/// * **Assembly:** `vpsubd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn sub_i32_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_sub_epi32(a.0, b.0) })
}

/// Lanewise `a - b` with lanes as `i64`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i64_m512i(5);
/// let b = set_splat_i64_m512i(10);
/// let c: [i64; 8] = sub_i64_m512i(a, b).into();
/// assert_eq!(c, [-5_i64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_sub_epi64`]
/// * **Assembly:** `vpsubd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn sub_i64_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_sub_epi64(a.0, b.0) })
}

/// Lanewise `a - b` with lanes as `f32`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(5.0);
/// let b = set_splat_m512(10.0);
/// let c: [f32; 16] = sub_m512(a, b).into();
/// assert_eq!(c, [-5.0_f32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_sub_ps`]
/// * **Assembly:** `vsubps zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn sub_m512(a: m512, b: m512) -> m512 {
  m512(unsafe { _mm512_sub_ps(a.0, b.0) })
}

/// Lanewise `a - b` with lanes as `f32`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(5.0);
/// let b = set_splat_m512d(10.0);
/// let c: [f64; 8] = sub_m512d(a, b).into();
/// assert_eq!(c, [-5.0_f64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_sub_ps`]
/// * **Assembly:** `vsubpd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn sub_m512d(a: m512d, b: m512d) -> m512d {
  m512d(unsafe { _mm512_sub_pd(a.0, b.0) })
}

/// Lanewise `a * b` with lanes as `f32`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(5.0);
/// let b = set_splat_m512(10.0);
/// let c: [f32; 16] = mul_m512(a, b).into();
/// assert_eq!(c, [50.0_f32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_mul_ps`]
/// * **Assembly:** `vmulps zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn mul_m512(a: m512, b: m512) -> m512 {
  m512(unsafe { _mm512_mul_ps(a.0, b.0) })
}

/// Lanewise `a * b` with lanes as `f64`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(5.0);
/// let b = set_splat_m512d(10.0);
/// let c: [f64; 8] = mul_m512d(a, b).into();
/// assert_eq!(c, [50.0_f64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_mul_ps`]
/// * **Assembly:** `vmulpd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn mul_m512d(a: m512d, b: m512d) -> m512d {
  m512d(unsafe { _mm512_mul_pd(a.0, b.0) })
}

/// Multiply the `i16` lanes and keep the low half of each 32-bit output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i16_m512i(5);
/// let b = set_splat_i16_m512i(10);
/// let c: [i16; 32] = mul_i16_keep_low_m512i(a, b).into();
/// assert_eq!(c, [50_i16; 32]);
/// ```
/// * **Intrinsic:** [`_mm512_mullo_epi16`]
/// * **Assembly:** `vpmullw zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn mul_i16_keep_low_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_mullo_epi16(a.0, b.0) })
}

/// Multiply the `i32` lanes and keep the low half of each 64-bit output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i32_m512i(5);
/// let b = set_splat_i32_m512i(10);
/// let c: [i32; 16] = mul_i32_keep_low_m512i(a, b).into();
/// assert_eq!(c, [50_i32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_mullo_epi32`]
/// * **Assembly:** `vpmulld zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn mul_i32_keep_low_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_mullo_epi32(a.0, b.0) })
}

/// Lanewise `a / b` with lanes as `f32`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(50.0);
/// let b = set_splat_m512(10.0);
/// let c: [f32; 16] = div_m512(a, b).into();
/// assert_eq!(c, [5.0_f32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_div_ps`]
/// * **Assembly:** `vdivps zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn div_m512(a: m512, b: m512) -> m512 {
  m512(unsafe { _mm512_div_ps(a.0, b.0) })
}

/// Lanewise `a / b` with lanes as `f64`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(50.0);
/// let b = set_splat_m512d(10.0);
/// let c: [f64; 8] = div_m512d(a, b).into();
/// assert_eq!(c, [5.0_f64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_div_pd`]
/// * **Assembly:** `vdivps zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn div_m512d(a: m512d, b: m512d) -> m512d {
  m512d(unsafe { _mm512_div_pd(a.0, b.0) })
}

/// Fused multiply-add. Computes `(a * b) + c` with a single rounding.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(2.0);
/// let b = set_splat_m512(3.0);
/// let c = set_splat_m512(1.0);
/// let d: [f32; 16] = fmadd_m512(a, b, c).into();
/// assert_eq!(d, [7.0_f32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_fmadd_ps`]
/// * **Assembly:** `vfmadd132ps zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn fmadd_m512(a: m512, b: m512, c: m512) -> m512 {
  m512(unsafe { _mm512_fmadd_ps(a.0, b.0, c.0) })
}

/// Fused multiply-add. Computes `(a * b) + c` with a single rounding.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(2.0);
/// let b = set_splat_m512d(3.0);
/// let c = set_splat_m512d(1.0);
/// let d: [f64; 8] = fmadd_m512d(a, b, c).into();
/// assert_eq!(d, [7.0_f64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_fmadd_pd`]
/// * **Assembly:** `vfmadd132pd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn fmadd_m512d(a: m512d, b: m512d, c: m512d) -> m512d {
  m512d(unsafe { _mm512_fmadd_pd(a.0, b.0, c.0) })
}

// Comparison operations

/// Compare `i8` lanes for equality, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i8_m512i(5);
/// let b = set_splat_i8_m512i(5);
/// let mask = cmp_eq_i8_mask_m512i(a, b);
/// assert_eq!(mask, 0xFFFFFFFFFFFFFFFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmpeq_epi8_mask`]
/// * **Assembly:** `vpcmpeqb k, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn cmp_eq_i8_mask_m512i(a: m512i, b: m512i) -> mmask64 {
  unsafe { _mm512_cmpeq_epi8_mask(a.0, b.0) }
}

/// Compare `u8` lanes for equality, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i8_m512i(5);
/// let b = set_splat_i8_m512i(5);
/// let mask = cmp_eq_u8_mask_m512i(a, b);
/// assert_eq!(mask, 0xFFFFFFFFFFFFFFFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmpeq_epu8_mask`]
/// * **Assembly:** `vpcmpeqb k, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn cmp_eq_u8_mask_m512i(a: m512i, b: m512i) -> mmask64 {
  unsafe { _mm512_cmpeq_epu8_mask(a.0, b.0) }
}

/// Compare `i16` lanes for equality, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i16_m512i(5);
/// let b = set_splat_i16_m512i(5);
/// let mask = cmp_eq_i16_mask_m512i(a, b);
/// assert_eq!(mask, 0xFFFFFFFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmp_epi16_mask`]
/// * **Assembly:** `vpcmpw k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn cmp_eq_i16_mask_m512i(a: m512i, b: m512i) -> mmask32 {
  unsafe { _mm512_cmp_epi16_mask(a.0, b.0, _MM_CMPINT_EQ) }
}

/// Compare `i32` lanes for equality, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i32_m512i(5);
/// let b = set_splat_i32_m512i(5);
/// let mask = cmp_eq_i32_mask_m512i(a, b);
/// assert_eq!(mask, 0xFFFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmp_epi32_mask`]
/// * **Assembly:** `vpcmpd k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cmp_eq_i32_mask_m512i(a: m512i, b: m512i) -> mmask16 {
  unsafe { _mm512_cmp_epi32_mask(a.0, b.0, _MM_CMPINT_EQ) }
}

/// Compare `f32` lanes for equality, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(5.0);
/// let b = set_splat_m512(5.0);
/// let mask = cmp_eq_mask_m512(a, b);
/// assert_eq!(mask, 0xFFFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmp_ps_mask`]
/// * **Assembly:** `vcmpps k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cmp_eq_mask_m512(a: m512, b: m512) -> mmask16 {
  unsafe { _mm512_cmp_ps_mask(a.0, b.0, _CMP_EQ_OQ) }
}

/// Compare `f64` lanes for equality, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(5.0);
/// let b = set_splat_m512d(5.0);
/// let mask = cmp_eq_mask_m512d(a, b);
/// assert_eq!(mask, 0xFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmp_pd_mask`]
/// * **Assembly:** `vcmppd k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cmp_eq_mask_m512d(a: m512d, b: m512d) -> mmask8 {
  unsafe { _mm512_cmp_pd_mask(a.0, b.0, _CMP_EQ_OQ) }
}

/// Compare `i8` lanes for `a > b`, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i8_m512i(10);
/// let b = set_splat_i8_m512i(5);
/// let mask = cmp_gt_i8_mask_m512i(a, b);
/// assert_eq!(mask, 0xFFFFFFFFFFFFFFFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmpgt_epi8_mask`]
/// * **Assembly:** `vpcmpgtb k, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn cmp_gt_i8_mask_m512i(a: m512i, b: m512i) -> mmask64 {
  unsafe { _mm512_cmpgt_epi8_mask(a.0, b.0) }
}

/// Compare `u8` lanes for `a > b`, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i8_m512i(10);
/// let b = set_splat_i8_m512i(5);
/// let mask = cmp_gt_u8_mask_m512i(a, b);
/// assert_eq!(mask, 0xFFFFFFFFFFFFFFFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmpgt_epu8_mask`]
/// * **Assembly:** `vpcmpub k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn cmp_gt_u8_mask_m512i(a: m512i, b: m512i) -> mmask64 {
  unsafe { _mm512_cmpgt_epu8_mask(a.0, b.0) }
}

/// Compare `i8` lanes for `a >= b`, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i8_m512i(10);
/// let b = set_splat_i8_m512i(10);
/// let mask = cmp_ge_i8_mask_m512i(a, b);
/// assert_eq!(mask, 0xFFFFFFFFFFFFFFFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmpge_epi8_mask`]
/// * **Assembly:** `vpcmpb k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn cmp_ge_i8_mask_m512i(a: m512i, b: m512i) -> mmask64 {
  unsafe { _mm512_cmpge_epi8_mask(a.0, b.0) }
}

/// Compare `u8` lanes for `a >= b`, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i8_m512i(10);
/// let b = set_splat_i8_m512i(10);
/// let mask = cmp_ge_u8_mask_m512i(a, b);
/// assert_eq!(mask, 0xFFFFFFFFFFFFFFFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmpge_epu8_mask`]
/// * **Assembly:** `vpcmpub k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn cmp_ge_u8_mask_m512i(a: m512i, b: m512i) -> mmask64 {
  unsafe { _mm512_cmpge_epu8_mask(a.0, b.0) }
}

/// Compare `i16` lanes for `a > b`, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i16_m512i(10);
/// let b = set_splat_i16_m512i(5);
/// let mask = cmp_gt_i16_mask_m512i(a, b);
/// assert_eq!(mask, 0xFFFFFFFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmp_epi16_mask`]
/// * **Assembly:** `vpcmpw k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn cmp_gt_i16_mask_m512i(a: m512i, b: m512i) -> mmask32 {
  unsafe { _mm512_cmp_epi16_mask(a.0, b.0, _MM_CMPINT_NLE) }
}

/// Compare `u16` lanes for `a > b`, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i16_m512i(10);
/// let b = set_splat_i16_m512i(5);
/// let mask = cmp_gt_u16_mask_m512i(a, b);
/// assert_eq!(mask, 0xFFFFFFFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmp_epu16_mask`]
/// * **Assembly:** `vpcmpuw k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn cmp_gt_u16_mask_m512i(a: m512i, b: m512i) -> mmask32 {
  unsafe { _mm512_cmp_epu16_mask(a.0, b.0, _MM_CMPINT_NLE) }
}

/// Compare `i32` lanes for `a > b`, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i32_m512i(10);
/// let b = set_splat_i32_m512i(5);
/// let mask = cmp_gt_i32_mask_m512i(a, b);
/// assert_eq!(mask, 0xFFFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmp_epi32_mask`]
/// * **Assembly:** `vpcmpd k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cmp_gt_i32_mask_m512i(a: m512i, b: m512i) -> mmask16 {
  unsafe { _mm512_cmp_epi32_mask(a.0, b.0, _MM_CMPINT_NLE) }
}

/// Compare `f32` lanes for `a > b`, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(10.0);
/// let b = set_splat_m512(5.0);
/// let mask = cmp_gt_mask_m512(a, b);
/// assert_eq!(mask, 0xFFFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmp_ps_mask`]
/// * **Assembly:** `vcmpps k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cmp_gt_mask_m512(a: m512, b: m512) -> mmask16 {
  unsafe { _mm512_cmp_ps_mask(a.0, b.0, _CMP_GT_OQ) }
}

/// Compare `i8` lanes for `a == b`, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i8_m512i(10);
/// let b = set_splat_i8_m512i(10);
/// let mask = cmp_eq_mask_i8_m512i(a, b);
/// assert_eq!(mask, 0xFFFFFFFFFFFFFFFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmp_epi8_mask`]
/// * **Assembly:** `vpcmpb k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn cmp_eq_mask_i8_m512i(a: m512i, b: m512i) -> mmask64 {
  unsafe { _mm512_cmp_epi8_mask(a.0, b.0, _MM_CMPINT_EQ) }
}

/// Compare `f32` lanes for `a == b`, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(10.0);
/// let b = set_splat_m512(10.0);
/// let mask = cmp_eq_mask_f32_m512(a, b);
/// assert_eq!(mask, 0xFFFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmp_ps_mask`]
/// * **Assembly:** `vcmpps k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cmp_eq_mask_f32_m512(a: m512, b: m512) -> mmask16 {
  unsafe { _mm512_cmp_ps_mask(a.0, b.0, _CMP_EQ_OQ) }
}

/// Compare `f64` lanes for `a == b`, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(10.0);
/// let b = set_splat_m512d(10.0);
/// let mask = cmp_eq_mask_f64_m512d(a, b);
/// assert_eq!(mask, 0xFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmp_pd_mask`]
/// * **Assembly:** `vcmppd k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cmp_eq_mask_f64_m512d(a: m512d, b: m512d) -> mmask8 {
  unsafe { _mm512_cmp_pd_mask(a.0, b.0, _CMP_EQ_OQ) }
}

/// Compare `f64` lanes for `a > b`, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(10.0);
/// let b = set_splat_m512d(5.0);
/// let mask = cmp_gt_mask_m512d(a, b);
/// assert_eq!(mask, 0xFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmp_pd_mask`]
/// * **Assembly:** `vcmppd k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cmp_gt_mask_m512d(a: m512d, b: m512d) -> mmask8 {
  unsafe { _mm512_cmp_pd_mask(a.0, b.0, _CMP_GT_OQ) }
}

/// Compare `f32` lanes for `a >= b`, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(10.0);
/// let b = set_splat_m512(10.0);
/// let mask = cmp_ge_mask_m512(a, b);
/// assert_eq!(mask, 0xFFFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmp_ps_mask`]
/// * **Assembly:** `vcmpps k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cmp_ge_mask_m512(a: m512, b: m512) -> mmask16 {
  unsafe { _mm512_cmp_ps_mask(a.0, b.0, _CMP_GE_OQ) }
}

/// Compare `f64` lanes for `a >= b`, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(10.0);
/// let b = set_splat_m512d(10.0);
/// let mask = cmp_ge_mask_m512d(a, b);
/// assert_eq!(mask, 0xFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmp_pd_mask`]
/// * **Assembly:** `vcmppd k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cmp_ge_mask_m512d(a: m512d, b: m512d) -> mmask8 {
  unsafe { _mm512_cmp_pd_mask(a.0, b.0, _CMP_GE_OQ) }
}

/// Compare `f32` lanes for `a < b`, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(5.0);
/// let b = set_splat_m512(10.0);
/// let mask = cmp_lt_mask_m512(a, b);
/// assert_eq!(mask, 0xFFFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmp_ps_mask`]
/// * **Assembly:** `vcmpps k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cmp_lt_mask_m512(a: m512, b: m512) -> mmask16 {
  unsafe { _mm512_cmp_ps_mask(a.0, b.0, _CMP_LT_OQ) }
}

/// Compare `f64` lanes for `a < b`, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(5.0);
/// let b = set_splat_m512d(10.0);
/// let mask = cmp_lt_mask_m512d(a, b);
/// assert_eq!(mask, 0xFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmp_pd_mask`]
/// * **Assembly:** `vcmppd k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cmp_lt_mask_m512d(a: m512d, b: m512d) -> mmask8 {
  unsafe { _mm512_cmp_pd_mask(a.0, b.0, _CMP_LT_OQ) }
}

/// Compare `f32` lanes for `a <= b`, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(10.0);
/// let b = set_splat_m512(10.0);
/// let mask = cmp_le_mask_m512(a, b);
/// assert_eq!(mask, 0xFFFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmp_ps_mask`]
/// * **Assembly:** `vcmpps k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cmp_le_mask_m512(a: m512, b: m512) -> mmask16 {
  unsafe { _mm512_cmp_ps_mask(a.0, b.0, _CMP_LE_OQ) }
}

/// Compare `f64` lanes for `a <= b`, mask output.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(10.0);
/// let b = set_splat_m512d(10.0);
/// let mask = cmp_le_mask_m512d(a, b);
/// assert_eq!(mask, 0xFF);
/// ```
/// * **Intrinsic:** [`_mm512_cmp_pd_mask`]
/// * **Assembly:** `vcmpps k, zmm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cmp_le_mask_m512d(a: m512d, b: m512d) -> mmask8 {
  unsafe { _mm512_cmp_pd_mask(a.0, b.0, _CMP_LE_OQ) }
}

// Bitwise operations

/// Bitwise `a & b`.
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([0_i64, 0, 1, 1, 0, 0, 1, 1]);
/// let b = m512i::from([0_i64, 1, 0, 1, 0, 1, 0, 1]);
/// let c: [i64; 8] = bitand_m512i(a, b).into();
/// assert_eq!(c, [0_i64, 0, 0, 1, 0, 0, 0, 1]);
/// ```
/// * **Intrinsic:** [`_mm512_and_si512`]
/// * **Assembly:** `vpandq zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn bitand_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_and_si512(a.0, b.0) })
}

/// Bitwise `a & b` with lanes as `f32`.
/// ```
/// # use safe_arch::*;
/// let a = m512::from([1.0_f32; 16]);
/// let b = m512::from([1.0_f32; 16]);
/// let c: [f32; 16] = bitand_m512(a, b).into();
/// assert_eq!(c, [1.0_f32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_and_ps`]
/// * **Assembly:** `vandps zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512dq")))]
pub fn bitand_m512(a: m512, b: m512) -> m512 {
  m512(unsafe { _mm512_and_ps(a.0, b.0) })
}

/// Bitwise `a & b` with lanes as `f64`.
/// ```
/// # use safe_arch::*;
/// let a = m512d::from([1.0_f64; 8]);
/// let b = m512d::from([1.0_f64; 8]);
/// let c: [f64; 8] = bitand_m512d(a, b).into();
/// assert_eq!(c, [1.0_f64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_and_pd`]
/// * **Assembly:** `vandpd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512dq")))]
pub fn bitand_m512d(a: m512d, b: m512d) -> m512d {
  m512d(unsafe { _mm512_and_pd(a.0, b.0) })
}

/// Bitwise `(!a) & b`.
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([0_i64, 0, 1, 1, 0, 0, 1, 1]);
/// let b = m512i::from([0_i64, 1, 0, 1, 0, 1, 0, 1]);
/// let c: [i64; 8] = bitandnot_m512i(a, b).into();
/// assert_eq!(c, [0_i64, 1, 0, 0, 0, 1, 0, 0]);
/// ```
/// * **Intrinsic:** [`_mm512_andnot_si512`]
/// * **Assembly:** `vpandnq zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn bitandnot_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_andnot_si512(a.0, b.0) })
}

/// Bitwise `(!a) & b` with lanes as `f32`.
/// ```
/// # use safe_arch::*;
/// let a = m512::from([0.0_f32; 16]);
/// let b = m512::from([1.0_f32; 16]);
/// let c: [f32; 16] = bitandnot_m512(a, b).into();
/// // The result is not 1.0 due to floating point bit patterns
/// ```
/// * **Intrinsic:** [`_mm512_andnot_ps`]
/// * **Assembly:** `vandnps zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512dq")))]
pub fn bitandnot_m512(a: m512, b: m512) -> m512 {
  m512(unsafe { _mm512_andnot_ps(a.0, b.0) })
}

/// Bitwise `(!a) & b` with lanes as `f64`.
/// ```
/// # use safe_arch::*;
/// let a = m512d::from([0.0_f64; 8]);
/// let b = m512d::from([1.0_f64; 8]);
/// let c: [f64; 8] = bitandnot_m512d(a, b).into();
/// // The result is not 1.0 due to floating point bit patterns
/// ```
/// * **Intrinsic:** [`_mm512_andnot_pd`]
/// * **Assembly:** `vandnpd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512dq")))]
pub fn bitandnot_m512d(a: m512d, b: m512d) -> m512d {
  m512d(unsafe { _mm512_andnot_pd(a.0, b.0) })
}

/// Average `u8` lanes (unsigned 8-bit integers) in two `m512i` vectors.
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([100_u8; 64]);
/// let b = m512i::from([120_u8; 64]);
/// let c: [u8; 64] = average_u8_m512i(a, b).into();
/// assert_eq!(c, [110_u8; 64]);
/// ```
/// * **Intrinsic:** [`_mm512_avg_epu8`]
/// * **Assembly:** `vpavgb zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn average_u8_m512i(a: m512i, b: m512i) -> m512i {
    m512i(unsafe { _mm512_avg_epu8(a.0, b.0) })
}

/// Average `u16` lanes in two `m512i` vectors (unsigned 16-bit integers).
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([100_u16; 32]);
/// let b = m512i::from([120_u16; 32]);
/// let c: [u16; 32] = average_u16_m512i(a, b).into();
/// assert_eq!(c, [110_u16; 32]);
/// ```
/// * **Intrinsic:** [`_mm512_avg_epu16`]
/// * **Assembly:** `vpavgw zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn average_u16_m512i(a: m512i, b: m512i) -> m512i {
    m512i(unsafe { _mm512_avg_epu16(a.0, b.0) })
}

/// Bitwise `a | b`.
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([0_i64, 0, 1, 1, 0, 0, 1, 1]);
/// let b = m512i::from([0_i64, 1, 0, 1, 0, 1, 0, 1]);
/// let c: [i64; 8] = bitor_m512i(a, b).into();
/// assert_eq!(c, [0_i64, 1, 1, 1, 0, 1, 1, 1]);
/// ```
/// * **Intrinsic:** [`_mm512_or_si512`]
/// * **Assembly:** `vporq zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn bitor_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_or_si512(a.0, b.0) })
}

/// Bitwise `a | b` with lanes as `f32`.
/// ```
/// # use safe_arch::*;
/// let a = m512::from([0.0_f32; 16]);
/// let b = m512::from([1.0_f32; 16]);
/// let c: [f32; 16] = bitor_m512(a, b).into();
/// assert_eq!(c, [1.0_f32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_or_ps`]
/// * **Assembly:** `vorps zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512dq")))]
pub fn bitor_m512(a: m512, b: m512) -> m512 {
  m512(unsafe { _mm512_or_ps(a.0, b.0) })
}

/// Bitwise `a | b` with lanes as `f64`.
/// ```
/// # use safe_arch::*;
/// let a = m512d::from([0.0_f64; 8]);
/// let b = m512d::from([1.0_f64; 8]);
/// let c: [f64; 8] = bitor_m512d(a, b).into();
/// assert_eq!(c, [1.0_f64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_or_pd`]
/// * **Assembly:** `vorpd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512dq")))]
pub fn bitor_m512d(a: m512d, b: m512d) -> m512d {
  m512d(unsafe { _mm512_or_pd(a.0, b.0) })
}

/// Bitwise `a ^ b`.
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([0_i64, 0, 1, 1, 0, 0, 1, 1]);
/// let b = m512i::from([0_i64, 1, 0, 1, 0, 1, 0, 1]);
/// let c: [i64; 8] = bitxor_m512i(a, b).into();
/// assert_eq!(c, [0_i64, 1, 1, 0, 0, 1, 1, 0]);
/// ```
/// * **Intrinsic:** [`_mm512_xor_si512`]
/// * **Assembly:** `vpxorq zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn bitxor_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_xor_si512(a.0, b.0) })
}

/// Bitwise `a ^ b` with lanes as `f32`.
/// ```
/// # use safe_arch::*;
/// let a = m512::from([1.0_f32; 16]);
/// let b = m512::from([1.0_f32; 16]);
/// let c: [f32; 16] = bitxor_m512(a, b).into();
/// assert_eq!(c, [0.0_f32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_xor_ps`]
/// * **Assembly:** `vxorps zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512dq")))]
pub fn bitxor_m512(a: m512, b: m512) -> m512 {
  m512(unsafe { _mm512_xor_ps(a.0, b.0) })
}

/// Bitwise `a ^ b` with lanes as `f64`.
/// ```
/// # use safe_arch::*;
/// let a = m512d::from([1.0_f64; 8]);
/// let b = m512d::from([1.0_f64; 8]);
/// let c: [f64; 8] = bitxor_m512d(a, b).into();
/// assert_eq!(c, [0.0_f64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_xor_pd`]
/// * **Assembly:** `vxorpd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512dq")))]
pub fn bitxor_m512d(a: m512d, b: m512d) -> m512d {
  m512d(unsafe { _mm512_xor_pd(a.0, b.0) })
}

// Blend operations

/// Blend `i8` values using a mask.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i8_m512i(10);
/// let b = set_splat_i8_m512i(20);
/// let mask = 0xAAAAAAAAAAAAAAAA;
/// let c: [i8; 64] = blend_varying_i8_m512i(a, b, mask).into();
/// for (i, &val) in c.iter().enumerate() {
///   assert_eq!(val, if (mask >> i) & 1 == 1 { 20 } else { 10 });
/// }
/// ```
/// * **Intrinsic:** [`_mm512_mask_blend_epi8`]
/// * **Assembly:** `vpblendmb zmm {k}, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn blend_varying_i8_m512i(a: m512i, b: m512i, mask: mmask64) -> m512i {
  m512i(unsafe { _mm512_mask_blend_epi8(mask, a.0, b.0) })
}

/// Blend `i16` values using a mask.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i16_m512i(10);
/// let b = set_splat_i16_m512i(20);
/// let mask = 0xAAAAAAAA;
/// let c: [i16; 32] = blend_i16_m512i(a, b, mask).into();
/// for (i, &val) in c.iter().enumerate() {
///   assert_eq!(val, if (mask >> i) & 1 == 1 { 20 } else { 10 });
/// }
/// ```
/// * **Intrinsic:** [`_mm512_mask_blend_epi16`]
/// * **Assembly:** `vpblendmw zmm {k}, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn blend_i16_m512i(a: m512i, b: m512i, mask: mmask32) -> m512i {
  m512i(unsafe { _mm512_mask_blend_epi16(mask, a.0, b.0) })
}

/// Blend `i32` values using a mask.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i32_m512i(10);
/// let b = set_splat_i32_m512i(20);
/// let mask = 0xAAAA;
/// let c: [i32; 16] = blend_i32_m512i(a, b, mask).into();
/// for (i, &val) in c.iter().enumerate() {
///   assert_eq!(val, if (mask >> i) & 1 == 1 { 20 } else { 10 });
/// }
/// ```
/// * **Intrinsic:** [`_mm512_mask_blend_epi32`]
/// * **Assembly:** `vpblendmd zmm {k}, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn blend_i32_m512i(a: m512i, b: m512i, mask: mmask16) -> m512i {
  m512i(unsafe { _mm512_mask_blend_epi32(mask, a.0, b.0) })
}

/// Blend `f32` values using a mask.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(10.0);
/// let b = set_splat_m512(20.0);
/// let mask = 0xAAAA;
/// let c: [f32; 16] = blend_m512(a, b, mask).into();
/// for (i, &val) in c.iter().enumerate() {
///   assert_eq!(val, if (mask >> i) & 1 == 1 { 20.0 } else { 10.0 });
/// }
/// ```
/// * **Intrinsic:** [`_mm512_mask_blend_ps`]
/// * **Assembly:** `vblendmps zmm {k}, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn blend_m512(a: m512, b: m512, mask: mmask16) -> m512 {
  m512(unsafe { _mm512_mask_blend_ps(mask, a.0, b.0) })
}

/// Blend `f64` values using a mask.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(10.0);
/// let b = set_splat_m512d(20.0);
/// let mask = 0xAA;
/// let c: [f64; 8] = blend_m512d(a, b, mask).into();
/// for (i, &val) in c.iter().enumerate() {
///   assert_eq!(val, if (mask >> i) & 1 == 1 { 20.0 } else { 10.0 });
/// }
/// ```
/// * **Intrinsic:** [`_mm512_mask_blend_pd`]
/// * **Assembly:** `vblendmpd zmm {k}, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn blend_m512d(a: m512d, b: m512d, mask: mmask8) -> m512d {
  m512d(unsafe { _mm512_mask_blend_pd(mask, a.0, b.0) })
}

/// Sets the lowest `i8` lane of an `m128i` as all lanes of an `m512i`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(7_i8 as i128);
/// let b: [i8; 64] = set_splat_i8_m128i_s_m512i(a).into();
/// assert_eq!(b, [7_i8; 64]);
/// ```
/// * **Intrinsic:** [`_mm512_broadcastb_epi8`]
/// * **Assembly:** `vpbroadcastb zmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(all(target_feature = "avx512bw", target_feature = "avx512vl"))))]
pub fn set_splat_i8_m128i_s_m512i(a: m128i) -> m512i {
    m512i(unsafe { _mm512_broadcastb_epi8(a.0) })
}

/// Sets the lowest `i16` lane of an `m128i` as all lanes of an `m512i`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(42_i16 as i128);
/// let b: [i16; 32] = set_splat_i16_m128i_s_m512i(a).into();
/// assert_eq!(b, [42_i16; 32]);
/// ```
/// * **Intrinsic:** [`_mm512_broadcastw_epi16`]
/// * **Assembly:** `vpbroadcastw zmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(all(target_feature = "avx512bw", target_feature = "avx512vl"))))]
pub fn set_splat_i16_m128i_s_m512i(a: m128i) -> m512i {
    m512i(unsafe { _mm512_broadcastw_epi16(a.0) })
}

/// Sets the lowest `i32` lane of an `m128i` as all lanes of an `m512i`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(123_i32 as i128);
/// let b: [i32; 16] = set_splat_i32_m128i_s_m512i(a).into();
/// assert_eq!(b, [123_i32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_broadcastd_epi32`]
/// * **Assembly:** `vpbroadcastd zmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn set_splat_i32_m128i_s_m512i(a: m128i) -> m512i {
    m512i(unsafe { _mm512_broadcastd_epi32(a.0) })
}

/// Sets the lowest `i64` lane of an `m128i` as all lanes of an `m512i`.
/// ```
/// # use safe_arch::*;
/// let a = m128i::from(99_i64 as i128);
/// let b: [i64; 8] = set_splat_i64_m128i_s_m512i(a).into();
/// assert_eq!(b, [99_i64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_broadcastq_epi64`]
/// * **Assembly:** `vpbroadcastq zmm, xmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn set_splat_i64_m128i_s_m512i(a: m128i) -> m512i {
    m512i(unsafe { _mm512_broadcastq_epi64(a.0) })
}

// Conversion operations

/// Convert `i8` values to `i16` values.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([-5_i8; 32]);
/// let b: [i16; 32] = convert_to_i16_m512i_from_i8_m256i(a).into();
/// assert_eq!(b, [-5_i16; 32]);
/// ```
/// * **Intrinsic:** [`_mm512_cvtepi8_epi16`]
/// * **Assembly:** `vpmovsxbw zmm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn convert_to_i16_m512i_from_i8_m256i(a: m256i) -> m512i {
  m512i(unsafe { _mm512_cvtepi8_epi16(a.0) })
}

/// Convert `u8` values to `i16` values.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([5_u8; 32]);
/// let b: [i16; 32] = convert_to_i16_m512i_from_u8_m256i(a).into();
/// assert_eq!(b, [5_i16; 32]);
/// ```
/// * **Intrinsic:** [`_mm512_cvtepu8_epi16`]
/// * **Assembly:** `vpmovzxbw zmm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn convert_to_i16_m512i_from_u8_m256i(a: m256i) -> m512i {
  m512i(unsafe { _mm512_cvtepu8_epi16(a.0) })
}

/// Convert `i16` values to `i32` values.
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([-5_i16; 16]);
/// let b: [i32; 16] = convert_to_i32_m512i_from_i16_m256i(a).into();
/// assert_eq!(b, [-5_i32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_cvtepi16_epi32`]
/// * **Assembly:** `vpmovsxwd zmm, ymm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn convert_to_i32_m512i_from_i16_m256i(a: m256i) -> m512i {
  m512i(unsafe { _mm512_cvtepi16_epi32(a.0) })
}

/// Convert `i16` values to `i8` values, saturating.
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([5_i16; 32]);
/// let b: [i8; 32] = convert_to_i8_m256i_from_i16_m512i(a).into();
/// assert_eq!(b, [5_i8; 32]);
/// ```
/// * **Intrinsic:** [`_mm512_cvtepi16_epi8`]
/// * **Assembly:** `vpmovwb ymm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn convert_to_i8_m256i_from_i16_m512i(a: m512i) -> m256i {
  m256i(unsafe { _mm512_cvtepi16_epi8(a.0) })
}

/// Convert `f64` values to `i64` values.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(5.5);
/// let b: [i64; 8] = convert_m512d_i64_m512i(a).into();
/// assert_eq!(b, [6_i64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_cvtpd_epi64`]
/// * **Assembly:** `vcvtpd2dq zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn convert_m512d_i64_m512i(a: m512d) -> m512i {
  m512i(unsafe { _mm512_cvtpd_epi64(a.0) })
}

/// Convert `f32` values to `i32` values.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(5.5);
/// let b: [i32; 16] = convert_m512_i32_m512i(a).into();
/// assert_eq!(b, [6_i32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_cvtps_epi32`]
/// * **Assembly:** `vcvtps2dq zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn convert_m512_i32_m512i(a: m512) -> m512i {
  m512i(unsafe { _mm512_cvtps_epi32(a.0) })
}

/// Convert `f32` values to `i32` values with truncation.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(5.9);
/// let b: [i32; 16] = convert_truncate_m512_i32_m512i(a).into();
/// assert_eq!(b, [5_i32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_cvttps_epi32`]
/// * **Assembly:** `vcvttps2dq zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn convert_truncate_m512_i32_m512i(a: m512) -> m512i {
  m512i(unsafe { _mm512_cvttps_epi32(a.0) })
}

/// Convert `f64` values to `i64` values with truncation.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(5.9);
/// let b: [i64; 8] = convert_truncate_m512d_i64_m512i(a).into();
/// assert_eq!(b, [5_i64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_cvttpd_epi64`]
/// * **Assembly:** `vcvttps2dq zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn convert_truncate_m512d_i64_m512i(a: m512d) -> m512i {
  m512i(unsafe { _mm512_cvttpd_epi64(a.0) })
}

// Pack operations

/// Saturating convert `i32` to `i16`, and pack the values.
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([1_i32; 16]);
/// let b = m512i::from([2_i32; 16]);
/// let c: [i16; 32] = pack_i32_to_i16_m512i(a, b).into();
/// assert_eq!(c, [
///   1, 1, 1, 1,
///   2, 2, 2, 2,
///   1, 1, 1, 1,
///   2, 2, 2, 2,
///   1, 1, 1, 1,
///   2, 2, 2, 2,
///   1, 1, 1, 1,
///   2, 2, 2, 2,
/// ]);
/// ```
/// * **Intrinsic:** [`_mm512_packs_epi32`]
/// * **Assembly:** `vpackssdw zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn pack_i32_to_i16_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_packs_epi32(a.0, b.0) })
}

/// Saturating convert `i16` to `u8`, and pack the values.
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([1_i16; 32]);
/// let b = m512i::from([2_i16; 32]);
/// let c: [u8; 64] = pack_i16_to_u8_m512i(a, b).into();
/// assert_eq!(c, [
///   1, 1, 1, 1, 1, 1, 1, 1,
///   2, 2, 2, 2, 2, 2, 2, 2,
///   1, 1, 1, 1, 1, 1, 1, 1,
///   2, 2, 2, 2, 2, 2, 2, 2,
///   1, 1, 1, 1, 1, 1, 1, 1,
///   2, 2, 2, 2, 2, 2, 2, 2,
///   1, 1, 1, 1, 1, 1, 1, 1,
///   2, 2, 2, 2, 2, 2, 2, 2
/// ]);
/// ```
/// * **Intrinsic:** [`_mm512_packus_epi16`]
/// * **Assembly:** `vpackuswb zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn pack_i16_to_u8_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_packus_epi16(a.0, b.0) })
}

// Unpack operations

/// Unpack and interleave high `i8` lanes of `a` and `b`.
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([1_i8; 64]);
/// let b = m512i::from([2_i8; 64]);
/// let c: [i8; 64] = unpack_high_i8_m512i(a, b).into();
/// // Unpacking happens within each 128-bit lane
/// ```
/// * **Intrinsic:** [`_mm512_unpackhi_epi8`]
/// * **Assembly:** `vpunpckhbw zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn unpack_high_i8_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_unpackhi_epi8(a.0, b.0) })
}

/// Unpack and interleave high `i16` lanes of `a` and `b`.
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([1_i16; 32]);
/// let b = m512i::from([2_i16; 32]);
/// let c: [i16; 32] = unpack_high_i16_m512i(a, b).into();
/// // Unpacking happens within each 128-bit lane
/// ```
/// * **Intrinsic:** [`_mm512_unpackhi_epi16`]
/// * **Assembly:** `vpunpckhwd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn unpack_high_i16_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_unpackhi_epi16(a.0, b.0) })
}

/// Unpack and interleave low `i8` lanes of `a` and `b`.
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([1_i8; 64]);
/// let b = m512i::from([2_i8; 64]);
/// let c: [i8; 64] = unpack_low_i8_m512i(a, b).into();
/// // Unpacking happens within each 128-bit lane
/// ```
/// * **Intrinsic:** [`_mm512_unpacklo_epi8`]
/// * **Assembly:** `vpunpcklbw zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn unpack_low_i8_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_unpacklo_epi8(a.0, b.0) })
}

/// Unpack and interleave low `i16` lanes of `a` and `b`.
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([1_i16; 32]);
/// let b = m512i::from([2_i16; 32]);
/// let c: [i16; 32] = unpack_low_i16_m512i(a, b).into();
/// // Unpacking happens within each 128-bit lane
/// ```
/// * **Intrinsic:** [`_mm512_unpacklo_epi16`]
/// * **Assembly:** `vpunpcklwd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn unpack_low_i16_m512i(a: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_unpacklo_epi16(a.0, b.0) })
}

// Shift operations

/// Lanewise `u16` shift left by the matching `u16` lane in `count`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i16_m512i(1);
/// let count = set_splat_i16_m512i(2);
/// let b: [u16; 32] = shl_each_u16_m512i(a, count).into();
/// assert_eq!(b, [4_u16; 32]);
/// ```
/// * **Intrinsic:** [`_mm512_sllv_epi16`]
/// * **Assembly:** `vpsllvw zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn shl_each_u16_m512i(a: m512i, count: m512i) -> m512i {
  m512i(unsafe { _mm512_sllv_epi16(a.0, count.0) })
}

/// Lanewise `u32` shift left by the matching `u32` lane in `count`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i32_m512i(1);
/// let count = set_splat_i32_m512i(2);
/// let b: [u32; 16] = shl_each_u32_m512i(a, count).into();
/// assert_eq!(b, [4_u32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_sllv_epi32`]
/// * **Assembly:** `vpsllvd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn shl_each_u32_m512i(a: m512i, count: m512i) -> m512i {
  m512i(unsafe { _mm512_sllv_epi32(a.0, count.0) })
}

/// Lanewise `u64` shift left by the matching `u64` lane in `count`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i64_m512i(1);
/// let count = set_splat_i64_m512i(2);
/// let b: [u64; 8] = shl_each_u64_m512i(a, count).into();
/// assert_eq!(b, [4_u64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_sllv_epi64`]
/// * **Assembly:** `vpdllvd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn shl_each_u64_m512i(a: m512i, count: m512i) -> m512i {
  m512i(unsafe { _mm512_sllv_epi64(a.0, count.0) })
}

// Extract and insert operations

/// Extract 256-bit integer from `a` at the specified index.
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([1_i64, 2, 3, 4, 5, 6, 7, 8]);
/// let b: [i64; 4] = extract_m256i_from_m512i::<0>(a).into();
/// assert_eq!(b, [1, 2, 3, 4]);
/// let c: [i64; 4] = extract_m256i_from_m512i::<1>(a).into();
/// assert_eq!(c, [5, 6, 7, 8]);
/// ```
/// * **Intrinsic:** [`_mm512_extracti64x4_epi64`]
/// * **Assembly:** `vextracti64x4 ymm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn extract_m256i_from_m512i<const LANE: i32>(a: m512i) -> m256i {
    m256i(unsafe { _mm512_extracti64x4_epi64(a.0, LANE) })
}

/// Extract 256-bit float from `a` at the specified index.
/// ```
/// # use safe_arch::*;
/// let a = m512::from([1.0_f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
///                     9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
/// let b: [f32; 8] = extract_m256_from_m512::<0>(a).into();
/// assert_eq!(b, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
/// let c: [f32; 8] = extract_m256_from_m512::<1>(a).into();
/// assert_eq!(c, [9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
/// ```
/// * **Intrinsic:** [`_mm512_extractf32x8_ps`]
/// * **Assembly:** `vextractf32x8 ymm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn extract_m256_from_m512<const LANE: i32>(a: m512) -> m256 {
    m256(unsafe { _mm512_extractf32x8_ps(a.0, LANE) })
}

/// Extract 256-bit double-precision float from `a` at the specified index.
/// ```
/// # use safe_arch::*;
/// let a = m512d::from([1.0_f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
/// let b: [f64; 4] = extract_m256d_from_m512d::<0>(a).into();
/// assert_eq!(b, [1.0, 2.0, 3.0, 4.0]);
/// let c: [f64; 4] = extract_m256d_from_m512d::<1>(a).into();
/// assert_eq!(c, [5.0, 6.0, 7.0, 8.0]);
/// ```
/// * **Intrinsic:** [`_mm512_extractf64x4_pd`]
/// * **Assembly:** `vextractf64x4 ymm, zmm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn extract_m256d_from_m512d<const LANE: i32>(a: m512d) -> m256d {
    m256d(unsafe { _mm512_extractf64x4_pd(a.0, LANE) })
}

/// Insert 256-bit integer into `a` at the specified index.
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([1_i64, 2, 3, 4, 5, 6, 7, 8]);
/// let b = m256i::from([10_i64, 11, 12, 13]);
/// let c: [i64; 8] = insert_m256i_to_m512i::<1>(a, b).into();
/// assert_eq!(c, [1, 2, 3, 4, 10, 11, 12, 13]);
/// ```
/// * **Intrinsic:** [`_mm512_inserti64x4`]
/// * **Assembly:** `vinserti64x4 zmm, zmm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn insert_m256i_to_m512i<const LANE: i32>(a: m512i, b: m256i) -> m512i {
    m512i(unsafe { _mm512_inserti64x4(a.0, b.0, LANE) })
}

/// Insert 256-bit single-precision float into `a` at the specified index.
/// ```
/// # use safe_arch::*;
/// let a = m512::from([1.0_f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
///                     9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
/// let b = m256::from([100.0, 101.0, 102.0, 103.0, 104.0, 105.0, 106.0, 107.0]);
/// let c: [f32; 16] = insert_m256_to_m512::<1>(a, b).into();
/// assert_eq!(c[8..], [100.0, 101.0, 102.0, 103.0, 104.0, 105.0, 106.0, 107.0]);
/// ```
/// * **Intrinsic:** [`_mm512_insertf32x8`]
/// * **Assembly:** `vinsertf32x8 zmm, zmm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn insert_m256_to_m512<const LANE: i32>(a: m512, b: m256) -> m512 {
    m512(unsafe { _mm512_insertf32x8(a.0, b.0, LANE) })
}

/// Insert 256-bit double-precision float into `a` at the specified index.
/// ```
/// # use safe_arch::*;
/// let a = m512d::from([1.0_f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
/// let b = m256d::from([10.0, 11.0, 12.0, 13.0]);
/// let c: [f64; 8] = insert_m256d_to_m512d::<1>(a, b).into();
/// assert_eq!(c, [1.0, 2.0, 3.0, 4.0, 10.0, 11.0, 12.0, 13.0]);
/// ```
/// * **Intrinsic:** [`_mm512_insertf64x4`]
/// * **Assembly:** `vinsertf64x4 zmm, zmm, ymm, imm8`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn insert_m256d_to_m512d<const LANE: i32>(a: m512d, b: m256d) -> m512d {
    m512d(unsafe { _mm512_insertf64x4(a.0, b.0, LANE) })
}

// Cast operations

/// Expand a `__mmask16` into a full-width `__m512` mask vector for `f32` lanes.
///
/// # Examples
/// ```rust
/// # use safe_arch::*;
/// let full = maskz_mov_f32_m512(!0u16);
/// assert_eq!(full.to_bits(), [u32::MAX; 16]);
/// let none = maskz_mov_f32_m512(0);
/// assert_eq!(none, set_splat_m512(0.0));
/// ```
/// * **Intrinsic:** `_mm512_maskz_mov_ps`  
/// * **Assembly:** `VMOVDQU32 zmm{dest}{mask}{z}, zmmones`
#[must_use]
#[inline(always)]
#[cfg(target_feature = "avx512f")]
pub fn maskz_mov_f32_m512(mask: __mmask16) -> m512 {
    let ones: __m512 = unsafe { _mm512_castsi512_ps(_mm512_set1_epi32(-1)) };
    m512(unsafe { _mm512_maskz_mov_ps(mask, ones) })
}

/// Expand a `__mmask16` into a full-width `__m512d` mask vector for `f64` lanes.
///
/// # Examples
/// ```rust
/// # use safe_arch::*;
/// let full = maskz_mov_f64_m512d(!0u8);
/// assert_eq!(full.to_bits(), [u64::MAX; 8]);
/// let none = maskz_mov_f64_m512d(0);
/// assert_eq!(none, set_splat_m512d(0.0));
/// ```
/// * **Intrinsic:** `_mm512_maskz_mov_pd`  
/// * **Assembly:** `VMOVDQU64 zmm{dest}{mask}{z}, zmmones`
#[must_use]
#[inline(always)]
#[cfg(target_feature = "avx512f")]
pub fn maskz_mov_f64_m512d(mask: __mmask8) -> m512d {
    let ones: __m512d = unsafe { _mm512_castsi512_pd(_mm512_set1_epi64(-1)) };
    m512d(unsafe { _mm512_maskz_mov_pd(mask, ones) })
}

/// Expand a `mmask8` into a full-width `__m512i` mask vector for 8 lanes of `i64`.
///
/// # Examples
/// ```rust
/// # use safe_arch::*;
/// let full = maskz_mov_i64_m512i(!0u8);
/// assert_eq!(full, set_splat_i64_m512i(-1));
/// let none = maskz_mov_i64_m512i(0);
/// assert_eq!(none, set_splat_i64_m512i(0));
/// ```
/// * **Intrinsic:** `_mm512_maskz_mov_epi64`  
/// * **Assembly:** `VMOVDQU64 zmm{dest}{mask}{z}, zmmones`
#[must_use]
#[inline(always)]
#[cfg(target_feature = "avx512f")]
pub fn maskz_mov_i64_m512i(mask: __mmask8) -> m512i {
    let ones: __m512i = unsafe { _mm512_set1_epi64(-1) };
    m512i(unsafe { _mm512_maskz_mov_epi64(mask, ones) })
}

/// Expand a `mmask16` into a full-width `__m512i` mask vector for 16 lanes of `i32`.
///
/// # Examples
/// ```rust
/// # use safe_arch::*;
/// let full = maskz_mov_i32_m512i(!0u16);
/// assert_eq!(full, set_splat_i32_m512i(-1));
/// let none = maskz_mov_i32_m512i(0);
/// assert_eq!(none, set_splat_i32_m512i(0));
/// ```
/// * **Intrinsic:** `_mm512_maskz_mov_epi32`  
/// * **Assembly:** `VMOVDQU32 zmm{dest}{mask}{z}, zmmones`
#[must_use]
#[inline(always)]
#[cfg(target_feature = "avx512f")]
pub fn maskz_mov_i32_m512i(mask: __mmask16) -> m512i {
    let ones: __m512i = unsafe { _mm512_set1_epi32(-1) };
    m512i(unsafe { _mm512_maskz_mov_epi32(mask, ones) })
}

/// Expand a `mmask32` into a full-width `__m512i` mask vector for 32 lanes of `i16`.
///
/// # Examples
/// ```rust
/// # use safe_arch::*;
/// let full = maskz_mov_i16_m512i(!0u32);
/// assert_eq!(full.to_array(), [i16::MAX; 32]);
/// let none = maskz_mov_i16_m512i(0);
/// assert_eq!(none.to_array(), [0; 32]);
/// ```
/// * **Intrinsic:** `_mm512_maskz_mov_epi16`  
/// * **Assembly:** `VMOVDQU16 zmm{dest}{mask}{z}, zmmones`
#[must_use]
#[inline(always)]
#[cfg(target_feature = "avx512bw")]
pub fn maskz_mov_i16_m512i(mask: __mmask32) -> m512i {
    let ones: __m512i = unsafe { _mm512_set1_epi16(-1) };
    m512i(unsafe { _mm512_maskz_mov_epi16(mask, ones) })
}

/// Expand a `mmask64` into a full-width `__m512i` mask vector for 64 lanes of `i8`.
///
/// # Examples
/// ```rust
/// # use safe_arch::*;
/// let full = maskz_mov_i8_m512i(!0u64);
/// assert_eq!(full.to_array(), [i8::MAX; 64]);
/// let none = maskz_mov_i8_m512i(0);
/// assert_eq!(none.to_array(), [0; 64]);
/// ```
/// * **Intrinsic:** `_mm512_maskz_mov_epi8`  
/// * **Assembly:** `VMOVDQU8 zmm{dest}{mask}{z}, zmmones`
#[must_use]
#[inline(always)]
#[cfg(target_feature = "avx512bw")]
pub fn maskz_mov_i8_m512i(mask: __mmask64) -> m512i {
    let ones: __m512i = unsafe { _mm512_set1_epi8(-1) };
    m512i(unsafe { _mm512_maskz_mov_epi8(mask, ones) })
}

/// Cast `m256i` to `m512i` (no conversion, upper bits undefined).
/// ```
/// # use safe_arch::*;
/// let a = m256i::from([1_i64; 4]);
/// let b = cast_m256i_to_m512i(a);
/// // Lower 256 bits are preserved, upper 256 bits are undefined
/// ```
/// * **Intrinsic:** [`_mm512_castsi256_si512`]
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cast_m256i_to_m512i(a: m256i) -> m512i {
  m512i(unsafe { _mm512_castsi256_si512(a.0) })
}

/// Cast `m256d` to `m512d` (no conversion, upper bits undefined).
/// ```
/// # use safe_arch::*;
/// let a = m256d::from([1.0_f64; 4]);
/// let b = cast_m256d_to_m512d(a);
/// // Lower 256 bits are preserved, upper 256 bits are undefined
/// ```
/// * **Intrinsic:** [`_mm512_castpd256_pd512`]
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cast_m256d_to_m512d(a: m256d) -> m512d {
    m512d(unsafe { _mm512_castpd256_pd512(a.0) })
}

/// Cast `m256` to `m512` (no conversion, upper bits undefined).
/// ```
/// # use safe_arch::*;
/// let a = m256::from([1.0_f32; 8]);
/// let b = cast_m256_to_m512(a);
/// // Lower 256 bits are preserved, upper 256 bits are undefined
/// ```
/// * **Intrinsic:** [`_mm512_castps256_ps512`]
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cast_m256_to_m512(a: m256) -> m512 {
    m512(unsafe { _mm512_castps256_ps512(a.0) })
}

/// Cast `m512i` to `m256i` (truncate to lower 256 bits).
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([1_i64, 2, 3, 4, 5, 6, 7, 8]);
/// let b: [i64; 4] = cast_m512i_to_m256i(a).into();
/// assert_eq!(b, [1, 2, 3, 4]);
/// ```
/// * **Intrinsic:** [`_mm512_castsi512_si256`]
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cast_m512i_to_m256i(a: m512i) -> m256i {
  m256i(unsafe { _mm512_castsi512_si256(a.0) })
}

/// Cast `m512` to `m256` (truncate to lower 256 bits).
/// ```
/// # use safe_arch::*;
/// let a = m512::from([1.0_f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
///                     9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
/// let b: [f32; 8] = cast_m512_to_m256(a).into();
/// assert_eq!(b, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
/// ```
/// * **Intrinsic:** [`_mm512_castps512_ps256`]
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cast_m512_to_m256(a: m512) -> m256 {
    m256(unsafe { _mm512_castps512_ps256(a.0) })
}

/// Cast `m512d` to `m256d` (truncate to lower 256 bits).
/// ```
/// # use safe_arch::*;
/// let a = m512d::from([1.0_f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
/// let b: [f64; 4] = cast_m512d_to_m256d(a).into();
/// assert_eq!(b, [1.0, 2.0, 3.0, 4.0]);
/// ```
/// * **Intrinsic:** [`_mm512_castpd512_pd256`]
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cast_m512d_to_m256d(a: m512d) -> m256d {
    m256d(unsafe { _mm512_castpd512_pd256(a.0) })
}

// Permutation operations

/// Shuffle `i32` values between `a` and `b` using variable indices.
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([0_i32; 16]);
/// let b = m512i::from([16_i32; 16]);
/// let idx = m512i::from([16_i32; 16]); // All select from b[0]
/// let c: [i32; 16] = shuffle_abv_i32_all_m512i(a, idx, b).into();
/// assert_eq!(c, [16_i32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_permutex2var_epi32`]
/// * **Assembly:** `vpermt2d zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn shuffle_abv_i32_all_m512i(a: m512i, idx: m512i, b: m512i) -> m512i {
  m512i(unsafe { _mm512_permutex2var_epi32(a.0, idx.0, b.0) })
}

/// Shuffle `i64` values in `a` using variable indices.
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([0_i64, 1, 2, 3, 4, 5, 6, 7]);
/// let idx = m512i::from([7_i64, 6, 5, 4, 3, 2, 1, 0]);
/// let b: [i64; 8] = permute_i64_m512i(idx, a).into();
/// assert_eq!(b, [7, 6, 5, 4, 3, 2, 1, 0]);
/// ```
/// * **Intrinsic:** [`_mm512_permutexvar_epi64`]
/// * **Assembly:** `vpermq zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn permute_i64_m512i(idx: m512i, a: m512i) -> m512i {
  m512i(unsafe { _mm512_permutexvar_epi64(idx.0, a.0) })
}

/// Shuffle `i32` values in `a` using variable indices.
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([0_i32, 1, 2, 3, 4, 5, 6, 7,
///                      8, 9, 10, 11, 12, 13, 14, 15]);
/// let idx = m512i::from([15_i32, 14, 13, 12, 11, 10, 9, 8,
///                        7, 6, 5, 4, 3, 2, 1, 0]);
/// let b: [i32; 16] = permute_i32_m512i(idx, a).into();
/// assert_eq!(b, [15, 14, 13, 12, 11, 10, 9, 8,
///                7, 6, 5, 4, 3, 2, 1, 0]);
/// ```
/// * **Intrinsic:** [`_mm512_permutexvar_epi32`]
/// * **Assembly:** `vpermd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn permute_i32_m512i(idx: m512i, a: m512i) -> m512i {
    m512i(unsafe { _mm512_permutexvar_epi32(idx.0, a.0) })
}

/// Permute `i32` values from `a` and `b` using index vector `idx`.
/// ```
/// # use safe_arch::*;
/// let a = m512i::from([0_i32, 1, 2, 3, 4, 5, 6, 7,
///                      8, 9, 10, 11, 12, 13, 14, 15]);
/// let b = m512i::from([100, 101, 102, 103, 104, 105, 106, 107,
///                      108, 109, 110, 111, 112, 113, 114, 115]);
/// // Even indices select from `a`, odd indices from `b`
/// let idx = m512i::from([0, 17, 2, 19, 4, 21, 6, 23,
///                        8, 25, 10, 27, 12, 29, 14, 31]);
/// let c: [i32; 16] = permute2_i32_m512i(a, idx, b).into();
/// assert_eq!(c, [0, 101, 2, 103, 4, 105, 6, 107,
///                8, 109, 10, 111, 12, 113, 14, 115]);
/// ```
/// * **Intrinsic:** [`_mm512_permutex2var_epi32`]
/// * **Assembly:** `vpermt2d zmm1, zmm2, zmm3`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512vl,avx512f")))]
pub fn permute2_i32_m512i(a: m512i, idx: m512i, b: m512i) -> m512i {
    m512i(unsafe { _mm512_permutex2var_epi32(a.0, idx.0, b.0) })
}

// Reduction operations

/// Reduce by adding all `f32` lanes together.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(1.0);
/// let sum = reduce_add_m512(a);
/// assert_eq!(sum, 16.0);
/// ```
/// * **Intrinsic:** [`_mm512_reduce_add_ps`]
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn reduce_add_m512(a: m512) -> f32 {
  unsafe { _mm512_reduce_add_ps(a.0) }
}

/// Reduce by adding all `f64` lanes together.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(1.0);
/// let sum = reduce_add_m512d(a);
/// assert_eq!(sum, 8.0);
/// ```
/// * **Intrinsic:** [`_mm512_reduce_add_pd`]
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn reduce_add_m512d(a: m512d) -> f64 {
    unsafe { _mm512_reduce_add_pd(a.0) }
}

// Max/min operations

/// Lanewise `max(a, b)` with lanes as `f32`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(1.0);
/// let b = set_splat_m512(2.0);
/// let c: [f32; 16] = max_m512(a, b).into();
/// assert_eq!(c, [2.0_f32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_max_ps`]
/// * **Assembly:** `vmaxps zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn max_m512(a: m512, b: m512) -> m512 {
  m512(unsafe { _mm512_max_ps(a.0, b.0) })
}

/// Lanewise `max(a, b)` with lanes as `f64`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(1.0);
/// let b = set_splat_m512d(2.0);
/// let c: [f64; 8] = max_m512d(a, b).into();
/// assert_eq!(c, [2.0_f64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_max_ps`]
/// * **Assembly:** `vmaxpd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn max_m512d(a: m512d, b: m512d) -> m512d {
  m512d(unsafe { _mm512_max_pd(a.0, b.0) })
}

/// Lanewise `min(a, b)` with lanes as `f32`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(1.0);
/// let b = set_splat_m512(2.0);
/// let c: [f32; 16] = min_m512(a, b).into();
/// assert_eq!(c, [1.0_f32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_min_ps`]
/// * **Assembly:** `vminps zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn min_m512(a: m512, b: m512) -> m512 {
  m512(unsafe { _mm512_min_ps(a.0, b.0) })
}

/// Lanewise `min(a, b)` with lanes as `f64`.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(1.0);
/// let b = set_splat_m512d(2.0);
/// let c: [f64; 8] = min_m512d(a, b).into();
/// assert_eq!(c, [1.0_f64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_min_pd`]
/// * **Assembly:** `vminpd zmm, zmm, zmm`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn min_m512d(a: m512d, b: m512d) -> m512d {
  m512d(unsafe { _mm512_min_pd(a.0, b.0) })
}

// Masked load/store operations

/// Load `i8` values from memory using a mask.
/// ```
/// # use safe_arch::*;
/// let src = set_splat_i8_m512i(1);
/// let data = [5_i8; 64];
/// let mask = 0xFFFFFFFFFFFFFFFF;
/// let a: [i8; 64] = load_masked_i8_m512i(src, mask, &data).into();
/// assert_eq!(a, [5_i8; 64]);
/// ```
/// * **Intrinsic:** [`_mm512_mask_loadu_epi8`]
/// * **Assembly:** `vmovdqu8 zmm {k}, m512`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn load_masked_i8_m512i(src: m512i, mask: mmask64, mem_addr: &[i8; 64]) -> m512i {
  m512i(unsafe { _mm512_mask_loadu_epi8(src.0, mask, mem_addr.as_ptr() as *const i8) })
}

/// Load `i16` values from memory using a mask.
/// ```
/// # use safe_arch::*;
/// let src = set_splat_i16_m512i(1);
/// let data = [5_i16; 32];
/// let mask = 0xFFFFFFFF;
/// let a: [i16; 32] = load_masked_i16_m512i(src, mask, &data).into();
/// assert_eq!(a, [5_i16; 32]);
/// ```
/// * **Intrinsic:** [`_mm512_mask_loadu_epi16`]
/// * **Assembly:** `vmovdqu16 zmm {k}, m512`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn load_masked_i16_m512i(src: m512i, mask: mmask32, mem_addr: &[i16; 32]) -> m512i {
  m512i(unsafe { _mm512_mask_loadu_epi16(src.0, mask, mem_addr.as_ptr() as *const i16) })
}

/// Load `i32` values from memory using a mask.
/// ```
/// # use safe_arch::*;
/// let src = set_splat_i32_m512i(1);
/// let data = [5_i32; 16];
/// let mask = 0xFFFF;
/// let a: [i32; 16] = load_masked_i32_m512i(src, mask, &data).into();
/// assert_eq!(a, [5_i32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_mask_loadu_epi32`]
/// * **Assembly:** `vmovdqu32 zmm {k}, m512`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn load_masked_i32_m512i(src: m512i, mask: mmask16, mem_addr: &[i32; 16]) -> m512i {
  m512i(unsafe { _mm512_mask_loadu_epi32(src.0, mask, mem_addr.as_ptr() as *const i32) })
}

/// Load `f32` values from memory using a mask.
/// ```
/// # use safe_arch::*;
/// let src = set_splat_m512(1.0);
/// let data = [5.0_f32; 16];
/// let mask = 0xFFFF;
/// let a: [f32; 16] = load_masked_m512(src, mask, &data).into();
/// assert_eq!(a, [5.0_f32; 16]);
/// ```
/// * **Intrinsic:** [`_mm512_mask_loadu_ps`]
/// * **Assembly:** `vmovups zmm {k}, m512`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn load_masked_m512(src: m512, mask: mmask16, mem_addr: &[f32; 16]) -> m512 {
  m512(unsafe { _mm512_mask_loadu_ps(src.0, mask, mem_addr.as_ptr() as *const f32) })
}

/// Load `f64` values from memory using a mask.
/// ```
/// # use safe_arch::*;
/// let src = set_splat_m512d(1.0);
/// let data = [5.0_f64; 8];
/// let mask = 0xFF;
/// let a: [f64; 8] = load_masked_m512d(src, mask, &data).into();
/// assert_eq!(a, [5.0_f64; 8]);
/// ```
/// * **Intrinsic:** [`_mm512_mask_loadu_pd`]
/// * **Assembly:** `vmovupd zmm {k}, m512`
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn load_masked_m512d(src: m512d, mask: mmask8, mem_addr: &[f64; 8]) -> m512d {
    m512d(unsafe { _mm512_mask_loadu_pd(src.0, mask, mem_addr.as_ptr() as *const f64) })
}

/// Store `i8` values to memory using a mask.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i8_m512i(5);
/// let mut mem = [0_i8; 64];
/// let mask = 0xAAAAAAAAAAAAAAAA;
/// store_masked_i8_m512i(&mut mem, mask, a);
/// for (i, &val) in mem.iter().enumerate() {
///   assert_eq!(val, if (mask >> i) & 1 == 1 { 5 } else { 0 });
/// }
/// ```
/// * **Intrinsic:** [`_mm512_mask_storeu_epi8`]
/// * **Assembly:** `vmovdqu8 m512 {k}, zmm`
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn store_masked_i8_m512i(mem_addr: &mut [i8; 64], mask: mmask64, a: m512i) {
  unsafe { _mm512_mask_storeu_epi8(mem_addr.as_mut_ptr() as *mut i8, mask, a.0) }
}

/// Store `i16` values to memory using a mask.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i16_m512i(5);
/// let mut mem = [0_i16; 32];
/// let mask = 0xAAAAAAAA;
/// store_masked_i16_m512i(&mut mem, mask, a);
/// for (i, &val) in mem.iter().enumerate() {
///   assert_eq!(val, if (mask >> i) & 1 == 1 { 5 } else { 0 });
/// }
/// ```
/// * **Intrinsic:** [`_mm512_mask_storeu_epi16`]
/// * **Assembly:** `vmovdqu16 m512 {k}, zmm`
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512bw")))]
pub fn store_masked_i16_m512i(mem_addr: &mut [i16; 32], mask: mmask32, a: m512i) {
  unsafe { _mm512_mask_storeu_epi16(mem_addr.as_mut_ptr() as *mut i16, mask, a.0) }
}

/// Store `i32` values to memory using a mask.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i32_m512i(5);
/// let mut mem = [0_i32; 16];
/// let mask = 0xAAAA;
/// store_masked_i32_m512i(&mut mem, mask, a);
/// for (i, &val) in mem.iter().enumerate() {
///   assert_eq!(val, if (mask >> i) & 1 == 1 { 5 } else { 0 });
/// }
/// ```
/// * **Intrinsic:** [`_mm512_mask_storeu_epi32`]
/// * **Assembly:** `vmovdqu32 m512 {k}, zmm`
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn store_masked_i32_m512i(mem_addr: &mut [i32; 16], mask: mmask16, a: m512i) {
  unsafe { _mm512_mask_storeu_epi32(mem_addr.as_mut_ptr() as *mut i32, mask, a.0) }
}

/// Store `f32` values to memory using a mask.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(5.0);
/// let mut mem = [0.0_f32; 16];
/// let mask = 0xAAAA;
/// store_masked_m512(&mut mem, mask, a);
/// for (i, &val) in mem.iter().enumerate() {
///   assert_eq!(val, if (mask >> i) & 1 == 1 { 5.0 } else { 0.0 });
/// }
/// ```
/// * **Intrinsic:** [`_mm512_mask_storeu_ps`]
/// * **Assembly:** `vmovups m512 {k}, zmm`
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn store_masked_m512(mem_addr: &mut [f32; 16], mask: mmask16, a: m512) {
  unsafe { _mm512_mask_storeu_ps(mem_addr.as_mut_ptr() as *mut f32, mask, a.0) }
}

/// Store `f64` values to memory using a mask.
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(5.0);
/// let mut mem = [0.0_f64; 8];
/// let mask = 0b10101010;
/// store_masked_m512d(&mut mem, mask, a);
/// for (i, &val) in mem.iter().enumerate() {
///   assert_eq!(val, if (mask >> i) & 1 == 1 { 5.0 } else { 0.0 });
/// }
/// ```
/// * **Intrinsic:** [`_mm512_mask_storeu_pd`]
/// * **Assembly:** `vmovupd m512 {k}, zmm`
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn store_masked_m512d(mem_addr: &mut [f64; 8], mask: mmask8, a: m512d) {
    unsafe { _mm512_mask_storeu_pd(mem_addr.as_mut_ptr() as *mut f64, mask, a.0) }
}

// Prefetch operations

/// Prefetch data into L1 cache (temporal).
/// ```
/// # use safe_arch::*;
/// let data = [1.0_f32; 16];
/// prefetch_t0(&data);
/// ```
/// * **Intrinsic:** [`_mm_prefetch`]
/// * **Assembly:** `prefetcht0 m8`
#[inline(always)]
pub fn prefetch_t0<T>(p: &T) {
  unsafe { _mm_prefetch(p as *const T as *const i8, _MM_HINT_T0) }
}

/// Prefetch data into L1 cache (non-temporal).
/// ```
/// # use safe_arch::*;
/// let data = [1.0_f32; 16];
/// prefetch_et0(&data);
/// ```
/// * **Intrinsic:** [`_mm_prefetch`]
/// * **Assembly:** `prefetchnta m8`
#[inline(always)]
pub fn prefetch_et0<T>(p: &T) {
  unsafe { _mm_prefetch(p as *const T as *const i8, _MM_HINT_ET0) }
}

/// Lanewise `sqrt` on `f64` lanes.
/// ```
/// # use safe_arch::*;
/// let input = m512d::from([1.0_f64, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0, 64.0]);
/// let output: [f64; 8] = sqrt_m512d(input).into();
/// assert_eq!(output, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
/// ```
/// * **Intrinsic:** [`_mm512_sqrt_pd`]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn sqrt_m512d(a: m512d) -> m512d {
  m512d(unsafe { _mm512_sqrt_pd(a.0) })
}

/// Lanewise `sqrt` on `f32` lanes.
/// ```
/// # use safe_arch::*;
/// let input = m512::from([1.0_f32, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0, 64.0,
///                         81.0, 100.0, 121.0, 144.0, 169.0, 196.0, 225.0, 256.0]);
/// let output: [f32; 16] = sqrt_m512(input).into();
/// assert_eq!(output, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
///                     9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
/// ```
/// * **Intrinsic:** [`_mm512_sqrt_ps`]
/// * **Assembly:**
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn sqrt_m512(a: m512) -> m512 {
  m512(unsafe { _mm512_sqrt_ps(a.0) })
}

/// Cast from `m512i` to `m512` (reinterpret bits).
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i32_m512i(0x3F800000_i32); // 1.0f32 in bits
/// let b = cast_to_m512_from_m512i(a);
/// let arr: [f32; 16] = b.into();
/// assert_eq!(arr[0], 1.0_f32);
/// ```
/// * **Intrinsic:** [`_mm512_castsi512_ps`]
/// * **Assembly:** (no-op, just reinterpretation)
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cast_to_m512_from_m512i(a: m512i) -> m512 {
  unsafe { m512(_mm512_castsi512_ps(a.0)) }
}

/// Cast from `m512i` to `m512d` (reinterpret bits).
/// ```
/// # use safe_arch::*;
/// let a = set_splat_i64_m512i(0x3FF0000000000000_i64); // 1.0f64 in bits
/// let b = cast_to_m512d_from_m512i(a);
/// let arr: [f64; 8] = b.into();
/// assert_eq!(arr[0], 1.0_f64);
/// ```
/// * **Intrinsic:** [`_mm512_castsi512_pd`]
/// * **Assembly:** (no-op, just reinterpretation)
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cast_to_m512d_from_m512i(a: m512i) -> m512d {
  unsafe { m512d(_mm512_castsi512_pd(a.0)) }
}

/// Cast from `m512` to `m512i` (reinterpret bits).
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(1.0_f32);
/// let b = cast_to_m512i_from_m512(a);
/// let arr: [i32; 16] = b.into();
/// assert_eq!(arr[0], 0x3F800000_i32);
/// ```
/// * **Intrinsic:** [`_mm512_castps_si512`]
/// * **Assembly:** (no-op, just reinterpretation)
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cast_to_m512i_from_m512(a: m512) -> m512i {
  unsafe { m512i(_mm512_castps_si512(a.0)) }
}

/// Cast from `m512d` to `m512i` (reinterpret bits).
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(1.0_f64);
/// let b = cast_to_m512i_from_m512d(a);
/// let arr: [i64; 8] = b.into();
/// assert_eq!(arr[0], 0x3FF0000000000000_i64);
/// ```
/// * **Intrinsic:** [`_mm512_castpd_si512`]
/// * **Assembly:** (no-op, just reinterpretation)
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cast_to_m512i_from_m512d(a: m512d) -> m512i {
  unsafe { m512i(_mm512_castpd_si512(a.0)) }
}

/// Cast from `m512` to `m512d` (reinterpret bits).
/// Note: This does NOT convert float values to double values!
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512(1.0_f32);
/// let b = cast_to_m512d_from_m512(a);
/// // b now contains garbage values, not 1.0_f64!
/// ```
/// * **Intrinsic:** [`_mm512_castps_pd`]
/// * **Assembly:** (no-op, just reinterpretation)
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cast_to_m512d_from_m512(a: m512) -> m512d {
  unsafe { m512d(_mm512_castps_pd(a.0)) }
}

/// Cast from `m512d` to `m512` (reinterpret bits).
/// Note: This does NOT convert double values to float values!
/// ```
/// # use safe_arch::*;
/// let a = set_splat_m512d(1.0_f64);
/// let b = cast_to_m512_from_m512d(a);
/// // b now contains garbage values, not 1.0_f32!
/// ```
/// * **Intrinsic:** [`_mm512_castpd_ps`]
/// * **Assembly:** (no-op, just reinterpretation)
#[must_use]
#[inline(always)]
#[cfg_attr(docsrs, doc(cfg(target_feature = "avx512f")))]
pub fn cast_to_m512_from_m512d(a: m512d) -> m512 {
  unsafe { m512(_mm512_castpd_ps(a.0)) }
}

// m512i implementations
impl Not for m512i {
  type Output = Self;
  /// Not a direct intrinsic, but it's very useful and the implementation is
  /// simple enough.
  ///
  /// Negates the bits by performing an `xor` with an all-1s bit pattern.
  /// ```
  /// # use safe_arch::*;
  /// let a = m512i::from([0_u128, 0, 0, 0]);
  /// let c: [u128; 4] = (!a).into();
  /// assert_eq!(c, [u128::MAX, u128::MAX, u128::MAX, u128::MAX]);
  /// ```
  #[must_use]
  #[inline(always)]
  fn not(self) -> Self {
    let all_bits = set_splat_i16_m512i(-1);
    self ^ all_bits
  }
}

impl BitAnd for m512i {
  type Output = Self;
  /// ```
  /// # use safe_arch::*;
  /// let a = m512i::from([0_i64, 0, 1, 1, 0, 0, 1, 1]);
  /// let b = m512i::from([0_i64, 1, 0, 1, 0, 1, 0, 1]);
  /// let c: [i64; 8] = (a & b).into();
  /// assert_eq!(c, [0_i64, 0, 0, 1, 0, 0, 0, 1]);
  /// ```
  #[must_use]
  #[inline(always)]
  fn bitand(self, rhs: Self) -> Self {
    bitand_m512i(self, rhs)
  }
}
impl BitAndAssign for m512i {
  #[inline(always)]
  fn bitand_assign(&mut self, rhs: Self) {
    *self = *self & rhs;
  }
}

impl BitOr for m512i {
  type Output = Self;
  /// ```
  /// # use safe_arch::*;
  /// let a = m512i::from([0_i64, 0, 1, 1, 0, 0, 1, 1]);
  /// let b = m512i::from([0_i64, 1, 0, 1, 0, 1, 0, 1]);
  /// let c: [i64; 8] = (a | b).into();
  /// assert_eq!(c, [0_i64, 1, 1, 1, 0, 1, 1, 1]);
  /// ```
  #[must_use]
  #[inline(always)]
  fn bitor(self, rhs: Self) -> Self {
    bitor_m512i(self, rhs)
  }
}
impl BitOrAssign for m512i {
  #[inline(always)]
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}

impl BitXor for m512i {
  type Output = Self;
  /// ```
  /// # use safe_arch::*;
  /// let a = m512i::from([0_i64, 0, 1, 1, 0, 0, 1, 1]);
  /// let b = m512i::from([0_i64, 1, 0, 1, 0, 1, 0, 1]);
  /// let c: [i64; 8] = (a ^ b).into();
  /// assert_eq!(c, [0_i64, 1, 1, 0, 0, 1, 1, 0]);
  /// ```
  #[must_use]
  #[inline(always)]
  fn bitxor(self, rhs: Self) -> Self {
    bitxor_m512i(self, rhs)
  }
}
impl BitXorAssign for m512i {
  #[inline(always)]
  fn bitxor_assign(&mut self, rhs: Self) {
    *self = *self ^ rhs;
  }
}

impl PartialEq for m512i {
  #[must_use]
  #[inline(always)]
  /// ```
  /// # use safe_arch::*;
  /// let a = m512i::from([0_i64, 0, 1, 1, 0, 0, 1, 1]);
  /// let b = m512i::from([0_i64, 1, 0, 1, 0, 1, 0, 1]);
  /// assert_eq!(a, a);
  /// assert_ne!(a, b);
  /// ```
  fn eq(&self, other: &Self) -> bool {
    let mask = cmp_eq_mask_i8_m512i(*self, *other);
    mask == 0xFFFFFFFFFFFFFFFF_u64
  }
}
impl Eq for m512i {}

// m512 (f32) implementations
impl Not for m512 {
  type Output = Self;
  /// Bitwise NOT operation on `m512`.
  /// ```
  /// # use safe_arch::*;
  /// let a = m512::from([0.0_f32; 16]);
  /// let c = !a;
  /// // Note: This is a bitwise NOT, not a logical NOT
  /// ```
  #[must_use]
  #[inline(always)]
  fn not(self) -> Self {
    let all_bits = cast_to_m512_from_m512i(set_splat_i32_m512i(-1));
    self ^ all_bits
  }
}

impl BitAnd for m512 {
  type Output = Self;
  /// Bitwise AND operation on `m512`.
  /// ```
  /// # use safe_arch::*;
  /// let a = m512::from_bits([0xFFFFFFFF_u32; 16]);
  /// let b = m512::from_bits([0x00000000_u32; 16]);
  /// let c = a & b;
  /// assert_eq!(c.to_bits(), [0x00000000_u32; 16]);
  /// ```
  #[must_use]
  #[inline(always)]
  fn bitand(self, rhs: Self) -> Self {
    bitand_m512(self, rhs)
  }
}
impl BitAndAssign for m512 {
  #[inline(always)]
  fn bitand_assign(&mut self, rhs: Self) {
    *self = *self & rhs;
  }
}

impl BitOr for m512 {
  type Output = Self;
  /// Bitwise OR operation on `m512`.
  /// ```
  /// # use safe_arch::*;
  /// let a = m512::from_bits([0xFFFFFFFF_u32; 16]);
  /// let b = m512::from_bits([0x00000000_u32; 16]);
  /// let c = a | b;
  /// assert_eq!(c.to_bits(), [0xFFFFFFFF_u32; 16]);
  /// ```
  #[must_use]
  #[inline(always)]
  fn bitor(self, rhs: Self) -> Self {
    bitor_m512(self, rhs)
  }
}
impl BitOrAssign for m512 {
  #[inline(always)]
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}

impl BitXor for m512 {
  type Output = Self;
  /// Bitwise XOR operation on `m512`.
  /// ```
  /// # use safe_arch::*;
  /// let a = m512::from_bits([0xFFFFFFFF_u32; 16]);
  /// let b = m512::from_bits([0xFFFFFFFF_u32; 16]);
  /// let c = a ^ b;
  /// assert_eq!(c.to_bits(), [0x00000000_u32; 16]);
  /// ```
  #[must_use]
  #[inline(always)]
  fn bitxor(self, rhs: Self) -> Self {
    bitxor_m512(self, rhs)
  }
}
impl BitXorAssign for m512 {
  #[inline(always)]
  fn bitxor_assign(&mut self, rhs: Self) {
    *self = *self ^ rhs;
  }
}

impl PartialEq for m512 {
  #[must_use]
  #[inline(always)]
  /// ```
  /// # use safe_arch::*;
  /// let a = m512::from([1.0_f32; 16]);
  /// let b = m512::from([2.0_f32; 16]);
  /// assert_eq!(a, a);
  /// assert_ne!(a, b);
  /// ```
  fn eq(&self, other: &Self) -> bool {
    let mask = cmp_eq_mask_f32_m512(*self, *other);
    mask == 0xFFFF
  }
}

// m512d (f64) implementations
impl Not for m512d {
  type Output = Self;
  /// Bitwise NOT operation on `m512d`.
  /// ```
  /// # use safe_arch::*;
  /// let a = m512d::from([0.0_f64; 8]);
  /// let c = !a;
  /// // Note: This is a bitwise NOT, not a logical NOT
  /// ```
  #[must_use]
  #[inline(always)]
  fn not(self) -> Self {
    let all_bits = cast_to_m512d_from_m512i(set_splat_i64_m512i(-1));
    self ^ all_bits
  }
}

impl BitAnd for m512d {
  type Output = Self;
  /// Bitwise AND operation on `m512d`.
  /// ```
  /// # use safe_arch::*;
  /// let a = m512d::from_bits([0xFFFFFFFFFFFFFFFF_u64; 8]);
  /// let b = m512d::from_bits([0x0000000000000000_u64; 8]);
  /// let c = a & b;
  /// assert_eq!(c.to_bits(), [0x0000000000000000_u64; 8]);
  /// ```
  #[must_use]
  #[inline(always)]
  fn bitand(self, rhs: Self) -> Self {
    bitand_m512d(self, rhs)
  }
}
impl BitAndAssign for m512d {
  #[inline(always)]
  fn bitand_assign(&mut self, rhs: Self) {
    *self = *self & rhs;
  }
}

impl BitOr for m512d {
  type Output = Self;
  /// Bitwise OR operation on `m512d`.
  /// ```
  /// # use safe_arch::*;
  /// let a = m512d::from_bits([0xFFFFFFFFFFFFFFFF_u64; 8]);
  /// let b = m512d::from_bits([0x0000000000000000_u64; 8]);
  /// let c = a | b;
  /// assert_eq!(c.to_bits(), [0xFFFFFFFFFFFFFFFF_u64; 8]);
  /// ```
  #[must_use]
  #[inline(always)]
  fn bitor(self, rhs: Self) -> Self {
    bitor_m512d(self, rhs)
  }
}
impl BitOrAssign for m512d {
  #[inline(always)]
  fn bitor_assign(&mut self, rhs: Self) {
    *self = *self | rhs;
  }
}

impl BitXor for m512d {
  type Output = Self;
  /// Bitwise XOR operation on `m512d`.
  /// ```
  /// # use safe_arch::*;
  /// let a = m512d::from_bits([0xFFFFFFFFFFFFFFFF_u64; 8]);
  /// let b = m512d::from_bits([0xFFFFFFFFFFFFFFFF_u64; 8]);
  /// let c = a ^ b;
  /// assert_eq!(c.to_bits(), [0x0000000000000000_u64; 8]);
  /// ```
  #[must_use]
  #[inline(always)]
  fn bitxor(self, rhs: Self) -> Self {
    bitxor_m512d(self, rhs)
  }
}
impl BitXorAssign for m512d {
  #[inline(always)]
  fn bitxor_assign(&mut self, rhs: Self) {
    *self = *self ^ rhs;
  }
}

impl PartialEq for m512d {
  #[must_use]
  #[inline(always)]
  /// ```
  /// # use safe_arch::*;
  /// let a = m512d::from([1.0_f64; 8]);
  /// let b = m512d::from([2.0_f64; 8]);
  /// assert_eq!(a, a);
  /// assert_ne!(a, b);
  /// ```
  fn eq(&self, other: &Self) -> bool {
    let mask = cmp_eq_mask_f64_m512d(*self, *other);
    mask == 0xFF
  }
}