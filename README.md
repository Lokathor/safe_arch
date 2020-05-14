[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
[![AppVeyor](https://ci.appveyor.com/api/projects/status/03fc5f30h755235h/branch/master?svg=true)](https://ci.appveyor.com/project/Lokathor/safe-arch/branch/master)
[![crates.io](https://img.shields.io/crates/v/safe_arch.svg)](https://crates.io/crates/safe_arch)
[![docs.rs](https://docs.rs/safe_arch/badge.svg)](https://docs.rs/safe_arch/)

# safe_arch

Exposes arch-specific intrinsics as safe function.

The crate aims to be _as minimal as possible_. Each intrinsic gets a function or
macro so that you can safely use it as directly as possible. Essentially no
additional abstractions are provided, that's not the point of this crate. The
goal is only to provide safety while getting in the way as little as possible.

All function and macro availability is done purely at compile time via
`#[cfg()]` attributes on the various modules. If a CPU feature isn't enabled for
the build then those functions or macros won't be available. If you'd like to
determine what CPU features are available at runtime and then call different
code accordingly, this crate is not for you.

See the [crate docs](https://docs.rs/safe_arch) for more details.

* **Minimum Rust:** The CI tests the crate against Stable 1.43.0. The doc-tests
  are known to require at least 1.43 because they use `u32::MAX`, `f32::NAN`,
  and similar short names for the common consts. You can _probably_ build the
  crate without the doc tests on older compiler versions, possibly as far back
  as 1.31, but that's not officially supported.

## Additional Resources

* [Intel Intrinsics Guide](https://software.intel.com/sites/landingpage/IntrinsicsGuide/)
  * [Raw Xml v3.5.0](https://software.intel.com/sites/landingpage/IntrinsicsGuide/files/data-3.5.0.xml) and you can check their [release notes](https://software.intel.com/sites/landingpage/IntrinsicsGuide/files/ReleaseNotes.html) to see if a later version has been put out since this readme file was last updated.
