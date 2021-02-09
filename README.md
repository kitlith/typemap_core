# typemap_core
![Build Status](https://github.com/kitlith/typemap_core/workflows/Main/badge.svg)
[![Current Crates.io Version](https://img.shields.io/crates/v/typemap_core.svg)](https://crates.io/crates/typemap_core)

*A no_std typemap with trait-based value-presence guarantees (on nightly)*  
or  
*A map from a type to a value of that type, without needing std/alloc*

## Nightly

This crate contains the `Contains<T>` and `ContainsMut<T>` traits.
These traits are only implemented correctly on nightly due to missing features in stable,
When using this library, you are encouraged to (occasionally) use the nightly compiler
to catch errors in your constraints at compile-time rather than run-time,
even if you are otherwise targeting stable.

This crate will properly implement those traits on stable as soon as we find a way to do so,
but for now they are implemented for all instances of `Ty<T, Rest>`
so that code running on stable doesn't need to cfg out all instances of requiring those traits.

## License

Licensed under either of

* Apache License, Version 2.0
  (http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  (http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
