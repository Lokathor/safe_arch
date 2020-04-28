[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
[![AppVeyor](https://ci.appveyor.com/api/projects/status/03fc5f30h755235h/branch/master?svg=true)](https://ci.appveyor.com/project/Lokathor/safe-arch/branch/master)
[![crates.io](https://img.shields.io/crates/v/safe_arch.svg)](https://crates.io/crates/safe_arch)
[![docs.rs](https://docs.rs/safe_arch/badge.svg)](https://docs.rs/safe_arch/)

# safe_arch

Exposes arch-specific intrinsics as safe function (via cfg).

I don't like putting too much in the Readme and also in the crate docs, it can
get out of sync. Just see the [crate docs](https://docs.rs/safe_arch) for more
details.

* **Minimum Rust:** The CI tests the crate against Stable 1.43.0. The doc-tests
  are known to require at least 1.43 because they use `u32::MAX`, `f32::NAN`,
  and similar short names for the common consts. You can _probably_ build the
  crate without the doc tests on older compiler versions, possibly as far back
  as 1.31, but that's not officially supported.
