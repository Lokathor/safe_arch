#![cfg(target_feature = "sse4.1")]

use super::*;

/// Lanewise `a > b` with lanes as `i64`.
///
/// All bits 1 for true (`-1`), all bit 0 for false (`0`).
/// ```
/// # use safe_arch::*;
/// let a = m128i::from([1_i64, 3]);
/// let b = m128i::from([0_i64, 3]);
/// let c: [i64; 2] = cmp_gt_mask_i64_m128i(a, b).into();
/// assert_eq!(c, [-1_i64, 0]);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn cmp_gt_mask_i64_m128i(a: m128i, b: m128i) -> m128i {
  m128i(unsafe { _mm_cmpgt_epi64(a.0, b.0) })
}

/// Accumulates the `u8` into a running CRC32 value.
/// ```
/// # use safe_arch::*;
/// assert_eq!(crc32_u8(u32::MAX, u8::MAX), 16777215_u32);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn crc32_u8(crc: u32, v: u8) -> u32 {
  unsafe { _mm_crc32_u8(crc, v) }
}

/// Accumulates the `u16` into a running CRC32 value.
/// ```
/// # use safe_arch::*;
/// assert_eq!(crc32_u16(u32::MAX, u16::MAX), 65535_u32);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn crc32_u16(crc: u32, v: u16) -> u32 {
  unsafe { _mm_crc32_u16(crc, v) }
}

/// Accumulates the `u32` into a running CRC32 value.
/// ```
/// # use safe_arch::*;
/// assert_eq!(crc32_u32(u32::MAX, u32::MAX), 0_u32);
/// ```
#[must_use]
#[inline(always)]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn crc32_u32(crc: u32, v: u32) -> u32 {
  unsafe { _mm_crc32_u32(crc, v) }
}

/// Accumulates the `u64` into a running CRC32 value.
///
/// Note that, unlike with the other `crc32` functions, the running CRC32 input
/// and output is a `u64` value _in addition_ to the new value to accumulate
/// being a `u64`.
///
/// ```
/// # use safe_arch::*;
/// assert_eq!(crc32_u64(u64::MAX, u64::MAX), 3080238136_u64);
/// ```
#[must_use]
#[inline(always)]
#[cfg(target_arch = "x86_64")]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
pub fn crc32_u64(crc: u64, v: u64) -> u64 {
  unsafe { _mm_crc32_u64(crc, v) }
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_e_str_a {
  ($needle:expr, $len_needle:expr, $haystack:expr, $len_haystack:expr, $imm:expr) => {{
    let a: m128i = $needle;
    let la: i32 = $len_needle;
    let b: m128i = $haystack;
    let lb: i32 = $len_haystack;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpestra;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpestra;
    unsafe { _mm_cmpestra(a.0, la, b.0, lb, IMM) }
  }};
}

/// This expands the string comparison types you can use into the appropriate
/// const.
#[doc(hidden)]
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! str_cmp_type {
  (@ u8) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_SIDD_UBYTE_OPS;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_SIDD_UBYTE_OPS;
    _SIDD_UBYTE_OPS
  }};
  (@ u16) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_SIDD_UWORD_OPS;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_SIDD_UWORD_OPS;
    _SIDD_UWORD_OPS
  }};
  (@ i8) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_SIDD_SBYTE_OPS;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_SIDD_SBYTE_OPS;
    _SIDD_SBYTE_OPS
  }};
  (@ i16) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_SIDD_SWORD_OPS;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_SIDD_SWORD_OPS;
    _SIDD_SWORD_OPS
  }};
  (@ $unknown:tt) => {
    compile_error!("legal str_cmp types are: u8, u16, i8, i16")
  };
}

/// This expands the string comparison operations you can do into the
/// appropriate const.
#[doc(hidden)]
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! str_cmp_op {
  (@ EqAny) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_SIDD_CMP_EQUAL_ANY;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_SIDD_CMP_EQUAL_ANY;
    _SIDD_CMP_EQUAL_ANY
  }};
  (@ CmpRanges) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_SIDD_CMP_RANGES;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_SIDD_CMP_RANGES;
    _SIDD_CMP_RANGES
  }};
  (@ CmpEqEach) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_SIDD_CMP_EQUAL_EACH;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_SIDD_CMP_EQUAL_EACH;
    _SIDD_CMP_EQUAL_EACH
  }};
  (@ CmpEqOrdered) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_SIDD_CMP_EQUAL_ORDERED;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_SIDD_CMP_EQUAL_ORDERED;
    _SIDD_CMP_EQUAL_ORDERED
  }};
  (@ $unknown:tt) => {
    compile_error!(
      "legal str_cmp ops are: EqAny, CmpRanges, CmpEqEach, CmpEqOrdered"
    )
  };
}

