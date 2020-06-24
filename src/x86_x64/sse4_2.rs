#![cfg(target_feature = "sse4.2")]

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
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.2")))]
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
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.2")))]
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
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.2")))]
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
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.2")))]
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
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.2")))]
pub fn crc32_u64(crc: u64, v: u64) -> u64 {
  unsafe { _mm_crc32_u64(crc, v) }
}

/// Looks for `$needle` in `$haystack` and gives the index of the either the
/// first or last match.
///
/// > This is a fairly flexible operation, and so I apologize in advance.
///
/// * The "needle" is the string you're looking _for_.
/// * The "haystack" is the string you're looking _inside of_.
/// * The lengths of each string can be "explicit" or "implicit".
///   * "explicit" is specified with `[str, len]` pairs.
///   * "implicit" just ends at the first `\0`.
///   * Either way a string doesn't go past the end of the register.
/// * You need to pick a "char type", which can be any of `u8`, `i8`, `u16`,
///   `i16`. These operations always operate on `m128i` registers, but the
///   interpretation of the data is configurable.
/// * You need to pick the search operation, which determines how the `needle`
///   is compared to the `haystack`:
///   * `EqAny`: Matches when _any_ haystack character equals _any_ needle
///     character, regardless of position.
///   * `CmpRanges`: Interprets consecutive pairs of characters in the needle as
///     `(low..=high)` ranges to compare each haystack character to.
///   * `CmpEqEach`: Matches when a character position in the needle is equal to
///     the character at the same position in the haystack.
///   * `CmpEqOrdered`: Matches when the complete needle string is a substring
///     somewhere in the haystack.
/// * Finally, you need to specify if you're looking for the `FirstMatch` or
///   `LastMatch`.
///   * If no match is found the output will be the length of the haystack.
///
/// It's a lot to take in. Hopefully the examples below can help clarify how
/// things work. They all use `u8` since Rust string literals are UTF-8, but
/// it's the same with the other character types.
///
/// ## EqAny
/// ```
/// # use safe_arch::*;
/// let hay: m128i = m128i::from(*b"some test words.");
///
/// // explicit needle length
/// let needle: m128i = m128i::from(*b"e_______________");
/// let i: ::core::primitive::i32 =
///   string_search_for_index!([needle, 1], [hay, 16], u8, EqAny, FirstMatch);
/// assert_eq!(i, 3);
/// let i: ::core::primitive::i32 =
///   string_search_for_index!([needle, 1], [hay, 16], u8, EqAny, LastMatch);
/// assert_eq!(i, 6);
///
/// // implicit needle length
/// let needle: m128i = m128i::from(*b"e\0______________");
/// let i: ::core::primitive::i32 =
///   string_search_for_index!(needle, hay, u8, EqAny, FirstMatch);
/// assert_eq!(i, 3);
/// let i: ::core::primitive::i32 =
///   string_search_for_index!(needle, hay, u8, EqAny, LastMatch);
/// assert_eq!(i, 6);
///
/// // more than one needle character will match any of them, though we
/// // don't get info about _which_ needle character matched.
/// let needle: m128i = m128i::from(*b"et\0_____________");
/// let i: ::core::primitive::i32 =
///   string_search_for_index!(needle, hay, u8, EqAny, FirstMatch);
/// assert_eq!(i, 3);
/// let i: ::core::primitive::i32 =
///   string_search_for_index!(needle, hay, u8, EqAny, LastMatch);
/// assert_eq!(i, 8);
/// ```
/// ## CmpRanges
/// ```
/// # use safe_arch::*;
/// let hay: m128i = m128i::from(*b"some test words.");
/// let needle: m128i = m128i::from(*b"vz\0_____________");
/// let i: ::core::primitive::i32 =
///   string_search_for_index!(needle, hay, u8, CmpRanges, FirstMatch);
/// assert_eq!(i, 10); // matches the 'w'
/// ```
/// ## CmpEqEach
/// ```
/// # use safe_arch::*;
/// let hay: m128i = m128i::from(*b"some test words.");
/// let needle: m128i = m128i::from(*b"_____test_______");
/// let i: ::core::primitive::i32 =
///   string_search_for_index!(needle, hay, u8, CmpEqEach, FirstMatch);
/// assert_eq!(i, 5); // start of "test"
/// let i: ::core::primitive::i32 =
///   string_search_for_index!(needle, hay, u8, CmpEqEach, LastMatch);
/// assert_eq!(i, 8); // end of "test"
/// ```
/// ## CmpEqOrdered
/// ```
/// # use safe_arch::*;
/// let hay: m128i = m128i::from(*b"some test words.");
/// let needle: m128i = m128i::from(*b"words\0__________");
/// let i: ::core::primitive::i32 =
///   string_search_for_index!(needle, hay, u8, CmpEqOrdered, FirstMatch);
/// assert_eq!(i, 10); // This is where the "words" substring begins
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.2")))]
macro_rules! string_search_for_index {
  ([$needle:expr, $needle_len:expr], [$haystack:expr, $haystack_len:expr], $char_type:tt, $search_op:tt, $index_end:tt) => {{
    $crate::string_search_for_index!(
      @_raw_explicit_len
      $needle,
      $needle_len,
      $haystack,
      $haystack_len,
      $crate::string_search_for_index!(@_char_type $char_type)
      | $crate::string_search_for_index!(@_search_op $search_op)
      | $crate::string_search_for_index!(@_index_end $index_end)
    )
  }};

  ($needle:expr, $haystack:expr, $char_type:tt, $search_op:tt, $index_end:tt) => {{
    $crate::string_search_for_index!(
      @_raw_implicit_len
      $needle,
      $haystack,
      $crate::string_search_for_index!(@_char_type $char_type)
      | $crate::string_search_for_index!(@_search_op $search_op)
      | $crate::string_search_for_index!(@_index_end $index_end)
    )
  }};

  // Character types

  (@_char_type u8) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_UBYTE_OPS;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_UBYTE_OPS;
    _SIDD_UBYTE_OPS
  }};
  (@_char_type u16) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_UWORD_OPS;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_UWORD_OPS;
    _SIDD_UWORD_OPS
  }};
  (@_char_type i8) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_SBYTE_OPS;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_SBYTE_OPS;
    _SIDD_SBYTE_OPS
  }};
  (@_char_type i16) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_SWORD_OPS;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_SWORD_OPS;
    _SIDD_SWORD_OPS
  }};
  (@_char_type $unknown:tt) => {
    compile_error!("legal character types are: u8, u16, i8, i16")
  };

  // Search styles

  (@_search_op EqAny) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_CMP_EQUAL_ANY;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_CMP_EQUAL_ANY;
    _SIDD_CMP_EQUAL_ANY
  }};
  (@_search_op CmpRanges) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_CMP_RANGES;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_CMP_RANGES;
    _SIDD_CMP_RANGES
  }};
  (@_search_op CmpEqEach) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_CMP_EQUAL_EACH;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_CMP_EQUAL_EACH;
    _SIDD_CMP_EQUAL_EACH
  }};
  (@_search_op CmpEqOrdered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_CMP_EQUAL_ORDERED;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_CMP_EQUAL_ORDERED;
    _SIDD_CMP_EQUAL_ORDERED
  }};
  (@_search_op $unknown:tt) => {
    compile_error!(
      "legal search operations are: EqAny, CmpRanges, CmpEqEach, CmpEqOrdered"
    )
  };

  // Index end

  (@_index_end FirstMatch) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_LEAST_SIGNIFICANT;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_LEAST_SIGNIFICANT;
    _SIDD_LEAST_SIGNIFICANT
  }};
  (@_index_end LastMatch) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_MOST_SIGNIFICANT;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_MOST_SIGNIFICANT;
    _SIDD_MOST_SIGNIFICANT
  }};
  (@_index_end $unknown:tt) => {
    compile_error!("legal index args are: FirstMatch, LastMatch")
  };

  // The final, actual, calls to the intrinsic.

  (@_raw_explicit_len $needle:expr, $needle_len:expr, $haystack:expr, $haystack_len:expr, $imm:expr) => {{
    let a: m128i = $needle;
    let la: ::core::primitive::i32 = $needle_len;
    let b: m128i = $haystack;
    let lb: ::core::primitive::i32 = $haystack_len;
    const IMM: ::core::primitive::i32 = $imm as ::core::primitive::i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_cmpestri;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_cmpestri;
    unsafe { _mm_cmpestri(a.0, la, b.0, lb, IMM) }
  }};

  (@_raw_implicit_len $needle:expr, $haystack:expr, $imm:expr) => {{
    let a: m128i = $needle;
    let b: m128i = $haystack;
    const IMM: ::core::primitive::i32 = $imm as ::core::primitive::i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_cmpistri;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_cmpistri;
    unsafe { _mm_cmpistri(a.0, b.0, IMM) }
  }};
}

