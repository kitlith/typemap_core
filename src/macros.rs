// TODO: could probably be written in terms of a helper trait, to prevent the requirement of recursion?
// typemap_ty!(Type, Type2, ... , ..rest)
/// Helper for expressing the type of a typemap.
///
/// Takes a comma separated list of types,
/// optionally followed by a single type representing the rest of the list in the form `..RestType`.
///
/// For instance,
/// ```
/// # use typemap_core::typemap_ty;
/// type Example = typemap_ty!(u16, u8);
/// let _: typemap_ty!(u64, u32, ..&Example);
/// ```
/// expands to:
/// ```
/// # use typemap_core::{Ty, TyEnd};
/// type Example = Ty<u16, Ty<u8, TyEnd>>;
/// let _: Ty<u64, Ty<u32, &Example>>;
/// ```
#[macro_export]
macro_rules! typemap_ty {
    () => { $crate::TyEnd };
    ($ty:ty, $($rest:tt)*) => {
        $crate::Ty::<$ty, $crate::typemap_ty!($($rest)*)>
    };
    ($ty:ty) => { $crate::typemap_ty!($ty, ..$crate::typemap_ty!()) };
    (..$ty:ty) => { $ty }
}

// TODO: write in terms of insert, to prevent the requirement of recursion?
// typemap!(Type = val, Type2 = val2, ... , ..rest)
/// Helper for creating a value of a typemap.
///
/// Takes a comma separated list of expressions of the form `Type = expr`,
/// optionally followed by a single expr representing the rest of the list in the form `..rest`.
/// Expressions have [`core::convert::Into::into`] called on them implicitly
///
/// For instance,
/// ```
/// # use typemap_core::typemap;
/// let example = typemap!(u16 = 2u16, u8 = 1u8);
/// let _ = typemap!(u64 = 4u64, u32 = 3u32, ..&example);
/// ```
/// is equivilant to:
/// ```
/// # use typemap_core::{Ty, TyTerm};
/// # use core::convert::Into;
/// let example = Ty::<u16, _>::new(2u16.into(), Ty::<u8, _>::new(1u8.into(), TyTerm));
/// let _ = Ty::<u64, _>::new(4u64.into(), Ty::<u32, _>::new(3u32, &example));
/// ```
#[macro_export]
macro_rules! typemap {
    () => { $crate::TyEnd };
    ($ty:ty = $val:expr, $($rest:tt)* ) => {
        $crate::Ty::<$ty, _>::new(
            ::core::convert::Into::<$ty>::into($val),
            $crate::typemap!($($rest)*)
        )
    };
    ($ty:ty = $val:expr) => {
        $crate::typemap!($ty = $val, ..$crate::typemap!())
    };
    (..$final:expr) => { $final };
}
