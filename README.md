<!-- cargo-sync-readme start -->

# `hs-bindgen`

Handy macro to generate C-FFI bindings from Rust to Haskell.

This library intended to work best in a project configured by
[`cabal-pack`](https://github.com/yvan-sraka/cabal-pack).

# Example

```rust
use hs_bindgen::*;

#[hs_bindgen]
fn greetings(name: &str) {
    println!("Hello, {name}!");
}
```

... will be expanded to ...

```rust
fn greetings(name: &str) {
    println!("Hello, {name}!");
}

#[no_mangle] // Mangling randomize symbols
extern "C" fn c_greetings(_0: *const std::os::raw::c_char) {
    greetings(hs_bindgen_traits::ReprC::from(_0))
}
```

## Acknowledgments

⚠️ This is still a working experiment, not yet production ready.

`hs-bindgen` was heavily inspired by other interoperability initiatives, as
[`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) and
[`PyO3`](https://github.com/PyO3/pyo3).

This project was part of a work assignment as an
[IOG](https://github.com/input-output-hk) contractor.

<!-- cargo-sync-readme end -->
