#[macro_export]
macro_rules! typemap_ty {
    () => { () };
    ($ty:ty $(, $rest:ty)*) => {
        $crate::Ty::<$ty, $crate::typemap_ty!($($rest),*)>
    };
}

#[macro_export]
macro_rules! typemap {
    () => { () };
    ($ty:ty = $val:expr $(, $rest_ty:ty = $rest_val:expr)* ) => {
        <$crate::typemap_ty!($ty $(, $rest_ty)*)>::new(
            ::core::convert::Into::<$ty>::into($val),
            $crate::typemap!($($rest_ty = $rest_val),*)
        )
    };
}
