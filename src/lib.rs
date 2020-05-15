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
//! to them, but the inner value is a `pub` field so feel free to just grab it
//! out if you need to. Trait impls of the newtypes include: `Default` (zeroed),
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
//! * Function names end with the register type they're most associated with. I
//!   say "most" because while most operations only work with a single register
//!   type at a time there are occasional operations that use more than one
//!   register type.
//!   * Eg: `and_m128` (for `m128`) and `and_m128d` (for `m128d`)
//! * If a function operates on just the lowest data lane it generally has `_s`
//!   after the register type, because it's a "scalar" operation. The higher
//!   lanes are generally just copied forward, or taken from a secondary
//!   argument, or something. Details vary.
//!   * Eg: `sqrt_m128` (all lanes) and `sqrt_m128_s` (low lane only)
//!
//! Of course, people can't even always agree on what words mean. The common
//! verb names for this crate, and their conventions, are as follows:
//! * `load`: Reads memory into a register (deref `&Foo` to `Foo`).
//! * `store`: Writes a register to memory (writes `Foo` to a `&mut Foo`).
//! * `set`: Packs values into a register (works like `[1, 2, 3, 4]` to build an
//!   array).
//! * `splat`: Copy a value as many times as possible across the bits of a
//!   register (works like `[1_i32; LEN]` array building).
//! * `extract`: Get an individual lane out of a SIMD register (works like array
//!   access). The lane to get has to be a const value.
//! * `insert`: Duplicate a register and then replace the value of a specific
//!   lane (works like `let mut a2 = a.clone(); a2[i] = new;`). The lane to
//!   overwrite has to be a const value.
//! * `cast`: change data types while preserving the bit pattern (like how
//!   `transmute` would do it).
//! * `convert`: change data types while trying to preserve the numeric value
//!   (which might change the bits, like how `as` would do it).
//!
//! **This crate is pre-1.0 and if you feel that an operation should have a
//! better name to improve the crate's consistency please file an issue.**
//!
//! ## Current Support
//! * `x86` / `x86_64` (Intel, AMD, etc)
//!   * 128-bit: `sse`, `sse2`, `sse3`, `ssse3`, `sse4.1`, `sse4.2`
//!   * 256-bit: `avx`
//!   * Other: `adx`, `aes`, `bmi1`, `bmi2`, `lzcnt`, `pclmulqdq`, `popcnt`,
//!     `rdrand`, `rdseed`
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

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
submodule!(pub x86_x64 {
  //! Types and functions for safe `x86` / `x86_64` intrinsic usage.
  //!
  //! `x86_64` is essentially a superset of `x86`, so we just lump it all into
  //! one module. Anything not available on `x86` simply won't be in the build
  //! on that arch.
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

  // Note(Lokathor): We only include these sub-modules with the actual functions
  // if the feature is enabled. Ae *also* have a cfg attribute on the inside of
  // the modules as a "double-verification" of sorts. Technically either way on
  // its own would also be fine.

  // These CPU features follow a fairly clear and strict progression that's easy
  // to remember. Most of them offer a fair pile of new functions.
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

  // These features aren't as easy to remember the progression of and they each
  // only add a small handful of functions.
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

  /// Reads the CPU's timestamp counter value.
  ///
  /// This is a monotonically increasing time-stamp that goes up every clock
  /// cycle of the CPU. However, since modern CPUs are variable clock rate
  /// depending on demand this can't actually be used for telling the time. It
  /// also does _not_ fully serialize all operations, so previous instructions
  /// might still be in progress when this reads the timestamp.
  ///
  /// * **Intrinsic:** `_rdtsc`
  /// * **Assembly:** `rdtsc`
  pub fn read_timestamp_counter() -> u64 {
    unsafe { _rdtsc() }
  }

  /// Reads the CPU's timestamp counter value and store the processor signature.
  ///
  /// This works similar to [`read_timestamp_counter`] with two main
  /// differences:
  /// * It and also stores the `IA32_TSC_AUX MSR` value to the reference given.
  /// * It waits on all previous instructions to finish before reading the
  ///   timestamp (though it doesn't prevent other instructions from starting).
  ///
  /// As with `read_timestamp_counter`, you can't actually use this to tell the
  /// time.
  ///
  /// * **Intrinsic:** `__rdtscp`
  /// * **Assembly:** `rdtscp`
  pub fn read_timestamp_counter_p(aux: &mut u32) -> u64 {
    unsafe { __rdtscp(aux) }
  }

  /// Swap the bytes of the given 32-bit value.
  ///
  /// ```
  /// # use safe_arch::*;
  /// assert_eq!(byte_swap_i32(0x0A123456), 0x5634120A);
  /// ```
  /// * **Intrinsic:** `_bswap`
  /// * **Assembly:** `bswap r32`
  pub fn byte_swap_i32(i: i32) -> i32 {
    unsafe { _bswap(i) }
  }

  /// Swap the bytes of the given 64-bit value.
  ///
  /// ```
  /// # use safe_arch::*;
  /// assert_eq!(byte_swap_i64(0x0A123456_789ABC01), 0x01BC9A78_5634120A);
  /// ```
  /// * **Intrinsic:** `_bswap64`
  /// * **Assembly:** `bswap r64`
  pub fn byte_swap_i64(i: i64) -> i64 {
    unsafe { _bswap64(i) }
  }
});
