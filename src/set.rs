use super::{Sealed, Ty, TyTerm};

use core::any::Any;

// Each implementation of ContainsMut corresponds with an implementation of TypeMapSet
#[cfg_attr(nightly, marker)]
pub trait ContainsMut<T>: TypeMapSet {}

pub trait TypeMapSet: Sealed {
    fn try_set<T: 'static>(&mut self, value: T) -> bool;
    fn set<T: 'static>(&mut self, value: T)
    where
        Self: ContainsMut<T>,
    {
        assert!(
            self.try_set(value),
            "Cannot set type! Check for errors by using the nightly compiler."
        )
    }
}

// # Terminating Impls -- You can't set any items on or beyond these types.

impl TypeMapSet for TyTerm {
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
        if let Some(val) = Any::downcast_mut(&mut self.val) {
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
