use super::*;

#[test]
fn test_add_m512() {
  let a = m512::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
  let b = m512::from_array([16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
  let c = add_m512(a, b).to_array();
  assert_eq!(c, [17.0; 16]);
}

#[test]
fn test_add_m512d() {
  let a = m512d::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  let b = m512d::from_array([8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
  let c = add_m512d(a, b).to_array();
  assert_eq!(c, [9.0; 8]);
}

#[test]
fn test_add_i8_m512i() {
  let a = m512i::from([5_i8; 64]);
  let b = m512i::from([10_i8; 64]);
  let c: [i8; 64] = add_i8_m512i(a, b).into();
  assert_eq!(c, [15_i8; 64]);
}

#[test]
fn test_add_i16_m512i() {
  let a = m512i::from([5_i16; 32]);
  let b = m512i::from([10_i16; 32]);
  let c: [i16; 32] = add_i16_m512i(a, b).into();
  assert_eq!(c, [15_i16; 32]);
}

#[test]
fn test_add_i32_m512i() {
  let a = m512i::from([5_i32; 16]);
  let b = m512i::from([10_i32; 16]);
  let c: [i32; 16] = add_i32_m512i(a, b).into();
  assert_eq!(c, [15_i32; 16]);
}

#[test]
fn test_add_i64_m512i() {
  let a = m512i::from([5_i64; 8]);
  let b = m512i::from([10_i64; 8]);
  let c: [i64; 8] = add_i64_m512i(a, b).into();
  assert_eq!(c, [15_i64; 8]);
}

#[test]
fn test_sub_m512() {
  let a = m512::from_array([16.0; 16]);
  let b = m512::from_array([1.0; 16]);
  let c = sub_m512(a, b).to_array();
  assert_eq!(c, [15.0; 16]);
}

#[test]
fn test_sub_m512d() {
  let a = m512d::from_array([16.0; 8]);
  let b = m512d::from_array([1.0; 8]);
  let c = sub_m512d(a, b).to_array();
  assert_eq!(c, [15.0; 8]);
}

#[test]
fn test_sub_i8_m512i() {
  let a = m512i::from([20_i8; 64]);
  let b = m512i::from([5_i8; 64]);
  let c: [i8; 64] = sub_i8_m512i(a, b).into();
  assert_eq!(c, [15_i8; 64]);
}

#[test]
fn test_sub_i16_m512i() {
  let a = m512i::from([20_i16; 32]);
  let b = m512i::from([5_i16; 32]);
  let c: [i16; 32] = sub_i16_m512i(a, b).into();
  assert_eq!(c, [15_i16; 32]);
}

#[test]
fn test_sub_i32_m512i() {
  let a = m512i::from([20_i32; 16]);
  let b = m512i::from([5_i32; 16]);
  let c: [i32; 16] = sub_i32_m512i(a, b).into();
  assert_eq!(c, [15_i32; 16]);
}

#[test]
fn test_sub_i64_m512i() {
  let a = m512i::from([20_i64; 8]);
  let b = m512i::from([5_i64; 8]);
  let c: [i64; 8] = sub_i64_m512i(a, b).into();
  assert_eq!(c, [15_i64; 8]);
}

#[test]
fn test_mul_m512() {
  let a = m512::from_array([2.0; 16]);
  let b = m512::from_array([3.0; 16]);
  let c = mul_m512(a, b).to_array();
  assert_eq!(c, [6.0; 16]);
}

#[test]
fn test_mul_m512d() {
  let a = m512d::from_array([2.0; 8]);
  let b = m512d::from_array([3.0; 8]);
  let c = mul_m512d(a, b).to_array();
  assert_eq!(c, [6.0; 8]);
}

#[test]
fn test_mul_i16_keep_low_m512i() {
  let a = m512i::from([5_i16; 32]);
  let b = m512i::from([3_i16; 32]);
  let c: [i16; 32] = mul_i16_keep_low_m512i(a, b).into();
  assert_eq!(c, [15_i16; 32]);
}

#[test]
fn test_mul_i32_keep_low_m512i() {
  let a = m512i::from([5_i32; 16]);
  let b = m512i::from([3_i32; 16]);
  let c: [i32; 16] = mul_i32_keep_low_m512i(a, b).into();
  assert_eq!(c, [15_i32; 16]);
}

#[test]
fn test_div_m512() {
  let a = m512::from_array([12.0; 16]);
  let b = m512::from_array([3.0; 16]);
  let c = div_m512(a, b).to_array();
  assert_eq!(c, [4.0; 16]);
}

#[test]
fn test_div_m512d() {
  let a = m512d::from_array([12.0; 8]);
  let b = m512d::from_array([3.0; 8]);
  let c = div_m512d(a, b).to_array();
  assert_eq!(c, [4.0; 8]);
}

#[test]
fn test_fmadd_m512() {
  let a = m512::from_array([2.0; 16]);
  let b = m512::from_array([3.0; 16]);
  let c = m512::from_array([1.0; 16]);
  let d = fmadd_m512(a, b, c).to_array();
  assert_eq!(d, [7.0; 16]);
}

#[test]
fn test_fmadd_m512d() {
  let a = m512d::from_array([2.0; 8]);
  let b = m512d::from_array([3.0; 8]);
  let c = m512d::from_array([1.0; 8]);
  let d = fmadd_m512d(a, b, c).to_array();
  assert_eq!(d, [7.0; 8]);
}

#[test]
fn test_max_m512() {
  let a = m512::from_array([1.0, 5.0, 3.0, 7.0, 2.0, 6.0, 4.0, 8.0, 9.0, 13.0, 11.0, 15.0, 10.0, 14.0, 12.0, 16.0]);
  let b = m512::from_array([2.0, 4.0, 6.0, 8.0, 1.0, 3.0, 5.0, 7.0, 10.0, 12.0, 14.0, 16.0, 9.0, 11.0, 13.0, 15.0]);
  let c = max_m512(a, b).to_array();
  assert_eq!(c, [2.0, 5.0, 6.0, 8.0, 2.0, 6.0, 5.0, 8.0, 10.0, 13.0, 14.0, 16.0, 10.0, 14.0, 13.0, 16.0]);
}

#[test]
fn test_max_m512d() {
  let a = m512d::from_array([1.0, 5.0, 3.0, 7.0, 2.0, 6.0, 4.0, 8.0]);
  let b = m512d::from_array([2.0, 4.0, 6.0, 8.0, 1.0, 3.0, 5.0, 7.0]);
  let c = max_m512d(a, b).to_array();
  assert_eq!(c, [2.0, 5.0, 6.0, 8.0, 2.0, 6.0, 5.0, 8.0]);
}

#[test]
fn test_min_m512() {
  let a = m512::from_array([1.0, 5.0, 3.0, 7.0, 2.0, 6.0, 4.0, 8.0, 9.0, 13.0, 11.0, 15.0, 10.0, 14.0, 12.0, 16.0]);
  let b = m512::from_array([2.0, 4.0, 6.0, 8.0, 1.0, 3.0, 5.0, 7.0, 10.0, 12.0, 14.0, 16.0, 9.0, 11.0, 13.0, 15.0]);
  let c = min_m512(a, b).to_array();
  assert_eq!(c, [1.0, 4.0, 3.0, 7.0, 1.0, 3.0, 4.0, 7.0, 9.0, 12.0, 11.0, 15.0, 9.0, 11.0, 12.0, 15.0]);
}

#[test]
fn test_min_m512d() {
  let a = m512d::from_array([1.0, 5.0, 3.0, 7.0, 2.0, 6.0, 4.0, 8.0]);
  let b = m512d::from_array([2.0, 4.0, 6.0, 8.0, 1.0, 3.0, 5.0, 7.0]);
  let c = min_m512d(a, b).to_array();
  assert_eq!(c, [1.0, 4.0, 3.0, 7.0, 1.0, 3.0, 4.0, 7.0]);
}

#[test]
fn test_sqrt_m512() {
  let a = m512::from_array([1.0, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0, 64.0, 81.0, 100.0, 121.0, 144.0, 169.0, 196.0, 225.0, 256.0]);
  let b = sqrt_m512(a).to_array();
  assert_eq!(b, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
}

#[test]
fn test_sqrt_m512d() {
  let a = m512d::from_array([1.0, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0, 64.0]);
  let b = sqrt_m512d(a).to_array();
  assert_eq!(b, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
}

#[test]
fn test_bitand_m512i() {
  let a = m512i::from([0b1111_i64; 8]);
  let b = m512i::from([0b1010_i64; 8]);
  let c: [i64; 8] = bitand_m512i(a, b).into();
  assert_eq!(c, [0b1010_i64; 8]);
}

#[test]
fn test_bitand_m512() {
  let a = m512::from_bits([u32::MAX; 16]);
  let b = m512::from_bits([0; 16]);
  let c = bitand_m512(a, b).to_bits();
  assert_eq!(c, [0; 16]);
}

#[test]
fn test_bitand_m512d() {
  let a = m512d::from_bits([u64::MAX; 8]);
  let b = m512d::from_bits([0; 8]);
  let c = bitand_m512d(a, b).to_bits();
  assert_eq!(c, [0; 8]);
}

#[test]
fn test_bitandnot_m512i() {
  let a = m512i::from([0b1010_i64; 8]);
  let b = m512i::from([0b1111_i64; 8]);
  let c: [i64; 8] = bitandnot_m512i(a, b).into();
  assert_eq!(c, [0b0101_i64; 8]);
}

#[test]
fn test_bitandnot_m512() {
  let a = m512::from_bits([0xAAAAAAAA_u32; 16]);
  let b = m512::from_bits([0xFFFFFFFF_u32; 16]);
  let c = bitandnot_m512(a, b).to_bits();
  assert_eq!(c, [0x55555555_u32; 16]);
}

#[test]
fn test_bitandnot_m512d() {
  let a = m512d::from_bits([0xAAAAAAAAAAAAAAAA_u64; 8]);
  let b = m512d::from_bits([0xFFFFFFFFFFFFFFFF_u64; 8]);
  let c = bitandnot_m512d(a, b).to_bits();
  assert_eq!(c, [0x5555555555555555_u64; 8]);
}

#[test]
fn test_bitor_m512i() {
  let a = m512i::from([0b1010_i64; 8]);
  let b = m512i::from([0b0101_i64; 8]);
  let c: [i64; 8] = bitor_m512i(a, b).into();
  assert_eq!(c, [0b1111_i64; 8]);
}

#[test]
fn test_bitor_m512() {
  let a = m512::from_bits([0xAAAAAAAA_u32; 16]);
  let b = m512::from_bits([0x55555555_u32; 16]);
  let c = bitor_m512(a, b).to_bits();
  assert_eq!(c, [0xFFFFFFFF_u32; 16]);
}

#[test]
fn test_bitor_m512d() {
  let a = m512d::from_bits([0xAAAAAAAAAAAAAAAA_u64; 8]);
  let b = m512d::from_bits([0x5555555555555555_u64; 8]);
  let c = bitor_m512d(a, b).to_bits();
  assert_eq!(c, [0xFFFFFFFFFFFFFFFF_u64; 8]);
}

#[test]
fn test_bitxor_m512i() {
  let a = m512i::from([0b1111_i64; 8]);
  let b = m512i::from([0b1010_i64; 8]);
  let c: [i64; 8] = bitxor_m512i(a, b).into();
  assert_eq!(c, [0b0101_i64; 8]);
}

#[test]
fn test_bitxor_m512() {
  let a = m512::from_bits([0xFFFFFFFF_u32; 16]);
  let b = m512::from_bits([0xAAAAAAAA_u32; 16]);
  let c = bitxor_m512(a, b).to_bits();
  assert_eq!(c, [0x55555555_u32; 16]);
}

#[test]
fn test_bitxor_m512d() {
  let a = m512d::from_bits([0xFFFFFFFFFFFFFFFF_u64; 8]);
  let b = m512d::from_bits([0xAAAAAAAAAAAAAAAA_u64; 8]);
  let c = bitxor_m512d(a, b).to_bits();
  assert_eq!(c, [0x5555555555555555_u64; 8]);
}

#[test]
fn test_cmp_eq_mask_i8_m512i() {
  let a = m512i::from([5_i8; 64]);
  let b = m512i::from([5_i8; 64]);
  let mask = cmp_eq_i8_mask_m512i(a, b);
  assert_eq!(mask, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_cmp_eq_mask_u8_m512i() {
  let a = m512i::from([5_u8; 64]);
  let b = m512i::from([6_u8; 64]);
  let mask = cmp_eq_u8_mask_m512i(a, b);
  assert_eq!(mask, 0);
}

#[test]
fn test_cmp_eq_mask_i16_m512i() {
  let a = m512i::from([5_i16; 32]);
  let b = m512i::from([5_i16; 32]);
  let mask = cmp_eq_i16_mask_m512i(a, b);
  assert_eq!(mask, 0xFFFFFFFF);
}

#[test]
fn test_cmp_eq_mask_i32_m512i() {
  let a = m512i::from([5_i32; 16]);
  let b = m512i::from([5_i32; 16]);
  let mask = cmp_eq_i32_mask_m512i(a, b);
  assert_eq!(mask, 0xFFFF);
}

#[test]
fn test_cmp_eq_mask_m512() {
  let a = m512::from_array([5.0; 16]);
  let b = m512::from_array([5.0; 16]);
  let mask = cmp_eq_mask_m512(a, b);
  assert_eq!(mask, 0xFFFF);
}

#[test]
fn test_cmp_eq_mask_m512d() {
  let a = m512d::from_array([5.0; 8]);
  let b = m512d::from_array([5.0; 8]);
  let mask = cmp_eq_mask_m512d(a, b);
  assert_eq!(mask, 0xFF);
}

#[test]
fn test_cmp_gt_mask_i8_m512i() {
  let a = m512i::from([10_i8; 64]);
  let b = m512i::from([5_i8; 64]);
  let mask = cmp_gt_i8_mask_m512i(a, b);
  assert_eq!(mask, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_cmp_gt_mask_u8_m512i() {
  let a = m512i::from([5_u8; 64]);
  let b = m512i::from([10_u8; 64]);
  let mask = cmp_gt_u8_mask_m512i(a, b);
  assert_eq!(mask, 0);
}

#[test]
fn test_cmp_gt_mask_i16_m512i() {
  let a = m512i::from([10_i16; 32]);
  let b = m512i::from([5_i16; 32]);
  let mask = cmp_gt_i16_mask_m512i(a, b);
  assert_eq!(mask, 0xFFFFFFFF);
}

#[test]
fn test_cmp_gt_mask_u16_m512i() {
  let a = m512i::from([5_u16; 32]);
  let b = m512i::from([10_u16; 32]);
  let mask = cmp_gt_u16_mask_m512i(a, b);
  assert_eq!(mask, 0);
}

#[test]
fn test_cmp_gt_mask_i32_m512i() {
  let a = m512i::from([10_i32; 16]);
  let b = m512i::from([5_i32; 16]);
  let mask = cmp_gt_i32_mask_m512i(a, b);
  assert_eq!(mask, 0xFFFF);
}

#[test]
fn test_cmp_gt_mask_m512() {
  let a = m512::from_array([10.0; 16]);
  let b = m512::from_array([5.0; 16]);
  let mask = cmp_gt_mask_m512(a, b);
  assert_eq!(mask, 0xFFFF);
}

#[test]
fn test_cmp_gt_mask_m512d() {
  let a = m512d::from_array([10.0; 8]);
  let b = m512d::from_array([5.0; 8]);
  let mask = cmp_gt_mask_m512d(a, b);
  assert_eq!(mask, 0xFF);
}

#[test]
fn test_cmp_ge_mask_i8_m512i() {
  let a = m512i::from([10_i8; 64]);
  let b = m512i::from([10_i8; 64]);
  let mask = cmp_ge_i8_mask_m512i(a, b);
  assert_eq!(mask, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_cmp_ge_mask_u8_m512i() {
  let a = m512i::from([10_u8; 64]);
  let b = m512i::from([10_u8; 64]);
  let mask = cmp_ge_u8_mask_m512i(a, b);
  assert_eq!(mask, 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_cmp_ge_mask_m512() {
  let a = m512::from_array([10.0; 16]);
  let b = m512::from_array([10.0; 16]);
  let mask = cmp_ge_mask_m512(a, b);
  assert_eq!(mask, 0xFFFF);
}

#[test]
fn test_cmp_ge_mask_m512d() {
  let a = m512d::from_array([10.0; 8]);
  let b = m512d::from_array([10.0; 8]);
  let mask = cmp_ge_mask_m512d(a, b);
  assert_eq!(mask, 0xFF);
}

#[test]
fn test_cmp_lt_mask_m512() {
  let a = m512::from_array([5.0; 16]);
  let b = m512::from_array([10.0; 16]);
  let mask = cmp_lt_mask_m512(a, b);
  assert_eq!(mask, 0xFFFF);
}

#[test]
fn test_cmp_lt_mask_m512d() {
  let a = m512d::from_array([5.0; 8]);
  let b = m512d::from_array([10.0; 8]);
  let mask = cmp_lt_mask_m512d(a, b);
  assert_eq!(mask, 0xFF);
}

#[test]
fn test_cmp_le_mask_m512() {
  let a = m512::from_array([10.0; 16]);
  let b = m512::from_array([10.0; 16]);
  let mask = cmp_le_mask_m512(a, b);
  assert_eq!(mask, 0xFFFF);
}

#[test]
fn test_cmp_le_mask_m512d() {
  let a = m512d::from_array([10.0; 8]);
  let b = m512d::from_array([10.0; 8]);
  let mask = cmp_le_mask_m512d(a, b);
  assert_eq!(mask, 0xFF);
}

#[test]
fn test_blend_varying_i8_m512i() {
  let a = m512i::from([10_i8; 64]);
  let b = m512i::from([20_i8; 64]);
  let mask = 0xAAAAAAAAAAAAAAAA;
  let c: [i8; 64] = blend_varying_i8_m512i(a, b, mask).into();
  for (i, &val) in c.iter().enumerate() {
    assert_eq!(val, if (mask >> i) & 1 == 1 { 20 } else { 10 });
  }
}

#[test]
fn test_blend_i16_m512i() {
  let a = m512i::from([10_i16; 32]);
  let b = m512i::from([20_i16; 32]);
  let mask = 0xAAAAAAAA;
  let c: [i16; 32] = blend_i16_m512i(a, b, mask).into();
  for (i, &val) in c.iter().enumerate() {
    assert_eq!(val, if (mask >> i) & 1 == 1 { 20 } else { 10 });
  }
}

#[test]
fn test_blend_i32_m512i() {
  let a = m512i::from([10_i32; 16]);
  let b = m512i::from([20_i32; 16]);
  let mask = 0xAAAA;
  let c: [i32; 16] = blend_i32_m512i(a, b, mask).into();
  for (i, &val) in c.iter().enumerate() {
    assert_eq!(val, if (mask >> i) & 1 == 1 { 20 } else { 10 });
  }
}

#[test]
fn test_blend_m512() {
  let a = m512::from_array([10.0; 16]);
  let b = m512::from_array([20.0; 16]);
  let mask = 0xAAAA;
  let c = blend_m512(a, b, mask).to_array();
  for (i, &val) in c.iter().enumerate() {
    assert_eq!(val, if (mask >> i) & 1 == 1 { 20.0 } else { 10.0 });
  }
}

#[test]
fn test_blend_m512d() {
  let a = m512d::from_array([10.0; 8]);
  let b = m512d::from_array([20.0; 8]);
  let mask = 0xAA;
  let c = blend_m512d(a, b, mask).to_array();
  for (i, &val) in c.iter().enumerate() {
    assert_eq!(val, if (mask >> i) & 1 == 1 { 20.0 } else { 10.0 });
  }
}

#[test]
fn test_convert_to_i16_m512i_from_i8_m256i() {
  let a = m256i::from([-5_i8; 32]);
  let b: [i16; 32] = convert_to_i16_m512i_from_i8_m256i(a).into();
  assert_eq!(b, [-5_i16; 32]);
}

#[test]
fn test_convert_to_i16_m512i_from_u8_m256i() {
  let a = m256i::from([5_u8; 32]);
  let b: [i16; 32] = convert_to_i16_m512i_from_u8_m256i(a).into();
  assert_eq!(b, [5_i16; 32]);
}

#[test]
fn test_convert_to_i32_m512i_from_i16_m256i() {
  let a = m256i::from([-5_i16; 16]);
  let b: [i32; 16] = convert_to_i32_m512i_from_i16_m256i(a).into();
  assert_eq!(b, [-5_i32; 16]);
}

#[test]
fn test_convert_to_i8_m256i_from_i16_m512i() {
  let a = m512i::from([5_i16; 32]);
  let b: [i8; 32] = convert_to_i8_m256i_from_i16_m512i(a).into();
  assert_eq!(b, [5_i8; 32]);
}

#[test]
fn test_convert_m512_i32_m512i() {
  let a = m512::from_array([5.5; 16]);
  let b: [i32; 16] = convert_m512_i32_m512i(a).into();
  assert_eq!(b, [6_i32; 16]);
}

#[test]
fn test_convert_m512d_i64_m512i() {
  let a = m512d::from_array([5.5; 8]);
  let b: [i64; 8] = convert_m512d_i64_m512i(a).into();
  assert_eq!(b, [6_i64; 8]);
}

#[test]
fn test_convert_truncate_m512_i32_m512i() {
  let a = m512::from_array([5.9; 16]);
  let b: [i32; 16] = convert_truncate_m512_i32_m512i(a).into();
  assert_eq!(b, [5_i32; 16]);
}

#[test]
fn test_convert_truncate_m512d_i64_m512i() {
  let a = m512d::from_array([5.9; 8]);
  let b: [i64; 8] = convert_truncate_m512d_i64_m512i(a).into();
  assert_eq!(b, [5_i64; 8]);
}

#[test]
fn test_pack_i32_to_i16_m512i() {
  let a = m512i::from([1_i32; 16]);
  let b = m512i::from([2_i32; 16]);
  let c: [i16; 32] = pack_i32_to_i16_m512i(a, b).into();
  let expected: Vec<i16> = vec![1; 4].into_iter()
    .chain(vec![2; 4])
    .chain(vec![1; 4])
    .chain(vec![2; 4])
    .chain(vec![1; 4])
    .chain(vec![2; 4])
    .chain(vec![1; 4])
    .chain(vec![2; 4])
    .collect();
  assert_eq!(&c[..], &expected[..]);
}

#[test]
fn test_pack_i16_to_u8_m512i() {
  let a = m512i::from([1_i16; 32]);
  let b = m512i::from([2_i16; 32]);
  let c: [u8; 64] = pack_i16_to_u8_m512i(a, b).into();
  let expected: Vec<u8> = vec![1; 8].into_iter()
    .chain(vec![2; 8])
    .chain(vec![1; 8])
    .chain(vec![2; 8])
    .chain(vec![1; 8])
    .chain(vec![2; 8])
    .chain(vec![1; 8])
    .chain(vec![2; 8])
    .collect();
  assert_eq!(&c[..], &expected[..]);
}

#[test]
fn test_unpack_high_i8_m512i() {
  let a = m512i::from([1_i8; 64]);
  let b = m512i::from([2_i8; 64]);
  let c: [i8; 64] = unpack_high_i8_m512i(a, b).into();
  // Due to the way unpacking works within 128-bit lanes, we expect alternating values
  // in the high half of each 128-bit segment
  for chunk in c.chunks(16) {
    for (i, &val) in chunk[8..].iter().enumerate() {
      assert_eq!(val, if i % 2 == 0 { 1 } else { 2 });
    }
  }
}

#[test]
fn test_unpack_high_i16_m512i() {
  let a = m512i::from([1_i16; 32]);
  let b = m512i::from([2_i16; 32]);
  let c: [i16; 32] = unpack_high_i16_m512i(a, b).into();
  // Similar pattern for i16
  for chunk in c.chunks(8) {
    for (i, &val) in chunk[4..].iter().enumerate() {
      assert_eq!(val, if i % 2 == 0 { 1 } else { 2 });
    }
  }
}

#[test]
fn test_unpack_low_i8_m512i() {
  let a = m512i::from([1_i8; 64]);
  let b = m512i::from([2_i8; 64]);
  let c: [i8; 64] = unpack_low_i8_m512i(a, b).into();
  // Low half of each 128-bit segment
  for chunk in c.chunks(16) {
    for (i, &val) in chunk[..8].iter().enumerate() {
      assert_eq!(val, if i % 2 == 0 { 1 } else { 2 });
    }
  }
}

#[test]
fn test_unpack_low_i16_m512i() {
  let a = m512i::from([1_i16; 32]);
  let b = m512i::from([2_i16; 32]);
  let c: [i16; 32] = unpack_low_i16_m512i(a, b).into();
  // Similar for i16
  for chunk in c.chunks(8) {
    for (i, &val) in chunk[..4].iter().enumerate() {
      assert_eq!(val, if i % 2 == 0 { 1 } else { 2 });
    }
  }
}

#[test]
fn test_shl_each_u16_m512i() {
  let a = m512i::from([1_u16; 32]);
  let count = m512i::from([2_u16; 32]);
  let b: [u16; 32] = shl_each_u16_m512i(a, count).into();
  assert_eq!(b, [4_u16; 32]);
}

#[test]
fn test_shl_each_u32_m512i() {
  let a = m512i::from([1_u32; 16]);
  let count = m512i::from([2_u32; 16]);
  let b: [u32; 16] = shl_each_u32_m512i(a, count).into();
  assert_eq!(b, [4_u32; 16]);
}

#[test]
fn test_shl_each_u64_m512i() {
  let a = m512i::from([1_u64; 8]);
  let count = m512i::from([2_u64; 8]);
  let b: [u64; 8] = shl_each_u64_m512i(a, count).into();
  assert_eq!(b, [4_u64; 8]);
}

#[test]
fn test_extract_m256i_from_m512i() {
  let a = m512i::from([1_i64, 2, 3, 4, 5, 6, 7, 8]);
  let b: [i64; 4] = extract_m256i_from_m512i::<0>(a).into();
  assert_eq!(b, [1, 2, 3, 4]);
  let c: [i64; 4] = extract_m256i_from_m512i::<1>(a).into();
  assert_eq!(c, [5, 6, 7, 8]);
}

#[test]
fn test_extract_m256_from_m512() {
  let a = m512::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
  let b = extract_m256_from_m512::<0>(a).to_array();
  assert_eq!(b, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  let c = extract_m256_from_m512::<1>(a).to_array();
  assert_eq!(c, [9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
}

#[test]
fn test_extract_m256d_from_m512d() {
  let a = m512d::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  let b = extract_m256d_from_m512d::<0>(a).to_array();
  assert_eq!(b, [1.0, 2.0, 3.0, 4.0]);
  let c = extract_m256d_from_m512d::<1>(a).to_array();
  assert_eq!(c, [5.0, 6.0, 7.0, 8.0]);
}

#[test]
fn test_insert_m256i_to_m512i() {
  let a = m512i::from([1_i64, 2, 3, 4, 5, 6, 7, 8]);
  let b = m256i::from([10_i64, 11, 12, 13]);
  let c: [i64; 8] = insert_m256i_to_m512i::<1>(a, b).into();
  assert_eq!(c, [1, 2, 3, 4, 10, 11, 12, 13]);
}

#[test]
fn test_insert_m256_to_m512() {
  let a = m512::from_array([0.0; 16]);
  let b = m256::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  let c = insert_m256_to_m512::<1>(a, b).to_array();
  assert_eq!(c, [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
}

#[test]
fn test_insert_m256d_to_m512d() {
  let a = m512d::from_array([0.0; 8]);
  let b = m256d::from_array([1.0, 2.0, 3.0, 4.0]);
  let c = insert_m256d_to_m512d::<1>(a, b).to_array();
  assert_eq!(c, [0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn test_cast_m256i_to_m512i() {
  let a = m256i::from([1_i64; 4]);
  let b = cast_m256i_to_m512i(a);
  let c: [i64; 4] = cast_m512i_to_m256i(b).into();
  assert_eq!(c, [1_i64; 4]);
}

#[test]
fn test_cast_m256_to_m512() {
  let a = m256::from_array([1.0; 8]);
  let b = cast_m256_to_m512(a);
  let c = cast_m512_to_m256(b).to_array();
  assert_eq!(c, [1.0; 8]);
}

#[test]
fn test_cast_m256d_to_m512d() {
  let a = m256d::from_array([1.0; 4]);
  let b = cast_m256d_to_m512d(a);
  let c = cast_m512d_to_m256d(b).to_array();
  assert_eq!(c, [1.0; 4]);
}

#[test]
fn test_cast_m512i_to_m256i() {
  let a = m512i::from([1_i64, 2, 3, 4, 5, 6, 7, 8]);
  let b: [i64; 4] = cast_m512i_to_m256i(a).into();
  assert_eq!(b, [1, 2, 3, 4]);
}

#[test]
fn test_cast_m512_to_m256() {
  let a = m512::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);
  let b = cast_m512_to_m256(a).to_array();
  assert_eq!(b, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
}

#[test]
fn test_cast_m512d_to_m256d() {
  let a = m512d::from_array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
  let b = cast_m512d_to_m256d(a).to_array();
  assert_eq!(b, [1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn test_permute2_i32_m512i() {
  let a = m512i::from([0_i32; 16]);
  let b = m512i::from([16_i32; 16]);
  let idx = m512i::from([16_i32; 16]); // All select from b[0]
  let c: [i32; 16] = permute2_i32_m512i(a, idx, b).into();
  assert_eq!(c, [16_i32; 16]);
}

#[test]
fn test_permute_i64_m512i() {
  let a = m512i::from([0_i64, 1, 2, 3, 4, 5, 6, 7]);
  let idx = m512i::from([7_i64, 6, 5, 4, 3, 2, 1, 0]);
  let b: [i64; 8] = permute_i64_m512i(idx, a).into();
  assert_eq!(b, [7, 6, 5, 4, 3, 2, 1, 0]);
}

#[test]
fn test_reduce_add_m512() {
  let a = m512::from_array([1.0; 16]);
  let sum = reduce_add_m512(a);
  assert_eq!(sum, 16.0);
}

#[test]
fn test_reduce_add_m512d() {
  let a = m512d::from_array([1.0; 8]);
  let sum = reduce_add_m512d(a);
  assert_eq!(sum, 8.0);
}

#[test]
fn test_load_masked_i8_m512i() {
  let src = m512i::from([1_i8; 64]);
  let data = [5_i8; 64];
  let mask = 0xFFFFFFFFFFFFFFFF;
  let a: [i8; 64] = load_masked_i8_m512i(src, mask, &data).into();
  assert_eq!(a, [5_i8; 64]);
}

#[test]
fn test_load_masked_i16_m512i() {
  let src = m512i::from([1_i16; 32]);
  let data = [5_i16; 32];
  let mask = 0xFFFFFFFF;
  let a: [i16; 32] = load_masked_i16_m512i(src, mask, &data).into();
  assert_eq!(a, [5_i16; 32]);
}

#[test]
fn test_load_masked_i32_m512i() {
  let src = m512i::from([1_i32; 16]);
  let data = [5_i32; 16];
  let mask = 0xFFFF;
  let a: [i32; 16] = load_masked_i32_m512i(src, mask, &data).into();
  assert_eq!(a, [5_i32; 16]);
}

#[test]
fn test_load_masked_m512() {
  let src = m512::from_array([1.0; 16]);
  let data = [5.0_f32; 16];
  let mask = 0xFFFF;
  let a = load_masked_m512(src, mask, &data).to_array();
  assert_eq!(a, [5.0; 16]);
}

#[test]
fn test_load_masked_m512d() {
  let src = m512d::from_array([1.0; 8]);
  let data = [5.0_f64; 8];
  let mask = 0xFF;
  let a = load_masked_m512d(src, mask, &data).to_array();
  assert_eq!(a, [5.0; 8]);
}

#[test]
fn test_store_masked_i8_m512i() {
  let a = m512i::from([5_i8; 64]);
  let mut mem = [0_i8; 64];
  let mask = 0xAAAAAAAAAAAAAAAA;
  store_masked_i8_m512i(&mut mem, mask, a);
  for (i, &val) in mem.iter().enumerate() {
    assert_eq!(val, if (mask >> i) & 1 == 1 { 5 } else { 0 });
  }
}

#[test]
fn test_store_masked_i16_m512i() {
  let a = m512i::from([5_i16; 32]);
  let mut mem = [0_i16; 32];
  let mask = 0xAAAAAAAA;
  store_masked_i16_m512i(&mut mem, mask, a);
  for (i, &val) in mem.iter().enumerate() {
    assert_eq!(val, if (mask >> i) & 1 == 1 { 5 } else { 0 });
  }
}

#[test]
fn test_store_masked_i32_m512i() {
  let a = m512i::from([5_i32; 16]);
  let mut mem = [0_i32; 16];
  let mask = 0xAAAA;
  store_masked_i32_m512i(&mut mem, mask, a);
  for (i, &val) in mem.iter().enumerate() {
    assert_eq!(val, if (mask >> i) & 1 == 1 { 5 } else { 0 });
  }
}

#[test]
fn test_store_masked_m512() {
  let a = m512::from_array([5.0; 16]);
  let mut mem = [0.0_f32; 16];
  let mask = 0xAAAA;
  store_masked_m512(&mut mem, mask, a);
  for (i, &val) in mem.iter().enumerate() {
    assert_eq!(val, if (mask >> i) & 1 == 1 { 5.0 } else { 0.0 });
  }
}

#[test]
fn test_store_masked_m512d() {
  let a = m512d::from_array([5.0; 8]);
  let mut mem = [0.0_f64; 8];
  let mask = 0xAA;
  store_masked_m512d(&mut mem, mask, a);
  for (i, &val) in mem.iter().enumerate() {
    assert_eq!(val, if (mask >> i) & 1 == 1 { 5.0 } else { 0.0 });
  }
}

#[test]
fn test_load_m512() {
  let a = [1.0_f32; 16];
  let b = load_m512(&a).to_array();
  assert_eq!(b, [1.0; 16]);
}

#[test]
fn test_load_m512d() {
  let a = [1.0_f64; 8];
  let b = load_m512d(&a).to_array();
  assert_eq!(b, [1.0; 8]);
}

#[test]
fn test_load_m512i() {
  let a = m512i::from([1_i32; 16]);
  let b = load_m512i(&a);
  let c: [i32; 16] = b.into();
  assert_eq!(c, [1; 16]);
}

#[test]
fn test_store_m512() {
  let a = m512::from_array([5.0; 16]);
  let mut b = [0.0_f32; 16];
  store_m512(&mut b, a);
  assert_eq!(b, [5.0; 16]);
}

#[test]
fn test_store_m512d() {
  let a = m512d::from_array([5.0; 8]);
  let mut b = [0.0_f64; 8];
  store_m512d(&mut b, a);
  assert_eq!(b, [5.0; 8]);
}

#[test]
fn test_store_m512i() {
  let a = m512i::from([5_i32; 16]);
  let mut b = m512i::default();
  store_m512i(&mut b, a);
  let c: [i32; 16] = b.into();
  assert_eq!(c, [5; 16]);
}

#[test]
fn test_set_splat_i8_m512i() {
  let a: [i8; 64] = set_splat_i8_m512i(5).into();
  assert_eq!(a, [5_i8; 64]);
}

#[test]
fn test_set_splat_i16_m512i() {
  let a: [i16; 32] = set_splat_i16_m512i(5).into();
  assert_eq!(a, [5_i16; 32]);
}

#[test]
fn test_set_splat_i32_m512i() {
  let a: [i32; 16] = set_splat_i32_m512i(5).into();
  assert_eq!(a, [5_i32; 16]);
}

#[test]
fn test_set_splat_i64_m512i() {
  let a: [i64; 8] = set_splat_i64_m512i(5).into();
  assert_eq!(a, [5_i64; 8]);
}

#[test]
fn test_set_splat_m512() {
  let a = set_splat_m512(5.0).to_array();
  assert_eq!(a, [5.0; 16]);
}

#[test]
fn test_set_splat_m512d() {
  let a = set_splat_m512d(5.0).to_array();
  assert_eq!(a, [5.0; 8]);
}

#[test]
fn test_zeroed_m512i() {
  let a: [i32; 16] = zeroed_m512i().into();
  assert_eq!(a, [0; 16]);
}

#[test]
fn test_zeroed_m512() {
  let a = zeroed_m512().to_array();
  assert_eq!(a, [0.0; 16]);
}

#[test]
fn test_zeroed_m512d() {
  let a = zeroed_m512d().to_array();
  assert_eq!(a, [0.0; 8]);
}

#[test]
fn test_prefetch_t0() {
  let data = [1.0_f32; 16];
  prefetch_t0(&data);
  // Prefetch doesn't return a value, just ensuring it compiles and runs
}

#[test]
fn test_prefetch_et0() {
  let data = [1.0_f32; 16];
  prefetch_et0(&data);
  // Prefetch doesn't return a value, just ensuring it compiles and runs
}

// Convenience method tests
#[test]
fn test_m512_convenience_methods() {
  let arr = [1.0_f32; 16];
  let a = m512::from_array(arr);
  assert_eq!(a.to_array(), arr);
  
  let bits = [0x3F800000_u32; 16]; // 1.0 in IEEE 754
  let b = m512::from_bits(bits);
  assert_eq!(b.to_array(), arr);
  assert_eq!(b.to_bits(), bits);
}

#[test]
fn test_m512d_convenience_methods() {
  let arr = [1.0_f64; 8];
  let a = m512d::from_array(arr);
  assert_eq!(a.to_array(), arr);
  
  let bits = [0x3FF0000000000000_u64; 8]; // 1.0 in IEEE 754
  let b = m512d::from_bits(bits);
  assert_eq!(b.to_array(), arr);
  assert_eq!(b.to_bits(), bits);
}