/// This expands the string comparison negations you can do into the appropriate
/// const.
#[doc(hidden)]
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! str_negation {
  (@ NegativePolarity) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_SIDD_NEGATIVE_POLARITY;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_SIDD_NEGATIVE_POLARITY;
    _SIDD_NEGATIVE_POLARITY
  }};
  (@ MaskedNegativePolarity) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_SIDD_MASKED_NEGATIVE_POLARITY;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_SIDD_MASKED_NEGATIVE_POLARITY;
    _SIDD_MASKED_NEGATIVE_POLARITY
  }};
  (@ $unknown:tt) => {
    compile_error!(
      "legal str negations are: NegativePolarity, MaskedNegativePolarity"
    )
  };
}

/// This expands the string comparison index types you can ask for.
#[doc(hidden)]
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! str_index {
  (@ LowIndex) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_SIDD_LEAST_SIGNIFICANT;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_SIDD_LEAST_SIGNIFICANT;
    _SIDD_LEAST_SIGNIFICANT
  }};
  (@ HighIndex) => {{
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_SIDD_MOST_SIGNIFICANT;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_SIDD_MOST_SIGNIFICANT;
    _SIDD_MOST_SIGNIFICANT
  }};
  (@ $unknown:tt) => {
    compile_error!("legal str index args are: LowIndex, HighIndex")
  };
}

/// String comparison with "was there any match at all?" output.
///
/// * Looks for `$needle` in `$haystack`, with explicit lengths for both.
/// * `$t`: one of `u8`, `u16`, `i8`, `i16`
/// * `$op`: one of `EqAny`, `CmpRanges`, `CmpEqEach`, `CmpEqOrdered`
/// * `$neg`: optional, one of `NegativePolarity`, `MaskedNegativePolarity`
///
/// The output itself is 1 for true and 0 for false.
///
/// ```
/// # use safe_arch::*;
/// let haystack: m128i = m128i::from(*b"some test words.");
///
/// // using `EqAny`, we can find an 'w' anywhere in `haystack`
/// let needle: m128i = m128i::from(*b"w_______________");
/// assert_eq!(1, cmp_e_str_c!(needle, 1, haystack, 16, u8, EqAny));
/// // but if we limit the length of `haystack` it's not found.
/// assert_eq!(0, cmp_e_str_c!(needle, 1, haystack, 5, u8, EqAny));
///
/// // using `CmpRanges`, `needle` is range pair to check for in `haystack`.
/// let needle: m128i = m128i::from(*b"az______________");
/// assert_eq!(1, cmp_e_str_c!(needle, 2, haystack, 16, u8, CmpRanges));
/// let needle: m128i = m128i::from(*b"AZ______________");
/// assert_eq!(0, cmp_e_str_c!(needle, 2, haystack, 16, u8, CmpRanges));
///
/// // using `CmpEqEach`, see if the start of `needle` partly matches
/// // the start of `haystack`.
/// let needle: m128i = m128i::from(*b"som.____________");
/// assert_eq!(1, cmp_e_str_c!(needle, 4, haystack, 16, u8, CmpEqEach));
/// // but a match farther into the string doesn't count.
/// let needle: m128i = m128i::from(*b"test____________");
/// assert_eq!(0, cmp_e_str_c!(needle, 4, haystack, 16, u8, CmpEqEach));
///
/// // using `CmpEqOrdered`, the `needle` substring-search needs to
/// // have a full sub-string match to trigger a hit.
/// let needle: m128i = m128i::from(*b"som.____________");
/// assert_eq!(0, cmp_e_str_c!(needle, 4, haystack, 16, u8, CmpEqOrdered));
/// // this time "test" sub-string is found somewhere in `haystack`.
/// let needle: m128i = m128i::from(*b"test____________");
/// assert_eq!(1, cmp_e_str_c!(needle, 4, haystack, 16, u8, CmpEqOrdered));
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_e_str_c {
  ($needle:expr, $len_needle:expr, $haystack:expr, $len_haystack:expr, $t:tt, $op:tt) => {{
    $crate::cmp_e_str_c!(
      @ $needle, $len_needle, $haystack, $len_haystack,
      $crate::str_cmp_type!(@ $t)
      | $crate::str_cmp_op!(@ $op)
    )
  }};
  ($needle:expr, $len_needle:expr, $haystack:expr, $len_haystack:expr, $t:tt, $op:tt, $neg:tt) => {{
    $crate::cmp_e_str_c!(
      @ $needle, $len_needle, $haystack, $len_haystack,
      $crate::str_cmp_type!(@ $t)
      | $crate::str_cmp_op!(@ $op)
      | $crate::str_negation!(@ $neg)
    )
  }};
  (@ $needle:expr, $len_needle:expr, $haystack:expr, $len_haystack:expr, $imm:expr) => {{
    let a: m128i = $needle;
    let la: i32 = $len_needle;
    let b: m128i = $haystack;
    let lb: i32 = $len_haystack;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpestrc;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpestrc;
    unsafe { _mm_cmpestrc(a.0, la, b.0, lb, IMM) }
  }};
}

