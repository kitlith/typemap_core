#![no_std]
#![cfg_attr(nightly, feature(marker_trait_attr))]

use core::any::Any;

mod private {
    use super::Ty;

    pub trait Sealed {}
    impl<T: 'static, R: TypeCollectionImpl> Sealed for Ty<T, R> {}
    impl<T: 'static, R: TypeCollectionImpl> Sealed for &Ty<T, R> {}
    impl<T: 'static, R: TypeCollectionImpl> Sealed for &mut Ty<T, R> {}
    impl Sealed for () {}

    pub trait CanMut {}
    impl<T: 'static, R: TypeCollectionImpl> CanMut for Ty<T, R> {}
    impl<T: 'static, R: TypeCollectionImpl> CanMut for &mut Ty<T, R> {}

    pub trait TypeCollectionImpl: Sealed {
        fn get_impl<T: 'static>(&self) -> Option<&T>;
        fn set_impl<T: 'static>(&mut self, value: T) -> bool;
    }
}

use private::TypeCollectionImpl;

#[derive(Clone, Default)]
pub struct Ty<V: 'static, R: TypeCollectionImpl> {
    val: V,
    rest: R,
}

#[cfg_attr(nightly, marker)]
pub trait Contains<T>: private::Sealed {}

#[cfg_attr(nightly, marker)]
pub trait ContainsMut<T>: private::CanMut {}

#[cfg(nightly)]
mod nightly_impls {
    use super::{Contains, ContainsMut, Ty, TypeCollectionImpl};

    impl<A: 'static, R: TypeCollectionImpl> Contains<A> for Ty<A, R> {}
    impl<A: 'static, B: 'static, R: TypeCollectionImpl> Contains<B> for Ty<A, R> where R: Contains<B> {}
    impl<A: 'static, B: 'static, R: TypeCollectionImpl> Contains<A> for &Ty<B, R> where
        Ty<B, R>: Contains<A>
    {
    }
    impl<A: 'static, B: 'static, R: TypeCollectionImpl> Contains<A> for &mut Ty<B, R> where
        Ty<B, R>: Contains<A>
    {
    }

    impl<A: 'static, R: TypeCollectionImpl> ContainsMut<A> for Ty<A, R> {}
    impl<A: 'static, B: 'static, R: TypeCollectionImpl> ContainsMut<B> for Ty<A, R> where
        R: ContainsMut<B>
    {
    }
    impl<A: 'static, B: 'static, R: TypeCollectionImpl> ContainsMut<A> for &mut Ty<B, R> where
        Ty<B, R>: ContainsMut<A>
    {
    }
}

#[cfg(not(nightly))]
mod stable_impls {
    use super::{Contains, ContainsMut, Ty, TypeCollectionImpl};

    impl<A: 'static, B: 'static, R: TypeCollectionImpl> Contains<A> for Ty<B, R> {}
    impl<A: 'static, B: 'static, R: TypeCollectionImpl> Contains<A> for &Ty<B, R> {}

    impl<A: 'static, B: 'static, R: TypeCollectionImpl> ContainsMut<A> for Ty<B, R> {}
}

pub trait TypeCollection: private::Sealed {
    fn try_get<T: 'static>(&self) -> Option<&T>;
    fn get<T: 'static>(&self) -> &T
    where
        Self: Contains<T>;

    #[must_use]
    fn try_set<T: 'static>(&mut self, value: T) -> bool;
    fn set<T: 'static>(&mut self, value: T)
    where
        Self: ContainsMut<T>;

    fn insert_ref<'a, T: 'static>(&'a self, val: T) -> Ty<T, &'a Self>
    where
        &'a Self: TypeCollectionImpl,
    {
        Ty { val, rest: self }
    }
    fn insert_mut<'a, T: 'static>(&'a mut self, val: T) -> Ty<T, &'a mut Self>
    where
        &'a mut Self: TypeCollectionImpl,
    {
        Ty { val, rest: self }
    }
}

pub trait TypeCollectionExt: TypeCollection + TypeCollectionImpl + Sized {
    fn insert<T: 'static>(self, val: T) -> Ty<T, Self> {
        Ty { val, rest: self }
    }
}

impl TypeCollectionImpl for () {
    fn get_impl<T: 'static>(&self) -> Option<&T> {
        None
    }

    fn set_impl<T: 'static>(&mut self, _value: T) -> bool {
        false
    }
}

impl<V: 'static, R: TypeCollectionImpl> TypeCollectionImpl for Ty<V, R> {
    fn get_impl<T: 'static>(&self) -> Option<&T> {
        Any::downcast_ref::<T>(&self.val).or_else(|| self.rest.get_impl::<T>())
    }

    fn set_impl<T: 'static>(&mut self, value: T) -> bool {
        if let Some(val) = Any::downcast_mut(&mut self.val) {
            *val = value;
            true
        } else {
            self.rest.set_impl(value)
        }
    }
}

impl<V: 'static, R: TypeCollectionImpl> TypeCollectionImpl for &Ty<V, R> {
    fn get_impl<T: 'static>(&self) -> Option<&T> {
        (**self).get_impl()
    }
    fn set_impl<T: 'static>(&mut self, _value: T) -> bool {
        false
    }
}

impl<V: 'static, R: TypeCollectionImpl> TypeCollectionImpl for &mut Ty<V, R> {
    fn get_impl<T: 'static>(&self) -> Option<&T> {
        (**self).get_impl()
    }
    fn set_impl<T: 'static>(&mut self, value: T) -> bool {
        (**self).set_impl(value)
    }
}

impl<V: 'static, R: TypeCollectionImpl> TypeCollection for Ty<V, R> {
    fn try_get<T: 'static>(&self) -> Option<&T> {
        self.get_impl()
    }

    fn get<T: 'static>(&self) -> &T {
        self.get_impl().unwrap()
    }

    fn try_set<T: 'static>(&mut self, value: T) -> bool {
        self.set_impl(value)
    }

    fn set<T: 'static>(&mut self, value: T) {
        assert!(self.set_impl(value));
    }
}

// trait DoesAThing<Opts> {
//     //type Options;
// }
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
//
// fn main() {
//     let mut test = Test::default();
//
//     test.set(1u8);
//     test.set(2u16);
//     test.set(3u32);
//     test.set(4u64);
//     // errors as not implemented
//     //test.set(5u128);
//
//     test_thing::<Test>();
//
//     assert_eq!(*test.get::<u8>(), 1u8);
//     assert_eq!(*test.get::<u16>(), 2u16);
//     assert_eq!(*test.get::<u32>(), 3u32);
//     assert_eq!(*test.get::<u64>(), 4u64);
//     //assert_eq!(*test.get::<u128>(), 5u128);
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
