// TODO: could probably be written in terms of a helper trait, to prevent the requirement of recursion?
/// typemap_ty!(Type, Type2, ... , @term = rest)
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
/// typemap!(Type = val, Type2 = val2, ... , rest)
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
