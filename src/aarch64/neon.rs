use super::*;

/// Lanewise absolute value.
///
/// [vabsq_f32](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabsq_f32)
#[must_use]
#[inline(always)]
pub fn abs_float32x4(x: float32x4) -> float32x4 {
  float32x4(unsafe { vabsq_f32(x.0) })
}

/// Lanewise absolute value.
///
/// [vabsq_f64](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabsq_f64)
#[must_use]
#[inline(always)]
pub fn abs_float64x2(x: float64x2) -> float64x2 {
  float64x2(unsafe { vabsq_f64(x.0) })
}

/// Lanewise absolute value.
///
/// [vabsq_s16](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabsq_s16)
#[must_use]
#[inline(always)]
pub fn abs_int16x8(x: int16x8) -> int16x8 {
  int16x8(unsafe { vabsq_s16(x.0) })
}

/// Lanewise absolute value.
///
/// [vabsq_s32](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabsq_s32)
#[must_use]
#[inline(always)]
pub fn abs_int32x4(x: int32x4) -> int32x4 {
  int32x4(unsafe { vabsq_s32(x.0) })
}

/// Lanewise absolute value.
///
/// [vabsq_s64](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabsq_s64)
#[must_use]
#[inline(always)]
pub fn abs_int64x2(x: int64x2) -> int64x2 {
  int64x2(unsafe { vabsq_s64(x.0) })
}

/// Lanewise absolute value.
///
/// [vabsq_s8](https://developer.arm.com/architectures/instruction-sets/intrinsics/vabsq_s8)
#[must_use]
#[inline(always)]
pub fn abs_int8x16(x: int8x16) -> int8x16 {
  int8x16(unsafe { vabsq_s8(x.0) })
}

/*  */

/// Lanewise addition.
///
/// [vaddq_f32](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_f32)
#[must_use]
#[inline(always)]
pub fn add_float32x4(x: float32x4, y: float32x4) -> float32x4 {
  float32x4(unsafe { vaddq_f32(x.0, y.0) })
}

/// Lanewise addition.
///
/// [vaddq_f64](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_f64)
#[must_use]
#[inline(always)]
pub fn add_float64x2(x: float64x2, y: float64x2) -> float64x2 {
  float64x2(unsafe { vaddq_f64(x.0, y.0) })
}

/// Lanewise addition.
///
/// [vaddq_s16](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_s16)
#[must_use]
#[inline(always)]
pub fn add_int16x8(x: int16x8, y: int16x8) -> int16x8 {
  int16x8(unsafe { vaddq_s16(x.0, y.0) })
}

/// Lanewise addition.
///
/// [vaddq_s32](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_s32)
#[must_use]
#[inline(always)]
pub fn add_int32x4(x: int32x4, y: int32x4) -> int32x4 {
  int32x4(unsafe { vaddq_s32(x.0, y.0) })
}

/// Lanewise addition.
///
/// [vaddq_s64](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_s64)
#[must_use]
#[inline(always)]
pub fn add_int64x2(x: int64x2, y: int64x2) -> int64x2 {
  int64x2(unsafe { vaddq_s64(x.0, y.0) })
}

/// Lanewise addition.
///
/// [vaddq_s8](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_s8)
#[must_use]
#[inline(always)]
pub fn add_int8x16(x: int8x16, y: int8x16) -> int8x16 {
  int8x16(unsafe { vaddq_s8(x.0, y.0) })
}

/// Lanewise addition.
///
/// [vaddq_u16](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_u16)
#[must_use]
#[inline(always)]
pub fn add_uint16x8(x: uint16x8, y: uint16x8) -> uint16x8 {
  uint16x8(unsafe { vaddq_u16(x.0, y.0) })
}

/// Lanewise addition.
///
/// [vaddq_u32](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_u32)
#[must_use]
#[inline(always)]
pub fn add_uint32x4(x: uint32x4, y: uint32x4) -> uint32x4 {
  uint32x4(unsafe { vaddq_u32(x.0, y.0) })
}

/// Lanewise addition.
///
/// [vaddq_u64](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_u64)
#[must_use]
#[inline(always)]
pub fn add_uint64x2(x: uint64x2, y: uint64x2) -> uint64x2 {
  uint64x2(unsafe { vaddq_u64(x.0, y.0) })
}

/// Lanewise addition.
///
/// [vaddq_u8](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_u8)
#[must_use]
#[inline(always)]
pub fn add_uint8x16(x: uint8x16, y: uint8x16) -> uint8x16 {
  uint8x16(unsafe { vaddq_u8(x.0, y.0) })
}

/*  */

/// Horizontal addition.
///
/// [vaddvq_f32](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddvq_f32)
#[must_use]
#[inline(always)]
pub fn horizontal_add_float32x4(x: float32x4) -> f32 {
  unsafe { vaddvq_f32(x.0) }
}

/// Horizontal addition.
///
/// [vaddvq_f64](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddvq_f64)
#[must_use]
#[inline(always)]
pub fn horizontal_add_float64x2(x: float64x2) -> f64 {
  unsafe { vaddvq_f64(x.0) }
}

/// Horizontal addition.
///
/// [vaddvq_s16](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddvq_s16)
#[must_use]
#[inline(always)]
pub fn horizontal_add_int16x8(x: int16x8) -> i16 {
  unsafe { vaddvq_s16(x.0) }
}

/// Horizontal addition.
///
/// [vaddvq_s32](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddvq_s32)
#[must_use]
#[inline(always)]
pub fn horizontal_add_int32x4(x: int32x4) -> i32 {
  unsafe { vaddvq_s32(x.0) }
}

/// Horizontal addition.
///
/// [vaddvq_s64](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddvq_s64)
#[must_use]
#[inline(always)]
pub fn horizontal_add_int64x2(x: int64x2) -> i64 {
  unsafe { vaddvq_s64(x.0) }
}

/// Horizontal addition.
///
/// [vaddvq_s8](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddvq_s8)
#[must_use]
#[inline(always)]
pub fn horizontal_add_int8x16(x: int8x16) -> i8 {
  unsafe { vaddvq_s8(x.0) }
}

/// Horizontal addition.
///
/// [vaddvq_u16](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddvq_u16)
#[must_use]
#[inline(always)]
pub fn horizontal_add_uint16x8(x: uint16x8) -> u16 {
  unsafe { vaddvq_u16(x.0) }
}

/// Horizontal addition.
///
/// [vaddvq_u32](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddvq_u32)
#[must_use]
#[inline(always)]
pub fn horizontal_add_uint32x4(x: uint32x4) -> u32 {
  unsafe { vaddvq_u32(x.0) }
}

/// Horizontal addition.
///
/// [vaddvq_u64](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddvq_u64)
#[must_use]
#[inline(always)]
pub fn horizontal_add_uint64x2(x: uint64x2) -> u64 {
  unsafe { vaddvq_u64(x.0) }
}

/// Horizontal addition.
///
/// [vaddq_u8](https://developer.arm.com/architectures/instruction-sets/intrinsics/vaddq_u8)
#[must_use]
#[inline(always)]
pub fn horizontal_add_uint8x16(x: uint8x16) -> u8 {
  unsafe { vaddvq_u8(x.0) }
}
