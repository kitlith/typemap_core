use super::{Sealed, Ty};

use core::any::Any;

#[cfg_attr(nightly, marker)]
pub trait ContainsMut<T>: TypeMapSetImpl {}

#[cfg(nightly)]
mod nightly_impls {
    use super::{ContainsMut, Ty, TypeMapSetImpl};

    impl<A: 'static, R: TypeMapSetImpl> ContainsMut<A> for Ty<A, R> {}
    // TODO: use BorrowMut?
    impl<A: 'static, B: 'static, R: ContainsMut<B>> ContainsMut<B> for Ty<A, R> {}
    impl<A: 'static, B: 'static, R: TypeMapSetImpl> ContainsMut<A> for &mut Ty<B, R> where
        Ty<B, R>: TypeMapSetImpl + ContainsMut<A>
    {
    }
}

#[cfg(not(nightly))]
mod stable_impls {
    use super::{ContainsMut, Ty, TypeMapSetImpl};

    impl<A: 'static, B: 'static, R: TypeMapSetImpl> ContainsMut<A> for Ty<B, R> {}
}

pub trait TypeMapSetImpl: Sealed {
    fn set_impl<T: 'static>(&mut self, value: T) -> bool;
}

impl TypeMapSetImpl for () {
    fn set_impl<T: 'static>(&mut self, _value: T) -> bool {
        false
    }
}

impl<V: 'static, R: TypeMapSetImpl> TypeMapSetImpl for &Ty<V, R> {
    fn set_impl<T: 'static>(&mut self, _value: T) -> bool {
        false
    }
}

// TODO: use BorrowMut?
impl<V: 'static, R: TypeMapSetImpl> TypeMapSetImpl for Ty<V, R> {
    fn set_impl<T: 'static>(&mut self, value: T) -> bool {
        if let Some(val) = Any::downcast_mut(&mut self.val) {
            *val = value;
            true
        } else {
            self.rest.set_impl(value)
        }
    }
}

impl<V: 'static, R: TypeMapSetImpl> TypeMapSetImpl for &mut Ty<V, R> {
    fn set_impl<T: 'static>(&mut self, value: T) -> bool {
        (**self).set_impl(value)
    }
}
