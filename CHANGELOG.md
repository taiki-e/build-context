# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

## [0.1.3] - 2024-09-13

- Support `CARGO_CFG_TARGET_ABI`, `CARGO_CFG_TARGET_FEATURE`, and `CARGO_CFG_TARGET_HAS_ATOMIC`.

- Improve Windows host support by workaround [rustc limitation](https://github.com/rust-lang/rust/issues/75075).

## [0.1.2] - 2024-04-13

- Relax the minimum supported Rust version from Rust 1.45 to Rust 1.31.

## [0.1.1] - 2023-05-05

- Support `CARGO_CFG_PANIC`.

## [0.1.0] - 2023-05-01

Initial release

[Unreleased]: https://github.com/taiki-e/build-context/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/taiki-e/build-context/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/taiki-e/build-context/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/taiki-e/build-context/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/taiki-e/build-context/releases/tag/v0.1.0
