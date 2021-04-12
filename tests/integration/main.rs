#![allow(bad_style)]
#![allow(unused_imports)]

use safe_arch::*;

#[cfg(target_feature = "adx")]
mod adx_tests;

#[cfg(target_feature = "bmi1")]
mod bmi1_tests;

#[cfg(target_feature = "bmi2")]
mod bmi2_tests;

#[cfg(target_feature = "lzcnt")]
mod lzcnt_tests;

#[cfg(target_feature = "rdrand")]
mod rdrand_tests;

#[cfg(target_feature = "sse2")]
mod sse2_tests;

#[allow(dead_code)]
fn approx_eq_f32(a: f32, b: f32) -> bool {
  (a - b).abs() < 0.00000001
}

#[allow(dead_code)]
fn approx_eq_f64(a: f64, b: f64) -> bool {
  (a - b).abs() < 0.00000000001
}
