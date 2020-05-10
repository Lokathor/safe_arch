#![no_std]
#![warn(missing_docs)]
#![allow(unused_imports)]
#![allow(clippy::transmute_ptr_to_ptr)]
#![cfg_attr(docs_rs, feature(doc_cfg))]

//! A crate that safely exposes arch intrinsics via `#[cfg()]`.
//!
//! `safe_arch` lets you safely use CPU intrinsics. Those things in the
//! [`core::arch`](core::arch) modules. It works purely via `#[cfg()]` and
//! compile time CPU feature declaration. If you want to check for a feature at
//! runtime and then call an intrinsic or use a fallback path based on that then
//! this crate is sadly not for you.
//!
//! SIMD register types are "newtype'd" so that better trait impls can be given
//! to them, but the inner value is a `pub` field so feel to just grab it out if
//! you need to. Trait impls of the newtypes include: `Default` (zeroed),
//! `From`/`Into` of appropriate data types, and appropriate operator
//! overloading.
//!
//! * Most intrinsics (like addition and multiplication) are totally safe to use
//!   as long as the CPU feature is available. In this case, what you get is 1:1
//!   with the actual intrinsic.
//! * Some intrinsics take a pointer of an assumed minimum alignment and
//!   validity span. For these, the `safe_arch` function takes a reference of an
//!   appropriate type to uphold safety.
//!   * Try the [bytemuck](https://docs.rs/bytemuck) crate (and turn on the
//!     `bytemuck` feature of this crate) if you want help safely casting
//!     between reference types.
//! * Some intrinsics are not safe unless you're _very_ careful about how you
//!   use them, such as the streaming operations requiring you to use them in
//!   combination with an appropriate memory fence. Those operations aren't
//!   exposed here.
//! * Some intrinsics mess with the processor state, such as changing the
//!   floating point flags, saving and loading special register state, and so
//!   on. LLVM doesn't really support you messing with that within a high level
//!   language, so those operations aren't exposed here. Use assembly or
//!   something if you want to do that.
//!
//! ## Naming Conventions
//! The actual names for each intrinsic are generally a flaming dumpster of
//! letters that only make sense _after_ you've learned all the names. They're
//! very bad for learning what things do. Accordingly, `safe_arch` uses very
//! verbose naming that (hopefully) improves the new-user experience.
//!
//! * Function names start with the primary "verb" of the operation, and then
//!   any adverbs go after that. This makes for slightly awkward English but
//!   helps the list of all the functions sort a little better.
//!   * Eg: `add_i32_m128i` and `add_i16_saturating_m128i`
//! * Function names end with the register type they're most associated with.
//!   * Eg: `and_m128` (for `m128`) and `and_m128d` (for `m128d`)
//! * If a function operates on just the lowest data lane it generally has `_s`
//!   after the register type, because it's a "scalar" operation. The higher
//!   lanes are generally just copied forward, or taken from a secondary
//!   argument, or something. Details vary.
//!   * Eg: `sqrt_m128` (all lanes) and `sqrt_m128_s` (low lane only)
//!
//! Sometimes there's more than one name for essentially the same operation,
//! because these intrinsics were added to the CPU slowly over a decade or more.
//! Particularly for going into and out of SIMD form there's the following
//! naming convention:
//! * `load` reads from memory into a register.
//! * `store` writes from a register into memory.
//! * `set` packs non-SIMD data into a SIMD register.
//! * `splat` copies a value as many times as it fits across a SIMD register.
//! * `extract` picks a lane's value out of SIMD into a non-SIMD data type.
//! * `insert` copies a register and replaces a particular lane's value.
//!
//! **This crate is pre-1.0 and if you feel that an operation should have a
//! better name to improve the crate's consistency please file an issue.**
//!
//! ## Current Support
//! * Intel (`x86` / `x86_64`)
//!   * 128-bit: `sse`, `sse2`, `sse3`, `ssse3`, `sse4.1`, `sse4.2`
//!   * 256-bit: `avx`
//!   * Other: `adx`, `aes`, `bmi1`, `bmi2`, `lzcnt`, `pclmulqdq`, `popcnt`,
//!     `rdrand`, `rdseed`
//!
//! ## Power License Levels
//! It sounds like a goofy thing from a video game, but different CPU operations
//! have a "license level", and this crate makes an attempt to recognize this
//! concept via cargo feature because it can have a performance impact.
//! * Any 256-bit multiplication (via either AVX or FMA), as well as all 512-bit
//!   operations, are "license 1".
//! * Any 512-bit multiplication is "license 2".
//! * Any other operation is just "license 0".
//! * A heavy enough mix of instructions that wouldn't normally demand a higher
//!   license level individually can still cause a core to request a higher
//!   license level, as can speculative executions from a failed branch
//!   prediction.
//!
//! If you execute an instruction that's of a _higher_ license level than the
//! CPU core's current license level then the core greatly slows down while it
//! negotiates with the power manager that it wants to be at the new higher
//! license level and drawing more power (this takes up to 500 micro-seconds).
//!
//! While the CPU is actually at license 1 or license 2 the core's clock rate is
//! limited compared to the normal license 0 clock rate. The higher power draw
//! means more heat, and so the core has to clock down to compensate. Exactly
//! how much the core has to clock down depends on the CPU and how many cores
//! are using what license levels and so on. Basically, more power used means
//! more heat build up means the device has to slow down or it would damage
//! itself.
//!
//! After a while (~2ms) of not executing any higher license level instructions
//! the CPU core will naturally reduce its license level back down on its own.
//!
//! * Dedicated usage of a higher license level feature will generally give an
//!   overall gain in performance. The core slows down a bit at the start, and
//!   then runs a little slower than the normal top speed while working, but
//!   processes through all the lane elements twice as fast and so overall
//!   there's a gain.
//! * Occasional usage of a higher license level _can_ make performance worse,
//!   because the CPU slows down, doesn't do enough work to justify the slow
//!   down, and then whatever happens after that work is done is _also_ slowed
//!   until the higher license level "wears off" and things go back to normal.
//!
//! So if you want to opt-in to the functions that will demand a higher license
//! level you have to enable them via the `license1` and `license2` cargo
//! features. This isn't meant to be a major obstacle, just a tiny reminder.
//! Even with the cargo features on you must also have the appropriate CPU
//! feature enabled during the build of course.
//!
//! For more details, search the word "license" in the [Intel 64 and IA-32
//! Architectures Optimization Reference Manual][pdf], it's in section 2.2.3 of
//! the current version.
//!
//! [pdf]:
//! https://software.intel.com/sites/default/files/managed/9e/bc/64-ia-32-architectures-optimization-manual.pdf
//!
//! ## Compile Time CPU Target Features
//!
//! At the time of me writing this, Rust enables the `sse` and `sse2` CPU
//! features by default for all `i686` (x86) and `x86_64` builds. Those CPU
//! features are built into the design of `x86_64`, and you'd need a _super_ old
//! `x86` CPU for it to not support at least `sse` and `sse2`, so they're a safe
//! bet for the language to enable all the time. In fact, because the standard
//! library is compiled with them enabled, simply trying to _disable_ those
//! features would actually cause ABI issues and fill your program with UB
//! ([link][rustc_docs]).
//!
//! If you want additional CPU features available at compile time you'll have to
//! enable them with an additional arg to `rustc`. For a feature named `name`
//! you pass `-C target-feature=+name`, such as `-C target-feature=+sse3` for
//! `sse3`.
//!
//! You can alternately enable _all_ target features of the current CPU with `-C
//! target-cpu=native`. This is primarily of use if you're building a program
//! you'll only run on your own system.
//!
//! It's sometimes hard to know if your target platform will support a given
//! feature set, but the [Steam Hardware Survey][steam-survey] is generally
//! taken as a guide to what you can expect people to have available. If you
//! click "Other Settings" it'll expand into a list of CPU target features and
//! how common they are. These days, it seems that `sse3` can be safely assumed,
//! and `ssse3`, `sse4.1`, and `sse4.2` are pretty safe bets as well. The stuff
//! above 128-bit isn't as common yet, give it another few years.
//!
//! **Please note that executing a program on a CPU that doesn't support the
//! target features it was compiles for is Undefined Behavior.**
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
//! [rustc_docs]: https://doc.rust-lang.org/rustc/targets/known-issues.html
//!
//! ### A Note On Working With Cfg
//!
//! There's two main ways to use `cfg`:
//! * Via an attribute placed on an item, block, or expression:
//!   * `#[cfg(debug_assertions)] println!("hello");`
//! * Via a macro used within an expression position:
//!   * `if cfg!(debug_assertions) { println!("hello"); }`
//!
//! The difference might seem small but it's actually very important:
//! * The attribute form will include code or not _before_ deciding if all the
//!   items named and so forth really exist or not. This means that code that is
//!   configured via attribute can safely name things that don't always exist as
//!   long as the things they name do exist whenever that code is configured
//!   into the build.
//! * The macro form will include the configured code _no matter what_, and then
//!   the macro resolves to a constant `true` or `false` and the compiler uses
//!   dead code elimination to cut out the path not taken.
//!
//! This crate uses `cfg` via the attribute, so the functions it exposes don't
//! exist at all when the appropriate CPU target features aren't enabled.
//! Accordingly, if you plan to call this crate or not depending on what
//! features are enabled in the build you'll also need to control your use of
//! this crate via cfg attribute, not cfg macro.

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
  submodule!(pub m128d_);
  submodule!(pub m128i_);
  submodule!(pub m256_);
  submodule!(pub m256d_);
  submodule!(pub m256i_);
  // Note(Lokathor): We only include these sub-modules if the feature is enabled
  // and we *also* cfg attribute on the inside of the modules as a
  // double-verification of sorts. Technically either way on its own would also
  // be fine.
  #[cfg(target_feature = "sse")]
  submodule!(pub sse);
  #[cfg(target_feature = "sse2")]
  submodule!(pub sse2);
  #[cfg(target_feature = "sse3")]
  submodule!(pub sse3);
  #[cfg(target_feature = "ssse3")]
  submodule!(pub ssse3);
  #[cfg(target_feature = "sse4.1")]
  submodule!(pub sse4_1);
  #[cfg(target_feature = "sse4.2")]
  submodule!(pub sse4_2);
  #[cfg(target_feature = "avx")]
  submodule!(pub avx);

  #[cfg(target_feature = "adx")]
  submodule!(pub adx);
  #[cfg(target_feature = "aes")]
  submodule!(pub aes);
  #[cfg(target_feature = "bmi1")]
  submodule!(pub bmi1);
  #[cfg(target_feature = "bmi2")]
  submodule!(pub bmi2);
  #[cfg(target_feature = "lzcnt")]
  submodule!(pub lzcnt);
  #[cfg(target_feature = "pclmulqdq")]
  submodule!(pub pclmulqdq);
  #[cfg(target_feature = "popcnt")]
  submodule!(pub popcnt);
  #[cfg(target_feature = "rdrand")]
  submodule!(pub rdrand);
  #[cfg(target_feature = "rdseed")]
  submodule!(pub rdseed);
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use intel::*;
