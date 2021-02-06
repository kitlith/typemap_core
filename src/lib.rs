//! A no_std, stack-based typemap with trait based value presence guarantees (on nightly)
//!
//! # Example
//!
//! ```
//! use typemap_core::{typemap, Contains, TypeMapGet};
//!
//! fn uses_options<Opts: Contains<&str> + Contains<u32>>(opts: &Opts) {
//!     println!("str: \"{}\", u32: {}", opts.get::<&str>(), opts.get::<u32>());
//! }
//!
//! let options = typemap!(u128 = 34u128, &str = "Hello, world!", u32 = 45u32);
//! uses_options(&options);
//! ```
//!
//! # Nightly
//! Nightly is not required to use this library, but it is reccomended to at least check your code on nightly
//! occasionally given the nature of the [`Contains<T>`] and [`ContainsMut<T>`] traits.
//! On nightly, these traits ensure that you can only call the methods that panic
//! when they are guaranteed not to panic.
//! On stable, we can't implement it properly, so it merely has a blanket impl so that your trait bounds
//! setup for nightly do not cause issues on stable.
#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(nightly, feature(marker_trait_attr))]

#[macro_use]
mod macros;

mod private {
    use super::{Ty, TyEnd};

    pub trait Sealed {}
    impl<T: 'static, R> Sealed for Ty<T, R> {}
    impl<T: 'static, R> Sealed for &Ty<T, R> {}
    impl<T: 'static, R> Sealed for &mut Ty<T, R> {}
    impl Sealed for TyEnd {}
}

mod get;
mod set;

use private::Sealed;

pub use {
    get::{Contains, TypeMapGet},
    set::{ContainsMut, TypeMapSet},
};

/// A type-level linked-list implementation of a typemap
///
/// The generic arguments can be treated as if this were a cons cell. i.e.
/// ```
/// use typemap_core::{Ty, TyEnd};
/// type Example = Ty<u32, Ty<u16, Ty<u8, TyEnd>>>;
/// ```
/// is akin to `(cons u32 (cons u16 (cons u8 nil)))` at the type level,
/// and creates a storage capable of holding a u32, a u16, and a u8.
///
/// A couple of helper macros, [`typemap_ty!()`] and [`typemap!()`] exist, for ease of definition.
/// ```
/// use typemap_core::{typemap, typemap_ty};
/// type Example = typemap_ty!(u32, u16, u8);
/// let example: Example = typemap!(u32 = 0u32, u16 = 1337u16, u8 = 255u8);
/// ```
///
/// As a linked list, it is fairly easy to prepend additional items:
/// ```
/// # use typemap_core::typemap;
/// # let example = typemap!(u32 = 0u32, u16 = 1337u16, u8 = 255u8);
/// let extended = typemap!(&str = "Hello!", ..example);
/// ```
///
/// Which also allows you to "override" existing values temporarilly.
/// ```
/// use typemap_core::{typemap, TypeMapGet};
/// let greeting_options = typemap!(&str = "Hello!");
/// let rude_greeting = typemap!(&str = "Go away.", ..&greeting_options);
/// assert_eq!(rude_greeting.get::<&str>(), &"Go away.");
/// drop(rude_greeting);
/// assert_eq!(greeting_options.get::<&str>(), &"Hello!");
/// ```
///
/// See the [`TypeMapGet`] and [`TypeMapSet`] traits for more details.
#[derive(Clone, Default)]
pub struct Ty<V: 'static, R> {
    val: V,
    rest: R,
}

/// The end of a typemap.
///
/// Following the analogy of [`Ty`] to a cons cell, [`TyEnd`] is akin to nil.
///
/// See [`Ty`] for more details.
pub struct TyEnd;

impl<V: 'static, R> Ty<V, R> {
    /// Construct a node of a typemap
    pub fn new(val: V, rest: R) -> Self {
        Ty { val, rest }
    }
}

// pub trait TypeMapInsert: Sized + private::Sealed {
//     fn insert<T: 'static>(self, val: T) -> Ty<T, Self>
//     where
//         Ty<T, Self>: TypeMapInsert,
//     {
//         Ty { val, rest: self }
//     }
//     fn insert_ref<'a, T: 'static>(&'a self, val: T) -> Ty<T, &'a Self>
//     where
//         Ty<T, &'a Self>: TypeMapInsert,
//     {
//         Ty { val, rest: self }
//     }
//     fn insert_mut<'a, T: 'static>(&'a mut self, val: T) -> Ty<T, &'a mut Self>
//     where
//         Ty<T, &'a mut Self>: TypeMapInsert,
//     {
//         Ty { val, rest: self }
//     }
// }
//
// impl<V: 'static, R> TypeMapInsert for Ty<V, R> {}

#[cfg(test)]
mod tests {
    use super::{TypeMapGet, TypeMapSet};

    // trait DoesAThing<Opts> {}
    //
    // struct A;
    // struct B;
    // struct C;
    //
    // impl<Opts> DoesAThing<Opts> for A where Opts: Contains<u32> {}
    // impl<Opts> DoesAThing<Opts> for B where Opts: Contains<u16> {}
    // impl<Opts> DoesAThing<Opts> for C
    // where
    //     Opts: Contains<u8>,
    //     B: DoesAThing<Opts>,
    //     A: DoesAThing<Opts>,
    // {
    // }
    //
    // type Test = Ty<u64, Ty<u32, Ty<u16, Ty<u8, ()>>>>;
    //
    // fn test_thing<T>()
    // where
    //     C: DoesAThing<T>,
    // {
    // }

    #[test]
    fn test_set_get() {
        let mut test = typemap!(u32 = 32u32, u64 = 64u64);
        assert_eq!(test.get::<u32>(), &32);
        assert_eq!(test.get::<u64>(), &64);

        test.set(1u32);
        test.set(2u64);
        assert_eq!(test.get::<u32>(), &1);
        assert_eq!(test.get::<u64>(), &2);
    }
}
