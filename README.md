<!-- cargo-sync-readme start -->

# `hs-bindgen`

Handy macro to generate C-FFI bindings to Rust for Haskell.

This library intended to work best in a project configured by
[`cabal-pack`](https://github.com/yvan-sraka/cabal-pack).

**N.B.** The MSRV is **1.64.0** since it use `core_ffi_c` feature.

## Examples

A minimal example would be to have a function annotated like this:

```rust
use hs_bindgen::*;

/// Haskell type signature are auto-magically inferred from Rust function
/// type! This feature could slow down compilation, and be enabled with:
/// `hs-bindgen = { ..., features = [ "full" ] }`
#[hs_bindgen]
fn greetings(name: &str) {
    println!("Hello, {name}!");
}
```

This will be expanded to (you can try yourself with `cargo expand`):

```rust
use hs_bindgen::*;

fn greetings(name: &str) {
    println!("Hello, {name}!");
}

#[no_mangle] // Mangling randomize symbols
extern "C" fn __c_greetings(__0: *const core::ffi::c_char) -> () {
    // `traits` module is `hs-bindgen::hs-bindgen-traits`
    // n.b. do not forget to import it, e.g., with `use hs-bindgen::*`
    traits::ReprC::from(greetings(traits::ReprRust::from(__0),))
}
```

A more complete example, when we now try to pass a custom type to our
interface:

```rust
use hs_bindgen::{traits::ReprRust, *};
use std::marker::PhantomData;

/// A custom Rust data-type, `#[repr(transparent)]` is not useful here
/// since `ReprRust` trait will offers the constructor we need to construct
/// our type out of a C-FFI safe primitive data-structure.
struct User<T: Kind> {
    name: String,
    kind: PhantomData<T>,
}

/** Overly engineered traits definitions just for the sake of demonstrating
limitations of this example, this isn't at all needed by default */

struct Super;

trait Kind {
    fn greet(name: &str) -> String;
}

impl Kind for Super {
    fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }
}

/// Declare targeted Haskell signature, return types should be wrapped in
/// an IO Monad (a behavior enforced by safety concerns)
#[hs_bindgen(hello :: CString -> IO CString)]
fn hello(user: User<Super>) -> String {
    Super::greet(&user.name)
}

/** n.b. functions wrapped by `#[hs_bindgen]` macro couldn't be
parametrized by generics (because monomorphisation occurs after macro
expansion during compilation, and how rustc assign unmangled symbols to
monomorphised methods are AFAIK not a publicly specified behavior), but
this limitation didn’t apply to `hs-bindgen-traits` implementations! */

impl<T: Kind> ReprRust<*const i8> for User<T> {
    fn from(ptr: *const i8) -> Self {
        User::<T> {
            name: <String as ReprRust<*const i8>>::from(ptr),
            kind: PhantomData::<T>
        }
    }
}
```

## Design

First, I would thank [Michael Gattozzi](https://twitter.com/mgattozzi) who
implement [a (no longer maintained) implementation](https://github.com/mgattozzi/curryrs)
to binding generation between Rust and Haskell and
[his writings](https://blog.mgattozzi.dev/haskell-rust/) and guidance
really help me to quick start this project.

I try to architect `hs-bindgen` with these core design principles:

- **Simplicity:** as KISS UNIX philosophy of minimalism, meaning here I
  tried to never re-implement feature already handled by Rust programming
  language (parsing code, infer types, etc.), I rather rely on capabilities
  of macro and trait systems. E.g. the only bit of parsing left in this
  code its Haskell function signature (which is trivial giving the feature
  set of authorized C-FFI safe types) ;

- **Modularity:** this library is design in mind to work in a broader range
  of usage, so this library should work in `#[no_std]` setting and most
  features could be opt-out. E.g. the type inference offered by
  [`antlion`](https://github.com/yvan-sraka/antlion) library is optional ;

- **Stability:** this library implements no trick outside the scope of
  stable C ABI (with well-defined memory layout convention), and ensure to
  provide ergonomics without breaking this safety rule of thumb. There is
  no magic that could be break by any `rustc` or GHC update!

## Acknowledgments

⚠️ This is still a working experiment, not yet production ready.

`hs-bindgen` was heavily inspired by other interoperability initiatives, as
[`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) and
[`PyO3`](https://github.com/PyO3/pyo3).

This project was part of a work assignment as an
[IOG](https://github.com/input-output-hk) contractor.

## License

Licensed under either of [Apache License](LICENSE-APACHE), Version 2.0 or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

<!-- cargo-sync-readme end -->
