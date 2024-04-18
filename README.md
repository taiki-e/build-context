# build-context

[![crates.io](https://img.shields.io/crates/v/build-context?style=flat-square&logo=rust)](https://crates.io/crates/build-context)
[![docs.rs](https://img.shields.io/badge/docs.rs-build--context-blue?style=flat-square&logo=docs.rs)](https://docs.rs/build-context)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)](#license)
[![msrv](https://img.shields.io/badge/msrv-1.31-blue?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![github actions](https://img.shields.io/github/actions/workflow/status/taiki-e/build-context/ci.yml?branch=main&style=flat-square&logo=github)](https://github.com/taiki-e/build-context/actions)

<!-- tidy:crate-doc:start -->
Make [build environment/target information](https://doc.rust-lang.org/nightly/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts) available as constants in normal libraries and binaries.

This is intended primarily for use in tests and its helpers. When used in libraries or binaries, be careful not to depend on constants that depend on the host environment.

Some constants duplicate those provided in `std::env::consts`.

<!-- tidy:crate-doc:end -->

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
