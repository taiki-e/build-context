/*!
<!-- tidy:crate-doc:start -->
Make build environment/target information available as constants in normal libraries and binaries.

This is intended primarily for use in tests and its helpers. When used in libraries or binaries, be careful not to depend on constants that depend on the host environment.

Some constants duplicate those provided in `std::env::consts`.

<!-- tidy:crate-doc:end -->
 */

#![no_std]
#![doc(test(
    no_crate_inject,
    attr(
        deny(warnings, rust_2018_idioms, single_use_lifetimes),
        allow(dead_code, unused_variables)
    )
))]
#![forbid(unsafe_code)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    single_use_lifetimes,
    unreachable_pub
)]
#![warn(
    clippy::pedantic,
    // lints for public library
    clippy::alloc_instead_of_core,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
)]

include!(concat!(env!("OUT_DIR"), "/build-context"));
