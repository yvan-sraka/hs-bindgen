//! # `hs-bindgen`
//!
//! Handy macro to generate C-FFI bindings from Rust to Haskell.
//!
//! This library intended to work best in a project configured by
//! [`cabal-pack`](https://github.com/yvan-sraka/cabal-pack).
//!
//! # Examples
//!
//! A minimal example would be to have a function annoted like this:
//!
//! ```rust
//! use hs_bindgen::*;
//!
//! /// Declare targeted Haskell signature
//! #[hs_bindgen(hello :: CString -> IO ())]
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
//! #[no_mangle] // Mangling randomize symbols
//! extern "C" fn __c_greetings(__0: *const std::os::raw::c_char) {
//!     greetings(traits::ReprC::from(__0))
//! }
//! ```
//!
//! A more complete example, when we now try to pass a custom type to our
//! interface:
//!
//! ```rust
//! use hs_bindgen::{traits::ReprC, *};
//!
//! /// A custom Rust data-type
//! struct User {
//!     name: String,
//! }
//!
//! /// Implementation of the helper trait require by `hs_bindgen`
//! impl ReprC<*const i8> for User {
//!     fn from(ptr: *const i8) -> Self {
//!         User {
//!             name: <String as ReprC<*const i8>>::from(ptr),
//!         }
//!     }
//! }
//!
//! #[hs_bindgen(hello :: CString -> IO ())]
//! fn hello(user: User) {
//!     println!("Hello, {}!", user.name);
//! }
//! ```
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

pub use hs_bindgen_derive::hs_bindgen;
pub use hs_bindgen_traits as traits;
