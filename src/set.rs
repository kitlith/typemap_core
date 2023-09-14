use super::{Sealed, Ty, TyEnd};

use core::any::Any;

/// The ContainsMut trait marks a map as containing at least one instance of a given type that can be mutated
///
/// ```
/// use typemap_core::{typemap, TypeMapGet, ContainsMut};
/// fn modifies_u32<Opts: ContainsMut<u32>>(opts: &mut Opts) {
///     opts.set::<u32>(1337);
/// }
///
/// let mut map = typemap!(u32 = 42u32);
/// modifies_u32(&mut map);
/// println!("{}", map.get::<u32>());
/// ```
///
/// On nightly, this trait functions as intended with the help of an unstable feature.
/// Unfortunately, it cannot be implemented as intended on stable at the moment,
/// so it is given a blanket implementation to ensure that bounds that work on nightly
/// do not cause errors on stable.
// Each implementation of ContainsMut corresponds with an implementation of TypeMapSet
#[cfg_attr(nightly, marker)]
pub trait ContainsMut<T>: TypeMapSet {}

// TODO: more fleshed out trait documentation
/// The TypeMapSet trait allows for setting values of types in a typemap.
pub trait TypeMapSet: Sealed {
    /// Attempts to mutably set a value of a given type in the map.
    ///
    /// This is mainly intended for the case where you don't require a type to be present,
    /// but would like to act on it if it is.
    /// Returns [`false`] if the type is not present in the map.
    /// On nightly, this should only occur if [`ContainsMut<T>`] is not implemented.
    ///
    /// ```
    /// use typemap_core::{typemap, TypeMapGet, TypeMapSet};
    ///
    /// let mut map = typemap!(u32 = 13u32);
    /// assert!(map.try_set(42u32));
    /// assert_eq!(map.get::<u32>(), &42);
    ///
    /// // type is not present in map
    /// assert!(!map.try_set(1u128));
    /// ```
    fn try_set<T: 'static>(&mut self, value: T) -> bool;

    /// Mutable sets a value of a given type in the map.
    ///
    /// On nightly, you can only call this method if the type is actually present.
    /// (i.e. the map implements implements [`ContainsMut<T>`])
    ///
    /// # Panics
    ///
    /// This function panics if [`try_set`] would return [`false`].
    /// This should only be possible on stable,
    /// so you can weed out potential panics by occasionally checking against nightly.
    ///
    /// ```
    /// use typemap_core::{typemap, TypeMapGet, TypeMapSet};
    ///
    /// let mut map = typemap!(&str = "");
    /// map.set("Hello, world!");
    /// println!("{}", map.get::<&str>());
    /// ```
    ///
    /// [`try_set`]: #tymethod.try_set
    fn set<T: 'static>(&mut self, value: T)
    where
        Self: ContainsMut<T>,
    {
        assert!(
            self.try_set(value),
            "Cannot set type! Check for errors at compile-time by using nightly."
        )
    }
}

// # Terminating Impls -- You can't set any items on or beyond these types.

impl TypeMapSet for TyEnd {
    fn try_set<T: 'static>(&mut self, _value: T) -> bool {
        false
    }
}

impl<V: 'static, R: TypeMapSet> TypeMapSet for &Ty<V, R> {
    fn try_set<T: 'static>(&mut self, _value: T) -> bool {
        false
    }
}

// # End terminating impls
// # Recursive Impl -- We are either setting the current item or delegating to the inner type.

// on nightly, when ContainsMut<T> is implemented, TypeMapSet::try_set should never return false
#[cfg(nightly)]
mod nightly_impls {
    use super::{ContainsMut, Ty, TypeMapSet};

    impl<A: 'static, R: TypeMapSet> ContainsMut<A> for Ty<A, R> {}
    // TODO: use BorrowMut?
    impl<A: 'static, B: 'static, R: ContainsMut<B>> ContainsMut<B> for Ty<A, R> {}
}

// on stable, we can't properly constrain the trait, so we just do a blanket impl and let it panic at runtime.
#[cfg(not(nightly))]
impl<A: 'static, B: 'static, R: TypeMapSet> ContainsMut<A> for Ty<B, R> {}

// TODO: use BorrowMut?
impl<V: 'static, R: TypeMapSet> TypeMapSet for Ty<V, R> {
    fn try_set<T: 'static>(&mut self, value: T) -> bool {
        if let Some(val) = <dyn Any>::downcast_mut(&mut self.val) {
            *val = value;
            true
        } else {
            self.rest.try_set(value)
        }
    }
}

// # End impls on Ty<V, R>
// # Reference Impl -- Thin shim that delegates to the Recursive Impl for mutable references

impl<A: 'static, B: 'static, R: TypeMapSet> ContainsMut<A> for &mut Ty<B, R> where
    Ty<B, R>: TypeMapSet + ContainsMut<A>
{
}

impl<V: 'static, R: TypeMapSet> TypeMapSet for &mut Ty<V, R> {
    fn try_set<T: 'static>(&mut self, value: T) -> bool {
        (**self).try_set(value)
    }
}

// # End Reference Impl
