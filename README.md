[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
[![AppVeyor](https://ci.appveyor.com/api/projects/status/03fc5f30h755235h/branch/master?svg=true)](https://ci.appveyor.com/project/Lokathor/safe-arch/branch/primary)
[![crates.io](https://img.shields.io/crates/v/safe_arch.svg)](https://crates.io/crates/safe_arch)
[![docs.rs](https://docs.rs/safe_arch/badge.svg)](https://docs.rs/safe_arch/)

# safe_arch

Exposes arch-specific intrinsics as safe function.

## Design

The crate aims to be _as minimal as possible_.

* SIMD types are newtype'd (with a `pub` field) and given appropriate trait
  impls such as `From`, `Into`, `Default`, etc.
* Each intrinsic gets either a function or macro so that you can safely use it
  as directly as possible.
  * Functions are used when all arguments are runtime arguments.
  * Macros are used when one of the arguments must be a compile time constant,
    because Rust doesn't let you "pass through" compile time constants.
* There's hundreds and hundreds of intrinsics, so the names of functions and
  macros tend to be very long and specific because there's often many similar
  ways to do nearly the same thing.
  * This crate isn't really intended for "everyday users". It is intended to be
    an "unopinionated" middle layer crate that just provides the safety. Higher
    level abstractions should mostly come from some other crate that wraps over
    this crate.

All function and macro availability is done purely at compile time via
`#[cfg()]` attributes on the various modules. If a CPU feature isn't enabled for
the build then those functions or macros won't be available. If you'd like to
determine what CPU features are available at runtime and then call different
code accordingly, this crate is not for you.

See the [crate docs](https://docs.rs/safe_arch) for more details.

## Minimum Rust Version.

The CI tests the crate against Stable 1.43.0. The doc-tests
are known to require at least 1.43 because they use `u32::MAX`, `f32::NAN`,
and similar short names for the common consts. You can _probably_ build the
crate without the doc tests on older compiler versions, possibly as far back
as 1.31, but that's not officially supported.

## Additional Resources

* [Intel Intrinsics Guide](https://software.intel.com/sites/landingpage/IntrinsicsGuide/)
  * [Raw Xml v3.5.2](https://software.intel.com/sites/landingpage/IntrinsicsGuide/files/data-3.5.2.xml) and you can check their [release notes](https://software.intel.com/sites/landingpage/IntrinsicsGuide/files/ReleaseNotes.html) to see if a later version has been put out since this readme file was last updated.

# Black Lives Matter
