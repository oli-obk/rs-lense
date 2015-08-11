pub trait Lense<'a> {
    fn new(ptr: &'a mut [u8]) -> (Self, &'a mut [u8]);
    fn size() -> usize;
}

#[doc(hidden)]
pub trait IntoLense<'a> {
    type Lense: Lense<'a>;
}

/// Continuous lense reader allowing the consumer to to read as many of their type that is present
/// in the stream. TODO: Replace with an Interator
pub trait ContinuousLenseReader<'a>: Lense<'a> + Sized {
    /// Start working or continue iterating over a byte stream collecting lenses one-by-one.
    fn from_buf(mut ptr: &mut &'a mut [u8]) -> Result<Option<Self>, ()> {
        match ptr.len() {
            // Complete, nothing more to read
            0 => Ok(None),
            // Incomplete entry, possibly to be stitched together
            x if x < Self::size() => Err(()),
            // Correct size found and able to be serialised directly into the consuming lense
            _ => Ok(Some(slice_lense_chunk(&mut ptr))),
        }
    }
}

#[doc(hidden)]
pub fn slice_lense_chunk<'a, L: Lense<'a>>(ptr: &mut &'a mut [u8]) -> L {
    let mut x: &mut [u8] = &mut [];
    ::std::mem::swap(&mut x, ptr);
    let (v, rest) = L::new(x);
    *ptr = rest;
    v
}

// Implement lense for primitive types, tuples and arrays

macro_rules! lense_prim {
    ($($ty:ty),+$(,)*) => {$(
        impl<'a> Lense<'a> for &'a mut $ty {
            #[inline]
            fn new(ptr: &'a mut [u8]) -> (Self, &'a mut [u8]) {
                let (v, rest) = ptr.split_at_mut(Self::size());
                (unsafe { &mut *(v.as_ptr() as *mut $ty) }, rest)
            }

            #[inline]
            fn size() -> usize { ::std::mem::size_of::<$ty>() }
        }

        impl<'a> IntoLense<'a> for $ty {
            type Lense = &'a mut $ty;
        }
    )+};
}

macro_rules! lense_tuple {
    (@tail $head:ident) => {};

    (@tail $head:ident $($ty:ident)+) => {
        lense_tuple!{ $($ty)+ }
    };

    ($($ty:ident)*) => {
        impl<'a, $($ty: Lense<'a>),*> Lense<'a> for ($($ty,)*) {
            fn new(mut ptr: &'a mut [u8]) -> (Self, &mut [u8]) {
                (($($crate::slice_lense_chunk::<'a, $ty>(&mut ptr),)*), ptr)
            }

            fn size() -> usize {
                0 $(+ $ty::size())*
            }
        }

        impl<'a, $($ty: IntoLense<'a>),*> IntoLense<'a> for ($($ty,)*) {
            type Lense = ($(<$ty as IntoLense<'a>>::Lense,)*);
        }

        lense_tuple!{ @tail $($ty)+ }
    };
}

macro_rules! lense_array {
    (@void ($x:expr) $expr:expr) => ($expr);

    () => ();

    (($n:expr) $(($m:expr))*) => {
        impl<'a, L> Lense<'a> for [L; $n] where L: Lense<'a> {
            fn new(mut v: &'a mut [u8]) -> (Self, &mut [u8]) {
                ([$(lense_array!( @void ($m) $crate::slice_lense_chunk(&mut v) )),*], v)
            }
            fn size() -> usize {
                $n * L::size()
            }
        }

        impl<'a, L: IntoLense<'a>> IntoLense<'a> for [L; $n] {
            type Lense = [<L as IntoLense<'a>>::Lense; $n];
        }

        lense_array!{ $(($m))* }
    }
}

lense_prim!{
    u8,    i8,
   u16,   i16,
   u32,   i32,   f32,
   u64,   i64,   f64,
}

lense_tuple!{A B C D E F G H I J K L M}
 
lense_array!{
    (32) (31) (30) (29) (28) (27) (26) (25)
    (24) (23) (22) (21) (20) (19) (18) (17)
    (16) (15) (14) (13) (12) (11) (10) ( 9)
    ( 8) ( 7) ( 6) ( 5) ( 4) ( 3) ( 2) ( 1)
    ( 0)
}

// Implement user defined structs

#[macro_export]
macro_rules! lense_struct {
    (@struct public $lense:ident: $($name:ident: $ty:ty),*) => {
        pub struct $lense<'a> {
            $($name: <$ty as $crate::IntoLense<'a>>::Lense),*
        }
    };

    (@struct private $lense:ident: $($name:ident: $ty:ty),*) => {
        struct $lense<'a> {
            $($name: <$ty as $crate::IntoLense<'a>>::Lense),*
        }
    };

    (@impl $vis:ident $lense:ident: $($name:ident: $ty:ty),*) => {
        lense_struct!{@struct $vis $lense: $($name: $ty),*}

        impl<'a> $crate::Lense<'a> for $lense<'a> {
            fn new(mut v: &'a mut [u8]) -> (Self, &mut [u8]) {
                ($lense {
                    $($name: $crate::slice_lense_chunk(&mut v)),*
                }, v)
            }

            fn size() -> usize {
                0 $(+ <$ty as $crate::IntoLense<'a>>::Lense::size())*
            }
        }

        impl<'a> $crate::IntoLense<'a> for $lense<'a> {
            type Lense = $lense<'a>;
        }

        impl<'a> $crate::ContinuousLenseReader<'a> for $lense<'a> { }
    };

    (pub $lense:ident: $($name:ident: $ty:ty),* $(,)*) => {
        lense_struct!{@impl public $lense: $($name: $ty),*}
    };

    ($lense:ident: $($name:ident: $ty:ty),* $(,)*) => {
        lense_struct!{@impl private $lense: $($name: $ty),*}
    };
}
