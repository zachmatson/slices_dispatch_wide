# `slices_dispatch_wide!`

[![Crates.io](https://img.shields.io/crates/v/slices_dispatch_wide)](https://crates.io/crates/slices_dispatch_wide)
[![docs.rs](https://img.shields.io/docsrs/slices_dispatch_wide)](https://docs.rs/slices_dispatch_wide)
[![Crates.io](https://img.shields.io/crates/l/slices_dispatch_wide)](https://crates.io/crates/slices_dispatch_wide)
[![Crates.io](https://img.shields.io/crates/d/slices_dispatch_wide)](https://crates.io/crates/slices_dispatch_wide)

---

A macro to dispatch vectorized math over slices using the `wide` crate for SIMD operations

This crate iterates over chunks of your slice, converts them to types from the
[wide](https://crates.io/crates/wide) crate, and dispatches some math over both those chunks
and the remaining scalar elements. This crate will not work for you in every situation,
does not do anything special for alignment, and probably won't beat the best possible hand-tuned
SIMD code. That being said, if it works for your use case it will make your life easier.

This crate will be most useful if:
- You need SIMD math operations (such as `sqrt`, `log`, `exp`, etc.) on *stable* Rust, today.
- You don't want to bring in non-Rust libraries or complicate your build process to do that.
- You need to iterate over multiple slices of the same length in lockstep, and modify
  at least one of them.
- You don't want to repeat yourself or write boilerplate.
- You don't need variable lane width.

## Examples

```rust
use slices_dispatch_wide::*;

let mut a = [1.0_f64, 2.0, 3.0, 4.0, 5.0, 6.0];
let b = [2.0_f64, 2.0, 2.0, 2.0, 2.0, 2.0];

// Dispatches using chunks/SIMD types of width 4
slices_dispatch_wide!(4, |a => a mut: f64, b => b: f64| {
    // We can mutate the slices when the mut keyword is specified as it is above
    a += b;
});
// Notice that the number of elements is not a multiple of the SIMD width, the remainder is
// taken care of with scalar operations
assert_eq!(a, [3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);

// We can also do some fancier math operations in place
slices_dispatch_wide!(2, |a => a mut: f64| {
    a = a.powf(2.0);
});
assert_eq!(a, [9.0, 16.0, 25.0, 36.0, 49.0, 64.0]);

// You can assign different names to the elements from the slices you use
// And you can mutate multiple variables at the same time
let mut some_container = ([1.0, 2.0, 3.0, 4.0, 5.0, 6.0], "test", 0);
let mut c = [4.0, 2.0, 1.0, 0.0, 1.0, 2.0];

slices_dispatch_wide!(4, |some_container.0 => sc mut: f64, c => d mut: f64| {
    sc += d;
    d += sc;
});
assert_eq!(some_container.0, [5.0, 4.0, 4.0, 4.0, 6.0, 8.0]);
assert_eq!(c, [9.0, 6.0, 5.0, 4.0, 7.0, 10.0]);

// If you need to get the result in a new array/vector, you can pre-allocate it
let mut d = [0.0; 6];
slices_dispatch_wide!(4, |some_container.0 => sc: f64, d => d mut: f64| {
    d = 2.0 * sc;
});
assert_eq!(d, [10.0, 8.0, 8.0, 8.0, 12.0, 16.0]);
```

Note that if the slice lengths don't match, this macro will panic
```should_panic
// This example will panic because the lengths are different

use slices_dispatch_wide::*;

let a = [0u32];
let b = [0u32, 1];
slices_dispatch_wide!(8, |a => a: u32, b => b: u32| {});
```

And the width must be a literal
```compile_fail
// This example will not compile because the width is not a literal

use slices_dispatch_wide::*;

let a = [0u32];
let b = [0u32];
slices_dispatch_wide!(4 + 4, |a => a: u32, b => b: u32| {});
```

### Caveats

- The SIMD types used (the combination of lane width and scalar type for each slice used) must
  exist in the [wide](https://crates.io/crates/wide) crate.
- The code in the block must be valid when the iteration variables are the given scalar type
  or the corresponding SIMD type.

## License

Licensed under any of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
* Zlib license ([LICENSE-ZLIB](LICENSE-ZLIB) or https://opensource.org/licenses/Zlib)

at your choice.
