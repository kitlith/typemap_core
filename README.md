# typemap_core
*A no_std typemap with trait-based value-presence guarantees (on nightly)*  
or  
*A map from a type to a value of that type, without needing std/alloc*

This crate contains the `Contains<T>` and `ContainsMut<T>` traits.
These traits are only implemented correctly on nightly due to missing features in stable,
When using this library, you are encouraged to (occasionally) use the nightly compiler
to catch errors in your constraints at compile-time rather than run-time,
even if you are otherwise targeting stable.

This crate will properly implement those traits on stable as soon as we find a way to do so,
but for now they are implemented for all instances of `Ty<T, Rest>`
so that code running on stable doesn't need to cfg out all instances of requiring those traits.
