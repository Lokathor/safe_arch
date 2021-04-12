use super::*;

#[test]
fn test_abs_i8_m128i() {
  let a = m128i::from([
    0_i8, -1, 2, -3, 4, -5, 6, -7, -8, 9, -10, 11, -12, 13, -14, -128,
  ]);
  let c: [i8; 16] = abs_i8_m128i(a).into();
  assert_eq!(c, [0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, -128]);
}

#[test]
fn test_abs_i16_m128i() {
  let a = m128i::from([0_i16, 1, 2, 3, 4, 5, 6, i16::MIN]);
  let c: [i16; 8] = abs_i16_m128i(a).into();
  assert_eq!(c, [0_i16, 1, 2, 3, 4, 5, 6, i16::MIN]);
}

#[test]
fn test_() {
  let a = m128i::from([0, -1, 2, i32::MIN]);
  let c: [i32; 4] = abs_i32_m128i(a).into();
  assert_eq!(c, [0, 1, 2, i32::MIN]);
}

#[test]
fn test_combined_byte_shr_imm_m128i() {
  let a =
    m128i::from([0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
  let b = m128i::from([
    16_i8, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
  ]);
  // `a` bytes come in to the high indexes because these are LE bytes.
  let c: [i8; 16] = combined_byte_shr_imm_m128i::<3>(a, b).into();
  assert_eq!(
    c,
    [19_i8, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2]
  );
  // If you feed the same register to both sides it becomes a rotate
  let c: [i8; 16] = combined_byte_shr_imm_m128i::<3>(a, a).into();
  assert_eq!(c, [3_i8, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 1, 2,]);
}

#[test]
fn test_add_horizontal_i16_m128i() {
  let a = m128i::from([1_i16, 2, 3, 4, -1, -2, -3, -4]);
  let b = m128i::from([5_i16, 6, 7, 8, -15, -26, -37, 48]);
  let c: [i16; 8] = add_horizontal_i16_m128i(a, b).into();
  assert_eq!(c, [3, 7, -3, -7, 11, 15, -41, 11]);
}

#[test]
fn test_add_horizontal_i32_m128i() {
  let a = m128i::from([1, 2, 3, 4]);
  let b = m128i::from([5, 6, 7, 8]);
  let c: [i32; 4] = add_horizontal_i32_m128i(a, b).into();
  assert_eq!(c, [3, 7, 11, 15]);
}

#[test]
fn test_add_horizontal_saturating_i16_m128i() {
  let a = m128i::from([i16::MAX, i16::MAX, 3, 4, -1, -2, -3, -4]);
  let b = m128i::from([5_i16, 6, 7, 8, -15, -26, -37, 48]);
  let c: [i16; 8] = add_horizontal_saturating_i16_m128i(a, b).into();
  assert_eq!(c, [i16::MAX, 7, -3, -7, 11, 15, -41, 11]);
}

#[test]
fn test_sub_horizontal_i16_m128i() {
  let a = m128i::from([1_i16, 29, 3, 64, -18, -23, -73, -14]);
  let b = m128i::from([50_i16, 76, 72, 89, -15, -26, -37, 48]);
  let c: [i16; 8] = sub_horizontal_i16_m128i(a, b).into();
  assert_eq!(c, [-28, -61, 5, -59, -26, -17, 11, -85]);
}

#[test]
fn test_sub_horizontal_i32_m128i() {
  let a = m128i::from([1, 29, 3, 42]);
  let b = m128i::from([5, 96, 7, 84]);
  let c: [i32; 4] = sub_horizontal_i32_m128i(a, b).into();
  assert_eq!(c, [-28, -39, -91, -77]);
}

#[test]
fn test_sub_horizontal_saturating_i16_m128i() {
  let a = m128i::from([i16::MIN, 1, 3, 49, -1, -27, -3, -412]);
  let b = m128i::from([5_i16, 699, 7, 877, -15, -2664, -37, 4008]);
  let c: [i16; 8] = sub_horizontal_saturating_i16_m128i(a, b).into();
  assert_eq!(c, [i16::MIN, -46, 26, 409, -694, -870, 2649, -4045]);
}

#[test]
fn test_mul_u8i8_add_horizontal_saturating_m128i() {
  let a = m128i::from([
    255_u8, 255, 0, 0, 255, 255, 1, 1, 8, 9, 10, 11, 12, 13, 14, 15,
  ]);
  let b = m128i::from([
    127_i8, 127, 0, 0, -127, -127, 1, 1, 24, 25, 26, 27, 28, 29, 30, 31,
  ]);
  let c: [i16; 8] = mul_u8i8_add_horizontal_saturating_m128i(a, b).into();
  assert_eq!(c, [i16::MAX, 0, i16::MIN, 2, 417, 557, 713, 885]);
}

#[test]
fn test_mul_i16_scale_round_m128i() {
  let a = m128i::from([0_i16, 100, 200, 300, 400, 500, 600, 700]);
  let b = m128i::from([800_i16, 900, 1000, 1100, 1200, 1300, 1400, 1500]);
  let c: [i16; 8] = mul_i16_scale_round_m128i(a, b).into();
  assert_eq!(c, [0, 3, 6, 10, 15, 20, 26, 32]);
}

#[test]
fn test_shuffle_av_i8z_all_m128i() {
  let a =
    m128i::from([70_i8, 1, 2, 3, 4, 5, 6, 7, 8, 99, 100, 11, 12, 13, 14, 55]);
  let v =
    m128i::from([-1_i8, 5, 4, 1, 3, 0, 9, 10, 2, 14, 6, 7, 15, 12, 13, 8]);
  let c: [i8; 16] = shuffle_av_i8z_all_m128i(a, v).into();
  assert_eq!(c, [0_i8, 5, 4, 1, 3, 70, 99, 100, 2, 14, 6, 7, 55, 12, 13, 8]);
}

#[test]
fn test_sign_apply_i8_m128i() {
  let a =
    m128i::from([0_i8, 1, -2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, -15]);
  let b = m128i::from([-1_i8, 1, 1, -1, -1, 1, 1, 1, 1, 0, 0, -1, -1, 0, 0, 1]);
  let c: [i8; 16] = sign_apply_i8_m128i(a, b).into();
  assert_eq!(c, [0_i8, 1, -2, -3, -4, 5, 6, 7, 8, 0, 0, -11, -12, 0, 0, -15]);
}

#[test]
fn test_sign_apply_i16_m128i() {
  let a = m128i::from([1_i16, 2, -3, 4, 5, 6, 7, 8]);
  let b = m128i::from([5_i16, -6, 7, 0, 1, 1, 0, 1]);
  let c: [i16; 8] = sign_apply_i16_m128i(a, b).into();
  assert_eq!(c, [1_i16, -2, -3, 0, 5, 6, 0, 8]);
}

#[test]
fn test_sign_apply_i32_m128i() {
  let a = m128i::from([1, 2, -3, 4]);
  let b = m128i::from([5, -6, 7, 0]);
  let c: [i32; 4] = sign_apply_i32_m128i(a, b).into();
  assert_eq!(c, [1, -2, -3, 0]);
}
