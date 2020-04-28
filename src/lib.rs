#![no_std]
#![warn(missing_docs)]
#![allow(unused_imports)]

//! Crate that safely exposes arch intrinsics via cfg.
//!
//! >Incomplete. WIP. Etc.
//! >
//! >Current content can be expected to remain stable, but the functionality
//! >coverage is not that much.
//!
//! This crate lets you safely use CPU intrinsics. Those things in
//! [`core::arch`](core::arch).
//! * Most of them are 100% safe to use as long as the CPU feature is available,
//!   like addition and multiplication and stuff.
//! * Some of them require that you uphold extra alignment requirements or
//!   whatever, which we do via the type system when necessary.
//! * Some of them are absolutely not safe at all because it causes UB at the
//!   LLVM level, so those things are not exposed here.
//!
//! This crate works purely via `cfg` and compile time feature selection, there
//! are no runtime checks. This means that if you _do_ want to do runtime
//! feature detection and then dynamically call an intrinsic if it happens to be
//! available, then this crate sadly isn't for you.
//!
//! ## Compile Time CPU Target Features
//!
//! At the time of me writing this, Rust enables the `sse` and `sse2` CPU target
//! features by default for all `i686` (x86) and `x86_64` builds. If you want
//! additional features available at compile time you'll have to enable them
//! with an additional arg to `rustc`. For a feature named `name` you pass `-C
//! target-feature=+name`, such as `-C target-feature=+sse3` for `sse3`.
//!
//! It's sometimes hard to know if your target platform will support a given
//! feature set, but the [Steam Hardware Survey][steam-survey] is generally
//! taken as a guide to what you can expect people to generally have. If you
//! click "Other Settings" it'll expand into a list of CPU target features and
//! how common they are. These days, it seems that `sse3` can be safely assumed,
//! and `ssse3`, `sse4.1`, and `sse4.2` are pretty safe bets as well. The stuff
//! above 128-bit isn't as common yet, give it another few years.
//!
//! **Please note that executing a program on a CPU that doesn't support the
//! target features it was compiles for is undefined behavior.**
//!
//! Currently, Rust doesn't actually support an easy way for you to check that a
//! feature enabled at compile time is _actually_ available at runtime. There is
//! the "[feature_detected][feature_detected]" family of macros, but if you
//! enable a feature they will evaluate to a constant `true` instead of actually
//! deferring the check for the feature to runtime. This means that, if you
//! _did_ want a check at the start of your program, to confirm that all the
//! assumed features are present and error out when the assumptions don't hold,
//! you can't use that macro. You gotta use CPUID and check manually. rip.
//! Hopefully we can make that process easier in a future version of this crate.
//!
//! [steam-survey]:
//! https://store.steampowered.com/hwsurvey/Steam-Hardware-Software-Survey-Welcome-to-Steam
//! [feature_detected]:
//! https://doc.rust-lang.org/std/index.html?search=feature_detected
//!
//! ## Current Support
//! As I said above, the crate is only Work In Progress status!
//!
//! * Intel (`x86` / `x86_64`)
//!   * `sse`

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
// arch-specific modules.

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod intel {
  //! Types and functions for safe `x86` / `x86_64` intrinsic usage.
  //!
  //! `x86_64` is essentially a superset of `x86`, so we just lump it all into
  //! one module.
  use super::*;
  #[cfg(target_arch = "x86")]
  use core::arch::x86::*;
  #[cfg(target_arch = "x86_64")]
  use core::arch::x86_64::*;

  submodule!(pub m128_);
  submodule!(pub sse);
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use intel::*;