/// String comparison with the index of the match returned.
///
/// * Looks for `$needle` in `$haystack`, with explicit lengths for both.
/// * `$t`: one of `u8`, `u16`, `i8`, `i16`
/// * `$op`: one of `EqAny`, `CmpRanges`, `CmpEqEach`, `CmpEqOrdered`
/// * `$i`: one of `LowIndex`, `HighIndex`
/// * `$neg`: optional, one of `NegativePolarity`, `MaskedNegativePolarity`
///
/// ```
/// # use safe_arch::*;
/// let haystack: m128i = m128i::from(*b"some test words.");
///
/// // using `EqAny`
/// let needle: m128i = m128i::from(*b"som_____________");
/// assert_eq!(1, cmp_e_str_i!(needle, 3, haystack, 16, u8, EqAny, LowIndex));
/// assert_eq!(1, cmp_e_str_i!(needle, 3, haystack, 16, u8, EqAny, HighIndex));
///
/// // TODO: CmpRanges
/// // TODO: CmpEqEach
/// // TODO: CmpEqOrdered
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_e_str_i {
  ($needle:expr, $len_needle:expr, $haystack:expr, $len_haystack:expr, $t:tt, $op:tt, $i:tt) => {{
    $crate::cmp_e_str_c!(
      @ $needle, $len_needle, $haystack, $len_haystack,
      $crate::str_cmp_type!(@ $t)
      | $crate::str_cmp_op!(@ $op)
      | $crate::str_index!(@ $i)
    )
  }};
  ($needle:expr, $len_needle:expr, $haystack:expr, $len_haystack:expr, $t:tt, $op:tt, $i:tt, $neg:tt) => {{
    $crate::cmp_e_str_c!(
      @ $needle, $len_needle, $haystack, $len_haystack,
      $crate::str_cmp_type!(@ $t)
      | $crate::str_cmp_op!(@ $op)
      | $crate::str_negation!(@ $neg)
      | $crate::str_index!(@ $i)
    )
  }};
  (@ $needle:expr, $len_needle:expr, $haystack:expr, $len_haystack:expr, $imm:expr) => {{
    let a: m128i = $needle;
    let la: i32 = $len_needle;
    let b: m128i = $haystack;
    let lb: i32 = $len_haystack;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpestri;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpestri;
    unsafe { _mm_cmpestri(a.0, la, b.0, lb, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_e_str_m {
  ($needle:expr, $len_needle:expr, $haystack:expr, $len_haystack:expr, $imm:expr) => {{
    let a: m128i = $needle;
    let la: i32 = $len_needle;
    let b: m128i = $haystack;
    let lb: i32 = $len_haystack;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpestrm;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpestrm;
    m128i(unsafe { _mm_cmpestrm(a.0, la, b.0, lb, IMM) })
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_e_str_o {
  ($needle:expr, $len_needle:expr, $haystack:expr, $len_haystack:expr, $imm:expr) => {{
    let a: m128i = $needle;
    let la: i32 = $len_needle;
    let b: m128i = $haystack;
    let lb: i32 = $len_haystack;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpestro;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpestro;
    unsafe { _mm_cmpestro(a.0, la, b.0, lb, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_e_str_s {
  ($needle:expr, $len_needle:expr, $haystack:expr, $len_haystack:expr, $imm:expr) => {{
    let a: m128i = $needle;
    let la: i32 = $len_needle;
    let b: m128i = $haystack;
    let lb: i32 = $len_haystack;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpestrs;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpestrs;
    unsafe { _mm_cmpestrs(a.0, la, b.0, lb, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_e_str_z {
  ($needle:expr, $len_needle:expr, $haystack:expr, $len_haystack:expr, $imm:expr) => {{
    let a: m128i = $needle;
    let la: i32 = $len_needle;
    let b: m128i = $haystack;
    let lb: i32 = $len_haystack;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpestrz;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpestrz;
    unsafe { _mm_cmpestrz(a.0, la, b.0, lb, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_i_str_a {
  ($needle:expr, $haystack:expr, $imm:expr) => {{
    let a: m128i = $needle;
    let b: m128i = $haystack;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpistra;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpistra;
    unsafe { _mm_cmpistra(a.0, b.0, IMM) }
  }};
}

/// Works like [`cmp_e_str_c`], but with implicit string lengths
/// (null-terminated).
///
/// See that macro's for more info.
///
/// ```
/// # use safe_arch::*;
/// let haystack: m128i = m128i::from(*b"some test words.");
///
/// // using `EqAny`
/// let needle: m128i = m128i::from(*b"w\0______________");
/// assert_eq!(1, cmp_i_str_c!(needle, haystack, u8, EqAny));
/// assert_eq!(
///   0,
///   cmp_i_str_c!(needle, m128i::from(*b"som\0 test words."), u8, EqAny)
/// );
///
/// // using `CmpRanges`
/// let needle: m128i = m128i::from(*b"az\0_____________");
/// assert_eq!(1, cmp_i_str_c!(needle, haystack, u8, CmpRanges));
/// let needle: m128i = m128i::from(*b"AZ\0_____________");
/// assert_eq!(0, cmp_i_str_c!(needle, haystack, u8, CmpRanges));
///
/// // using `CmpEqEach`
/// let needle: m128i = m128i::from(*b"som.\0___________");
/// assert_eq!(1, cmp_i_str_c!(needle, haystack, u8, CmpEqEach));
/// let needle: m128i = m128i::from(*b"test\0___________");
/// assert_eq!(0, cmp_i_str_c!(needle, haystack, u8, CmpEqEach));
///
/// // using `CmpEqOrdered`
/// let needle: m128i = m128i::from(*b"som.\0___________");
/// assert_eq!(0, cmp_i_str_c!(needle, haystack, u8, CmpEqOrdered));
/// let needle: m128i = m128i::from(*b"test\0___________");
/// assert_eq!(1, cmp_i_str_c!(needle, haystack, u8, CmpEqOrdered));
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_i_str_c {
  ($needle:expr, $haystack:expr, $t:tt, $op:tt) => {{
    $crate::cmp_i_str_c!(
      @ $needle, $haystack,
      $crate::str_cmp_type!(@ $t)
      | $crate::str_cmp_op!(@ $op)
    )
  }};
  ($needle:expr, $haystack:expr, $t:tt, $op:tt, $neg:tt) => {{
    $crate::cmp_i_str_c!(
      @ $needle, $haystack,
      $crate::str_cmp_type!(@ $t)
      | $crate::str_cmp_op!(@ $op)
      | $crate::str_negation!(@ $neg)
    )
  }};
  (@ $needle:expr, $haystack:expr, $imm:expr) => {{
    let a: m128i = $needle;
    let b: m128i = $haystack;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpistrc;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpistrc;
    unsafe { _mm_cmpistrc(a.0, b.0, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_i_str_i {
  ($needle:expr, $haystack:expr, $imm:expr) => {{
    let a: m128i = $needle;
    let b: m128i = $haystack;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpistri;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpistri;
    unsafe { _mm_cmpistri(a.0, b.0, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_i_str_m {
  ($needle:expr, $haystack:expr, $imm:expr) => {{
    let a: m128i = $needle;
    let b: m128i = $haystack;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpistrm;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpistrm;
    m128i(unsafe { _mm_cmpistrm(a.0, b.0, IMM) })
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_i_str_o {
  ($needle:expr, $haystack:expr, $imm:expr) => {{
    let a: m128i = $needle;
    let b: m128i = $haystack;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpistro;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpistro;
    unsafe { _mm_cmpistro(a.0, b.0, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_i_str_s {
  ($needle:expr, $haystack:expr, $imm:expr) => {{
    let a: m128i = $needle;
    let b: m128i = $haystack;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpistrs;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpistrs;
    unsafe { _mm_cmpistrs(a.0, b.0, IMM) }
  }};
}

/// ?
///
/// ```
/// # use safe_arch::*;
/// //
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.1")))]
macro_rules! cmp_i_str_z {
  ($needle:expr, $haystack:expr, $imm:expr) => {{
    let a: m128i = $needle;
    let b: m128i = $haystack;
    const IMM: i32 = $imm as i32;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::_mm_cmpistrz;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::_mm_cmpistrz;
    unsafe { _mm_cmpistrz(a.0, b.0, IMM) }
  }};
}
