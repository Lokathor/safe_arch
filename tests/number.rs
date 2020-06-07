use safe_arch::*;

#[inline(always)]
fn atoi(x: [u8; 16]) -> u64 {
  let ascii_zero = set_splat_i8_m128i(b'0' as i8);
  let x: m128i = x.into();
  let x = sub_i8_m128i(x, ascii_zero);

  let tens = set_splat_i16_m128i(1 << 8 | 10);
  let x = mul_u8i8_add_horizontal_saturating_m128i(x, tens); /* eeee macarena! */

  let tens = set_splat_i32_m128i(1 << 16 | 100);
  let x = mul_i16_horizontal_add_m128i(x, tens);

  let tens = set_i16_m128i(0, 0, 0, 0, 1, 10000, 1, 10000);
  let x = pack_i32_to_u16_m128i(x, x);
  let x = mul_i16_horizontal_add_m128i(x, tens);

  let x: [u32; 4] = x.into();
  return x[1] as u64 + x[0] as u64 * 100000000;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn atoi_test() {
    assert_eq!(atoi(*b"1234567812345678"), 1234567812345678);
    assert_eq!(atoi(*b"0000000000000000"), 0000000000000000);
    assert_eq!(atoi(*b"1982379879823749"), 1982379879823749);
  }
}
