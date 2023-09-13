# typemap_core
![Build Status](https://github.com/kitlith/typemap_core/workflows/Main/badge.svg)
[![Current Crates.io Version](https://img.shields.io/crates/v/typemap_core.svg)](https://crates.io/crates/typemap_core)
[![Current Documentation](https://docs.rs/typemap_core/badge.svg)](https://docs.rs/typemap_core)

*A no_std typemap with trait-based value-presence guarantees (on nightly)*  
or  
*A map from a type to a value of that type, without needing std/alloc*

## Context Example

```rust
use typemap_core::{typemap, Contains};

struct A;
struct ContextA(u8);

impl From<u8> for ContextA {
    fn from(val: u8) -> Self {
        Self(val)
    }
}

struct B;
struct ContextB(u16);

impl From<u16> for ContextB {
    fn from(val: u16) -> Self {
        Self(val)
    }
}

#[allow(unused)]
struct C {
    a: A,
    b: B,
}

trait ParseTrait<Ctx> {
    fn parse(context: &Ctx) -> Self;
}

impl<Ctx> ParseTrait<Ctx> for A
where
    Ctx: Contains<ContextA>,
{
    fn parse(context: &Ctx) -> Self {
        context.get::<ContextA>();
        A
    }
}

impl<Ctx> ParseTrait<Ctx> for B
where
    Ctx: Contains<ContextB>,
{
    fn parse(context: &Ctx) -> Self {
        context.get::<ContextB>();
        B
    }
}

impl<Ctx> ParseTrait<Ctx> for C
where
    A: ParseTrait<Ctx>,
    B: ParseTrait<Ctx>,
{
    fn parse(context: &Ctx) -> Self {
        Self {
            a: A::parse(context),
            b: B::parse(context)
        }
    }
}

fn main() {
    C::parse(&typemap!(ContextA = 0u8, ContextB = 10u16));
    
    // Will panic at runtime on stable, and produce a compilation error on nightly
    // C::parse(&typemap!(ContextA = 0u8));
}
```

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
