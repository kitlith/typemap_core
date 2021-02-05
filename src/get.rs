use super::{Sealed, Ty, TyEnd};

use core::any::Any;

// Each implementation of Contains corresponds with an implementation of TypeMapGet
#[cfg_attr(nightly, marker)]
pub trait Contains<T>: TypeMapGet {}

pub trait TypeMapGet: Sealed {
    fn try_get<T: 'static>(&self) -> Option<&T>;
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
    fn test_get_impl() {
        type Test = typemap_ty!(u64, u32, u16, u8);
        let mut test: Test = typemap!(u64 = 4u64, u32 = 3u32, u16 = 2u16, u8 = 1u8);

        assert_eq!(test.get::<u8>(), &1u8);
        assert_eq!(test.get::<u16>(), &2u16);
        assert_eq!(test.get::<u32>(), &3u32);
        assert_eq!(test.get::<u64>(), &4u64);
        assert_eq!(test.try_get::<u128>(), None);

        let test_ref: typemap_ty!(u128, ..&mut Test) = typemap!(u128 = 5u128, ..&mut test);
        assert_eq!(test_ref.get::<u8>(), &1u8);
        assert_eq!(test_ref.get::<u128>(), &5u128);

        let test_ref: typemap_ty!(u128, ..&Test) = typemap!(u128 = 5u128, ..&test);
        assert_eq!(test_ref.get::<u8>(), &1u8);
        assert_eq!(test_ref.get::<u128>(), &5u128);

        let test: typemap_ty!(u128, ..Test) = typemap!(u128 = 5u128, ..test);
        assert_eq!(test.get::<u8>(), &1u8);
        assert_eq!(test.get::<u128>(), &5u128);
    }
}
