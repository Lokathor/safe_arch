#![cfg(target_feature = "sse4.1")]

use core::convert::TryFrom;

use safe_arch::*;

// Notes on how I think this stuff is supposed to work:
// https://hackmd.io/@ai5OEYRrSJKhrJP85tjTZQ/BkslfpW98

#[allow(unused_imports)]
#[cfg(target_arch = "x86")]
use core::arch::x86::{
  _SIDD_BIT_MASK, _SIDD_CMP_EQUAL_ANY, _SIDD_CMP_EQUAL_EACH,
  _SIDD_CMP_EQUAL_ORDERED, _SIDD_CMP_RANGES, _SIDD_LEAST_SIGNIFICANT,
  _SIDD_MASKED_NEGATIVE_POLARITY, _SIDD_MOST_SIGNIFICANT,
  _SIDD_NEGATIVE_POLARITY, _SIDD_SBYTE_OPS, _SIDD_SWORD_OPS, _SIDD_UBYTE_OPS,
  _SIDD_UNIT_MASK, _SIDD_UWORD_OPS,
};
#[allow(unused_imports)]
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
  _SIDD_BIT_MASK, _SIDD_CMP_EQUAL_ANY, _SIDD_CMP_EQUAL_EACH,
  _SIDD_CMP_EQUAL_ORDERED, _SIDD_CMP_RANGES, _SIDD_LEAST_SIGNIFICANT,
  _SIDD_MASKED_NEGATIVE_POLARITY, _SIDD_MOST_SIGNIFICANT,
  _SIDD_NEGATIVE_POLARITY, _SIDD_SBYTE_OPS, _SIDD_SWORD_OPS, _SIDD_UBYTE_OPS,
  _SIDD_UNIT_MASK, _SIDD_UWORD_OPS,
};

#[test]
fn foo() {
  let a_bytes: [u8; 16] = <[u8; 16]>::try_from(*b"this is a testXX").unwrap();
  let b_bytes: [u8; 16] = <[u8; 16]>::try_from(*b"this is a testXX").unwrap();
  let a: m128i = m128i::from(a_bytes);
  let b: m128i = m128i::from(b_bytes);
  const MODE: i32 = _SIDD_UWORD_OPS | _SIDD_CMP_EQUAL_EACH;
  dbg!(cmp_e_str_a!(a, 5, b, 5, MODE));
  dbg!(cmp_e_str_c!(a, 5, b, 5, MODE));
  dbg!(cmp_e_str_i!(a, 5, b, 5, MODE));
  dbg!(cmp_e_str_m!(a, 5, b, 5, MODE));
  dbg!(cmp_e_str_o!(a, 5, b, 5, MODE));
  dbg!(cmp_e_str_s!(a, 5, b, 5, MODE));
  dbg!(cmp_e_str_z!(a, 5, b, 5, MODE));
  //panic!();
}
