#![no_std]
#![cfg_attr(nightly, feature(marker_trait_attr))]

#[macro_use]
mod macros;

mod private {
    use super::Ty;

    pub trait Sealed {}
    impl<T: 'static, R> Sealed for Ty<T, R> {}
    impl<T: 'static, R> Sealed for &Ty<T, R> {}
    impl<T: 'static, R> Sealed for &mut Ty<T, R> {}
    impl Sealed for () {}
}

mod get;
mod set;

use private::Sealed;

use get::TypeMapGetImpl;
use set::TypeMapSetImpl;

pub use {get::Contains, set::ContainsMut};

#[derive(Clone, Default)]
pub struct Ty<V: 'static, R> {
    pub val: V,
    pub rest: R,
}

impl<V: 'static, R> Ty<V, R> {
    pub fn new(val: V, rest: R) -> Self {
        Ty { val, rest }
    }
}

// TODO: should I continue wrapping the Set and Get impl traits? or just expose them directly?
//  I'm leaning towards the latter since at the moment there's no way to guarantee
//  that you can call try_get()/try_set()
pub trait TypeCollection: Sized + private::Sealed {
    fn try_get<T: 'static>(&self) -> Option<&T>
    where
        Self: TypeMapGetImpl,
    {
        self.get_impl()
    }

    fn get<T: 'static>(&self) -> &T
    where
        Self: Contains<T>,
    {
        self.get_impl()
            .expect("Does not contain type! Check for errors by using the nightly compiler.")
    }

    #[must_use]
    fn try_set<T: 'static>(&mut self, value: T) -> bool
    where
        Self: TypeMapSetImpl,
    {
        self.set_impl(value)
    }

    fn set<T: 'static>(&mut self, value: T)
    where
        Self: ContainsMut<T>,
    {
        assert!(
            self.set_impl(value),
            "Cannot set type! Check for errors by using the nightly compiler."
        )
    }

    fn insert<T: 'static>(self, val: T) -> Ty<T, Self>
    where
        Ty<T, Self>: TypeCollection,
    {
        Ty { val, rest: self }
    }
    fn insert_ref<'a, T: 'static>(&'a self, val: T) -> Ty<T, &'a Self>
    where
        Ty<T, &'a Self>: TypeCollection,
    {
        Ty { val, rest: self }
    }
    fn insert_mut<'a, T: 'static>(&'a mut self, val: T) -> Ty<T, &'a mut Self>
    where
        Ty<T, &'a mut Self>: TypeCollection,
    {
        Ty { val, rest: self }
    }
}

impl<V: 'static, R> TypeCollection for Ty<V, R> {}

#[cfg(test)]
mod tests {
    use super::TypeCollection;

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
