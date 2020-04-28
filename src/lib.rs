#![no_std]
#![warn(missing_docs)]
#![allow(unused_imports)]

//! Crate that safely exposes arch intrinsics via cfg.
//!
//! WIP.
//!
//! If a given CPU target feature is enabled for the build, then the associated
//! hardware intrinsics will be available as safe functions.

use core::{
  convert::AsRef,
  fmt::{
    Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex,
  },
  ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor,
    BitXorAssign, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub, SubAssign,
  },
};

/// Declares a private mod and then a glob `use` with the visibility specified.
macro_rules! submodule {
  ($v:vis $name:ident) => {
    mod $name;
    $v use $name::*;
  };
  ($v:vis $name:ident { $($content:tt)* }) => {
    mod $name { $($content)* }
    $v use $name::*;
  };
}

// unlike with the `submodule!` macro, we _want_ to expose the existence these
// arch-specific modules

#[cfg(target_arch = "x86_64")]
pub mod x86_64 {
  //! Types and functions for safe `x86_64` intrinsic usage.
  use super::*;
  use core::arch::x86_64::*;
  submodule!(pub m128_);
  submodule!(pub sse);
}
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
