#![no_std]
#![cfg_attr(nightly, feature(marker_trait_attr))]

#[macro_use]
mod macros;

mod private {
    use super::{Ty, TyTerm};

    pub trait Sealed {}
    impl<T: 'static, R> Sealed for Ty<T, R> {}
    impl<T: 'static, R> Sealed for &Ty<T, R> {}
    impl<T: 'static, R> Sealed for &mut Ty<T, R> {}
    impl Sealed for TyTerm {}
}

mod get;
mod set;

use private::Sealed;

pub use {
    get::{Contains, TypeMapGet},
    set::{ContainsMut, TypeMapSet},
};

#[derive(Clone, Default)]
pub struct Ty<V: 'static, R> {
    pub val: V,
    pub rest: R,
}

pub struct TyTerm;

impl<V: 'static, R> Ty<V, R> {
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
