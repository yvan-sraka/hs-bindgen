//! # `hs-bindgen`
//!
//! Handy macro to generate C-FFI bindings to Rust for Haskell.
//!
//! This library intended to work best in a project configured by
//! [`cargo-cabal`](https://github.com/yvan-sraka/cargo-cabal).
//!
//! **N.B.** The MSRV is **1.64.0** since it use `core_ffi_c` feature.
//!
//! ## Examples
//!
//! A minimal example would be to have a function annotated like this:
//!
//! ```rust
//! use hs_bindgen::*;
//!
//! /// Haskell type signatures are auto-magically inferred from Rust function
//! /// types! This feature could slow down compilation, and be enabled with:
//! /// `hs-bindgen = { ..., features = [ "full" ] }`
//! #[hs_bindgen]
//! fn greetings(name: &str) {
//!     println!("Hello, {name}!");
//! }
//! ```
//!
//! This will be expanded to (you can try yourself with `cargo expand`):
//!
//! ```rust
//! use hs_bindgen::*;
//!
//! fn greetings(name: &str) {
//!     println!("Hello, {name}!");
//! }
//!
//! #[no_mangle] // Mangling makes symbol names more difficult to predict.
//!              // We disable it to ensure that the resulting symbol is really `__c_greetings`.
//! extern "C" fn __c_greetings(__0: *const core::ffi::c_char) -> () {
//!     // `traits` module is `hs-bindgen::hs-bindgen-traits`
//!     // n.b. do not forget to import it, e.g., with `use hs-bindgen::*`
//!     traits::FromReprC::from(greetings(traits::FromReprRust::from(__0),))
//! }
//! ```
//!
//! A more complete example, that use `borsh` to serialize ADT from Rust to Haskell
//! can be found [here](https://github.com/yvan-sraka/hs-bindgen-borsh-example).
//!
//! ## Design
//!
//! First, I would thank [Michael Gattozzi](https://twitter.com/mgattozzi) who
//! implement [a (no longer maintained) implementation](https://github.com/mgattozzi/curryrs)
//! to binding generation between Rust and Haskell and
//! [his writings](https://blog.mgattozzi.dev/haskell-rust/) and guidance
//! really help me to quick start this project.
//!
//! I try to architect `hs-bindgen` with these core design principles:
//!
//! - **Simplicity:** as KISS UNIX philosophy of minimalism, meaning here I
//!   tried to never re-implement feature already handled by Rust programming
//!   language (parsing code, infer types, etc.), I rather rely on capabilities
//!   of macro and trait systems. E.g. the only bit of parsing left in this
//!   code its Haskell function signature (which is trivial giving the feature
//!   set of authorized C-FFI safe types) ;
//!
//! - **Modularity:** this library is design in mind to work in a broader range
//!   of usage, so this library should work in `#[no_std]` setting and most
//!   features could be opt-out. E.g. the type inference offered by
//!   [`antlion`](https://github.com/yvan-sraka/antlion) library is optional ;
//!
//! - **Stability:** this library implements no trick outside the scope of
//!   stable C ABI (with well-defined memory layout convention), and ensure to
//!   provide ergonomics without breaking this safety rule of thumb. There is
//!   no magic that could be break by any `rustc` or GHC update!
//!
//! ## Acknowledgments
//!
//! ⚠️ This is still a working experiment, not yet production ready.
//!
//! `hs-bindgen` was heavily inspired by other interoperability initiatives, as
//! [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) and
//! [`PyO3`](https://github.com/PyO3/pyo3).
//!
//! This project was part of a work assignment as an
//! [IOG](https://github.com/input-output-hk) contractor.
//!
//! ## License
//!
//! Licensed under either of [Apache License](LICENSE-APACHE), Version 2.0 or
//! [MIT license](LICENSE-MIT) at your option.
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in this project by you, as defined in the Apache-2.0 license,
//! shall be dual licensed as above, without any additional terms or conditions.

#![forbid(unsafe_code)]

pub use hs_bindgen_attribute::hs_bindgen;
pub use hs_bindgen_traits as traits;
