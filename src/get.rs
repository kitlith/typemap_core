use super::{Sealed, Ty, TyEnd};

use core::any::Any;

/// The Contains trait marks a map as containing at least one instance of a given type that can be accessed
///
/// ```
/// use typemap_core::{typemap, Contains};
/// fn requires_u32<Opts: Contains<u32>>(opts: &Opts) {
///     println!("{}", opts.get::<u32>());
/// }
///
/// requires_u32(&typemap!(u32 = 42u32));
/// ```
///
/// On nightly, this trait functions as intended with the help of an unstable feature.
/// Unfortunately, it cannot be implemented as intended on stable at the moment,
/// so it is given a blanket implementation to ensure that bounds that work on stable
/// do not cause errors on nightly.
// Each implementation of Contains corresponds with an implementation of TypeMapGet
#[cfg_attr(nightly, marker)]
pub trait Contains<T>: TypeMapGet {}

// TODO: more fleshed out trait documentation
/// The TypeMapGet trait allows for obtaining values of types from a typemap.
pub trait TypeMapGet: Sealed {
    /// Attempts to obtain a value of a given type from the map.
    ///
    /// This is mainly intended for the case where you don't require a type to be present,
    /// but would like to act on it if it is.
    /// Returns [`None`] if the type is not present in the map.
    /// On nightly, this should only occur if [`Contains<T>`] is not implemented.
    ///
    /// ```
    /// use typemap_core::{typemap, TypeMapGet};
    ///
    /// let map = typemap!(u32 = 23u32);
    ///
    /// assert_eq!(map.try_get::<u32>(), Some(&23u32));
    /// assert_eq!(map.try_get::<u128>(), None);
    /// ```
    fn try_get<T: 'static>(&self) -> Option<&T>;

    /// Obtains a value of a given type from the map.
    ///
    /// On nightly, you can only call this method if the type is actually present.
    /// (i.e. the map implements implements [`Contains<T>`])
    /// On stable, it panics on failure, containing a hint to use nightly to check your bounds at compile time.
    ///
    /// ```
    /// use typemap_core::{typemap, TypeMapGet};
    ///
    /// let map = typemap!(u32 = 23u32, &str = "Hello, world!");
    ///
    /// println!("{}", map.get::<&str>());
    /// ```
    fn get<T: 'static>(&self) -> &T
    where
        Self: Contains<T>,
    {
        self.try_get()
            .expect("Does not contain type! Check for errors by using the nightly compiler.")
    }
}

// Terminating impl. It contains no items, and there are no items past it that you can get.
impl TypeMapGet for TyEnd {
    fn try_get<T: 'static>(&self) -> Option<&T> {
        None
    }
}

// # Recursive impl. We are either returning the current item or delegating to the inner type.

// on nightly, when Contains<T> is implemented, TypeMapGet::try_get should never return None
#[cfg(nightly)]
mod nightly_contains_impls {
    use super::{Contains, Ty, TypeMapGet};

    impl<A: 'static, R: TypeMapGet> Contains<A> for Ty<A, R> {}

    // TODO: use Borrow?
    impl<A: 'static, B: 'static, R: Contains<B>> Contains<B> for Ty<A, R> {}
}

// on stable, we can't properly constrain the trait, so we just do a blanket impl and let it panic at runtime.
#[cfg(not(nightly))]
impl<A: 'static, B: 'static, R: TypeMapGet> Contains<A> for Ty<B, R> {}

// TODO: use Borrow?
impl<V: 'static, R: TypeMapGet> TypeMapGet for Ty<V, R> {
    fn try_get<T: 'static>(&self) -> Option<&T> {
        Any::downcast_ref::<T>(&self.val).or_else(|| self.rest.try_get::<T>())
    }
}

// # End Recursive impl
// # Reference Impls. Thin shims that delegate to the Recursive Impl for references

impl<A: 'static, B: 'static, R: TypeMapGet> Contains<A> for &Ty<B, R> where Ty<B, R>: Contains<A> {}

impl<V: 'static, R: TypeMapGet> TypeMapGet for &Ty<V, R> {
    fn try_get<T: 'static>(&self) -> Option<&T> {
        Ty::<V, R>::try_get(*self)
    }
}

impl<A: 'static, B: 'static, R: TypeMapGet> Contains<A> for &mut Ty<B, R> where Ty<B, R>: Contains<A>
{}

impl<V: 'static, R: TypeMapGet> TypeMapGet for &mut Ty<V, R> {
    fn try_get<T: 'static>(&self) -> Option<&T> {
        Ty::<V, R>::try_get(*self)
    }
}

// # End Reference Impls

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get() {
        type Test = typemap_ty!(u64, u32, u16, u8);
        let mut test: Test = typemap!(u64 = 4u64, u32 = 3u32, u16 = 2u16, u8 = 1u8);

        assert_eq!(test.get::<u8>(), &1u8);
        assert_eq!(test.get::<u16>(), &2u16);
        assert_eq!(test.get::<u32>(), &3u32);
        assert_eq!(test.get::<u64>(), &4u64);
        assert_eq!(test.try_get::<u128>(), None);

        let test_ref: typemap_ty!(u128, u16, ..&mut Test) =
            typemap!(u128 = 5u128, u16 = 16u16, ..&mut test);
        assert_eq!(test_ref.get::<u8>(), &1u8);
        assert_eq!(test_ref.get::<u16>(), &16u16);
        assert_eq!(test_ref.get::<u128>(), &5u128);

        assert_eq!(test.get::<u16>(), &2u16);

        let test_ref: typemap_ty!(u128, u16, ..&Test) =
            typemap!(u128 = 5u128, u16 = 16u16, ..&test);
        assert_eq!(test_ref.get::<u8>(), &1u8);
        assert_eq!(test_ref.get::<u16>(), &16u16);
        assert_eq!(test_ref.get::<u128>(), &5u128);

        assert_eq!(test.get::<u16>(), &2u16);

        let test: typemap_ty!(u128, u16, ..Test) = typemap!(u128 = 5u128, u16 = 16u16, ..test);
        assert_eq!(test.get::<u8>(), &1u8);
        // NOTE: making it so that you can access the original u16 gets a bit more difficult if you do this
        assert_eq!(test.get::<u16>(), &16u16);
        assert_eq!(test.get::<u128>(), &5u128);

        // TODO: we don't really have a *great* way of destructuring things at the moment.
        //  also not a publicly exposed way to do it.
        let Ty {
            val: u128_val,
            rest: Ty {
                val: u16_val,
                rest: test,
            },
        } = test;

        assert_eq!(u128_val, 5u128);
        assert_eq!(u16_val, 16u16);
        assert_eq!(test.get::<u16>(), &2u16)
    }
}
