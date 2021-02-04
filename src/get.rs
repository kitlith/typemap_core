use super::{Sealed, Ty};

use core::any::Any;

#[cfg_attr(nightly, marker)]
pub trait Contains<T>: TypeMapGetImpl {}

#[cfg(nightly)]
mod nightly_contains_impls {
    use super::{Contains, Ty, TypeMapGetImpl};

    impl<A: 'static, R: TypeMapGetImpl> Contains<A> for Ty<A, R> {}

    // TODO: use Borrow?
    impl<A: 'static, B: 'static, R: Contains<B>> Contains<B> for Ty<A, R> {}

    // the following are implemented in terms of the former

    impl<A: 'static, B: 'static, R: TypeMapGetImpl> Contains<A> for &Ty<B, R> where Ty<B, R>: Contains<A>
    {}

    impl<A: 'static, B: 'static, R: TypeMapGetImpl> Contains<A> for &mut Ty<B, R> where
        Ty<B, R>: Contains<A>
    {
    }
}

#[cfg(not(nightly))]
mod stable_contains_impls {
    use super::{Contains, Ty, TypeMapGetImpl};

    impl<A: 'static, B: 'static, R: TypeMapGetImpl> Contains<A> for Ty<B, R> {}
    impl<A: 'static, B: 'static, R: TypeMapGetImpl> Contains<A> for &Ty<B, R> {}
}

pub trait TypeMapGetImpl: Sealed {
    fn get_impl<T: 'static>(&self) -> Option<&T>;
}

impl TypeMapGetImpl for () {
    fn get_impl<T: 'static>(&self) -> Option<&T> {
        None
    }
}

// TODO: use Borrow?
impl<V: 'static, R: TypeMapGetImpl> TypeMapGetImpl for Ty<V, R> {
    fn get_impl<T: 'static>(&self) -> Option<&T> {
        Any::downcast_ref::<T>(&self.val).or_else(|| self.rest.get_impl::<T>())
    }
}

impl<V: 'static, R: TypeMapGetImpl> TypeMapGetImpl for &Ty<V, R> {
    fn get_impl<T: 'static>(&self) -> Option<&T> {
        Ty::<V, R>::get_impl(*self)
    }
}

impl<V: 'static, R: TypeMapGetImpl> TypeMapGetImpl for &mut Ty<V, R> {
    fn get_impl<T: 'static>(&self) -> Option<&T> {
        Ty::<V, R>::get_impl(*self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_impl() {
        let test = Ty {
            val: 4u64,
            rest: Ty {
                val: 3u32,
                rest: Ty {
                    val: 2u16,
                    rest: Ty { val: 1u8, rest: () },
                },
            },
        };

        assert_eq!(test.get_impl::<u8>(), Some(&1u8));
        assert_eq!(test.get_impl::<u16>(), Some(&2u16));
        assert_eq!(test.get_impl::<u32>(), Some(&3u32));
        assert_eq!(test.get_impl::<u64>(), Some(&4u64));
        assert_eq!(test.get_impl::<u128>(), None);
    }
}