/// Looks for `$needle` in `$haystack` and gives the mask of where the matches
/// were.
///
/// > This is a fairly flexible operation, and so I apologize in advance.
///
/// * The "needle" is the string you're looking _for_.
/// * The "haystack" is the string you're looking _inside of_.
/// * The lengths of each string can be "explicit" or "implicit".
///   * "explicit" is specified with `[str, len]` pairs.
///   * "implicit" just ends at the first `\0`.
///   * Either way a string doesn't go past the end of the register.
/// * You need to pick a "char type", which can be any of `u8`, `i8`, `u16`,
///   `i16`. These operations always operate on `m128i` registers, but the
///   interpretation of the data is configurable.
/// * You need to pick the search operation, which determines how the `needle`
///   is compared to the `haystack`:
///   * `EqAny`: Matches when _any_ haystack character equals _any_ needle
///     character, regardless of position.
///   * `CmpRanges`: Interprets consecutive pairs of characters in the needle as
///     `(low..=high)` ranges to compare each haystack character to.
///   * `CmpEqEach`: Matches when a character position in the needle is equal to
///     the character at the same position in the haystack.
///   * `CmpEqOrdered`: Matches when the complete needle string is a substring
///     somewhere in the haystack.
/// * Finally, you need to specify if you want to have a `BitMask` or a
///   `UnitMask`.
///   * With a `BitMask`, each bit in the output will be set if there was a
///     match at that position of the haystack.
///   * In the `UnitMask` case, each "unit" in the output will be set if there's
///     a match at that position in the haystack. The size of a unit is set by
///     the "char type" you select (either 8 bits or 16 bits at a time).
///
/// It's a lot to take in. Hopefully the examples below can help clarify how
/// things work. They all use `u8` since Rust string literals are UTF-8, but
/// it's the same with the other character types.
///
/// ## EqAny
/// ```
/// # use safe_arch::*;
/// let hay: m128i = m128i::from(*b"some test words.");
///
/// // explicit needle length
/// let needle: m128i = m128i::from(*b"e_______________");
/// let i: u128 =
///   string_search_for_mask!([needle, 1], [hay, 16], u8, EqAny, BitMask).into();
/// assert_eq!(i, 0b0000000001001000);
/// let i: [i8; 16] =
///   string_search_for_mask!([needle, 1], [hay, 16], u8, EqAny, UnitMask).into();
/// assert_eq!(i, [0, 0, 0, -1, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
///
/// // implicit needle length
/// let needle: m128i = m128i::from(*b"e\0______________");
/// let i: u128 = string_search_for_mask!(needle, hay, u8, EqAny, BitMask).into();
/// assert_eq!(i, 0b0000000001001000);
///
/// // more than one needle character will match any of them, though we
/// // don't get info about _which_ needle character matched.
/// let needle: m128i = m128i::from(*b"et\0_____________");
/// let i: u128 = string_search_for_mask!(needle, hay, u8, EqAny, BitMask).into();
/// assert_eq!(i, 0b0000000101101000);
/// ```
/// ## CmpRanges
/// ```
/// # use safe_arch::*;
/// let hay: m128i = m128i::from(*b"some test words.");
/// let needle: m128i = m128i::from(*b"am\0_____________");
/// let i: u128 =
///   string_search_for_mask!(needle, hay, u8, CmpRanges, BitMask).into();
/// assert_eq!(i, 0b0010000001001100);
/// ```
/// ## CmpEqEach
/// ```
/// # use safe_arch::*;
/// let hay: m128i = m128i::from(*b"some test words.");
/// let needle: m128i = m128i::from(*b"_____test_______");
/// let i: u128 =
///   string_search_for_mask!(needle, hay, u8, CmpEqEach, BitMask).into();
/// assert_eq!(i, 0b0000000111100000);
/// ```
/// ## CmpEqOrdered
/// ```
/// # use safe_arch::*;
/// let hay: m128i = m128i::from(*b"some test words.");
/// let needle: m128i = m128i::from(*b"words\0__________");
/// let i: u128 =
///   string_search_for_mask!(needle, hay, u8, CmpEqOrdered, BitMask).into();
/// assert_eq!(i, 0b00000010000000000); // one bit at the start of the match
/// ```
#[macro_export]
#[cfg_attr(docs_rs, doc(cfg(target_feature = "sse4.2")))]
macro_rules! string_search_for_mask {
  ([$needle:expr, $needle_len:expr], [$haystack:expr, $haystack_len:expr], $char_type:tt, $search_op:tt, $mask_style:tt) => {{
    $crate::string_search_for_mask!(
      @_raw_explicit_len
      $needle,
      $needle_len,
      $haystack,
      $haystack_len,
      $crate::string_search_for_mask!(@_char_type $char_type)
      | $crate::string_search_for_mask!(@_search_op $search_op)
      | $crate::string_search_for_mask!(@_mask_style $mask_style)
    )
  }};

  ($needle:expr, $haystack:expr, $char_type:tt, $search_op:tt, $mask_style:tt) => {{
    $crate::string_search_for_mask!(
      @_raw_implicit_len
      $needle,
      $haystack,
      $crate::string_search_for_mask!(@_char_type $char_type)
      | $crate::string_search_for_mask!(@_search_op $search_op)
      | $crate::string_search_for_mask!(@_mask_style $mask_style)
    )
  }};

  // Character types

  (@_char_type u8) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_UBYTE_OPS;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_UBYTE_OPS;
    _SIDD_UBYTE_OPS
  }};
  (@_char_type u16) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_UWORD_OPS;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_UWORD_OPS;
    _SIDD_UWORD_OPS
  }};
  (@_char_type i8) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_SBYTE_OPS;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_SBYTE_OPS;
    _SIDD_SBYTE_OPS
  }};
  (@_char_type i16) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_SWORD_OPS;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_SWORD_OPS;
    _SIDD_SWORD_OPS
  }};
  (@_char_type $unknown:tt) => {
    compile_error!("legal character types are: u8, u16, i8, i16")
  };

  // Search styles

  (@_search_op EqAny) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_CMP_EQUAL_ANY;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_CMP_EQUAL_ANY;
    _SIDD_CMP_EQUAL_ANY
  }};
  (@_search_op CmpRanges) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_CMP_RANGES;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_CMP_RANGES;
    _SIDD_CMP_RANGES
  }};
  (@_search_op CmpEqEach) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_CMP_EQUAL_EACH;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_CMP_EQUAL_EACH;
    _SIDD_CMP_EQUAL_EACH
  }};
  (@_search_op CmpEqOrdered) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_CMP_EQUAL_ORDERED;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_CMP_EQUAL_ORDERED;
    _SIDD_CMP_EQUAL_ORDERED
  }};
  (@_search_op $unknown:tt) => {
    compile_error!(
      "legal search operations are: EqAny, CmpRanges, CmpEqEach, CmpEqOrdered"
    )
  };

  // Mask output style

  (@_mask_style BitMask) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_BIT_MASK;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_BIT_MASK;
    _SIDD_BIT_MASK
  }};
  (@_mask_style UnitMask) => {{
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_SIDD_UNIT_MASK;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_SIDD_UNIT_MASK;
    _SIDD_UNIT_MASK
  }};
  (@_mask_style $unknown:tt) => {
    compile_error!("legal str mask style are: BitMask, UnitMask")
  };

  // The final, actual, calls to the intrinsic.

  (@_raw_explicit_len $needle:expr, $needle_len:expr, $haystack:expr, $haystack_len:expr, $imm:expr) => {{
    let a: m128i = $needle;
    let la: ::core::primitive::i32 = $needle_len;
    let b: m128i = $haystack;
    let lb: ::core::primitive::i32 = $haystack_len;
    const IMM: ::core::primitive::i32 = $imm as ::core::primitive::i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_cmpestrm;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_cmpestrm;
    m128i(unsafe { _mm_cmpestrm(a.0, la, b.0, lb, IMM) })
  }};

  (@_raw_implicit_len $needle:expr, $haystack:expr, $imm:expr) => {{
    let a: m128i = $needle;
    let b: m128i = $haystack;
    const IMM: ::core::primitive::i32 = $imm as ::core::primitive::i32;
    #[cfg(target_arch = "x86")]
    use ::core::arch::x86::_mm_cmpistrm;
    #[cfg(target_arch = "x86_64")]
    use ::core::arch::x86_64::_mm_cmpistrm;
    m128i(unsafe { _mm_cmpistrm(a.0, b.0, IMM) })
  }};
}
