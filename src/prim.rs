
#[macro_export]
macro_rules! mk_lense_ty {
    (@void $void:tt $expr:expr) => { $expr };

    (tuple $($ty:ident)*) =>
        { mk_lense_ty!{() tuple $($ty)* } };
    (array $($n:tt)*) =>
        { mk_lense_ty!{[] $(($n))*} };
    (pub struct $ident:ident $ref_mut:tt $($field:ident: $ty:ty),* $(,)*) =>
        { mk_lense_ty!{{} public $ident $ref_mut $($field: $ty),* } };
    (struct $ident:ident $ref_mut:tt $($field:ident: $ty:ty),* $(,)*) =>
        { mk_lense_ty!{{} private $ident $ref_mut $($field: $ty),* } };

    (prim $($ty:ty)+) => {$(
        impl<'a> $crate::Lense<'a> for $ty {
            #[inline]
            fn size() -> usize { ::std::mem::size_of::<$ty>() }
        }

        impl<'a> $crate::LenseRef<'a> for $ty {
            type Ref = &'a $ty;
            fn slice<L: $crate::Dice<'a>>(buf: &mut L) -> Self::Ref {
                buf.slice::<Self>()
            }
        }

        impl<'a> $crate::LenseMut<'a> for $ty {
            type Mut = &'a mut $ty;
            fn slice_mut<L: $crate::DiceMut<'a>>(buf: &mut L) -> Self::Mut {
                buf.slice_mut::<Self>()
            }
        }
    )+};

    (()) => { };
    (() $head:ident $($ty:ident)*) => {
        impl<'a, $($ty: 'a + $crate::Lense<'a>),*> $crate::Lense<'a> for ($($ty,)*) {
            fn size() -> usize {
                0usize $(+ $ty::size())*
            }
        }

        impl<'a, $($ty: 'a + $crate::LenseRef<'a>),*> $crate::LenseRef<'a> for ($($ty,)*) {
            type Ref = ($($ty::Ref,)*);
            #[allow(unused_variables)]
            fn slice<BB: $crate::Dice<'a>>(buf: &mut BB) -> Self::Ref {
                ($( <$ty>::slice(buf), )*)
            }
        }

        impl<'a, $($ty: 'a + $crate::LenseMut<'a>),*> $crate::LenseMut<'a> for ($($ty,)*) {
            type Mut = ($($ty::Mut,)*);
            #[allow(unused_variables)]
            fn slice_mut<BB: $crate::DiceMut<'a>>(buf: &mut BB) -> Self::Mut {
                ($( <$ty>::slice_mut(buf), )*)
            }
        }
        mk_lense_ty!{() $($ty)*}
    };

    ([]) => { };
    ([] ($n:expr) $(($m:expr))*) => {
        impl<'a, L: 'a + $crate::Lense<'a>> $crate::Lense<'a> for [L; $n] {
            fn size() -> usize {
                $n * L::size()
            }
        }

        impl<'a, L: 'a + $crate::LenseRef<'a>> $crate::LenseRef<'a> for [L; $n] {
            type Ref = [L::Ref; $n];

            #[allow(unused_variables)]
            fn slice<B: $crate::Dice<'a>>(buf: &mut B) -> Self::Ref {
                [$(mk_lense_ty!{ @void ($m) L::slice(buf) }),*]
            }
        }

        impl<'a, L: 'a + $crate::LenseMut<'a>> $crate::LenseMut<'a> for [L; $n] {
            type Mut = [L::Mut; $n];

            #[allow(unused_variables)]
            fn slice_mut<B: $crate::DiceMut<'a>>(buf: &mut B) -> Self::Mut {
                [$(mk_lense_ty!{ @void ($m) L::slice_mut(buf) }),*]
            }
        }
        mk_lense_ty!{[] $(($m))*}
    };

    ({} @struct public ref $ident:ident $($field:ident: $ty:ty),*) =>
        { #[allow(dead_code)] pub struct $ident<'a> { $($field: <$ty as $crate::LenseRef<'a>>::Ref),* } };
    ({} @struct public mut $ident:ident $($field:ident: $ty:ty),*) =>
        { #[allow(dead_code)] pub struct $ident<'a> { $($field: <$ty as $crate::LenseMut<'a>>::Mut),* } };
    ({} @struct private ref $ident:ident $($field:ident: $ty:ty),*) =>
        { #[allow(dead_code)] struct $ident<'a> { $($field: <$ty as $crate::LenseRef<'a>>::Ref),* } };
    ({} @struct private mut $ident:ident $($field:ident: $ty:ty),*) =>
        { #[allow(dead_code)] struct $ident<'a> { $($field: <$ty as $crate::LenseMut<'a>>::Mut),* } };
    ({} @impl $ident:ident size $($field:ident: $ty:ty),*) => {
        impl<'a> $crate::Lense<'a> for $ident<'a> {
            fn size() -> usize {
                0usize $(+ <$ty as $crate::Lense<'a>>::size())*
            }
        }
    };
    ({} @impl $ident:ident ref $($field:ident: $ty:ty),*) => {
        impl<'a> $crate::LenseRef<'a> for $ident<'a> {
            type Ref = $ident<'a>;

            fn slice<B: $crate::Dice<'a>>(buf: &mut B) -> Self::Ref {
                $ident { $($field: <$ty>::slice(buf)),* }
            }
        }
    };
    ({} @impl $ident:ident mut $($field:ident: $ty:ty),*) => {
        impl<'a> $crate::LenseMut<'a> for $ident<'a> {
            type Mut = $ident<'a>;

            fn slice_mut<B: $crate::DiceMut<'a>>(buf: &mut B) -> Self::Mut {
                $ident { $($field: <$ty>::slice_mut(buf)),* }
            }
        }
    };
    ({} $vis:ident $ident:ident $ref_mut:tt $($field:ident: $ty:ty),*) => {
        mk_lense_ty!{{} @struct $vis $ref_mut $ident $($field: $ty),*}
        mk_lense_ty!{{} @impl $ident size $($field: $ty),*}
        mk_lense_ty!{{} @impl $ident $ref_mut $($field: $ty),*}
    };
}

mk_lense_ty!{prim
    u8  i8
    u16 i16
    u32 i32 f32
    u64 i64 f64
}
mk_lense_ty!{tuple
    A B C D E F
    G H I J K L
}
mk_lense_ty!{array
    32 31 30 29 28 27 26 25
    24 23 22 21 20 19 18 17
    16 15 14 13 12 11 10  9
     8  7  6  5  4  3  2  1
     0
}
