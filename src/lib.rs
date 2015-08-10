pub trait Lense {
    fn decode_mut(v: &mut [u8]) -> &mut Self; // Should this be unsafe fn?
    fn size() -> usize;
}

pub trait LenseView<'a, L> {
    fn new(ptr: &'a mut [u8]) -> (Self, &'a mut [u8]);
    fn size() -> usize;
}

pub trait IntoLenseView<'a> {
    type LenseView;
}

// View into primitive types

pub struct View<'a, T>(&'a mut T) where T: 'a;

impl<'a, L: Lense> LenseView<'a, L> for View<'a, L> {
    fn new(ptr: &'a mut [u8]) -> (Self, &'a mut [u8]) {
        let (v, rest) = ptr.split_at_mut(L::size());
        (View(L::decode_mut(v)), rest)
    }
    fn size() -> usize {
        L::size()
    }
}

impl<'a, L> IntoLenseView<'a> for L where L: Lense {
    type LenseView = View<'a, L>;
}

impl<'a, T: Lense> ::std::ops::Deref for View<'a, T> {
    type Target = T; 
    fn deref<'b>(&'b self) -> &'b T { self.0 }
}
impl<'a, T: Lense> ::std::ops::DerefMut for View<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T { self.0 }
}

// Implement standard numeric types
macro_rules! impl_prim {
    ($($ty:ty: $expr:expr),+$(,)*) => {$(
        impl Lense for $ty {
            #[inline]
            fn decode_mut(v: &mut [u8]) -> &mut Self {
                unsafe { &mut *(v[0..Self::size()].as_ptr() as *mut Self) }
            }
            #[inline]
            fn size() -> usize { $expr } // equal to the below
//          fn size() -> usize { ::std::mem::size_of::<Self>() }
        }
    )+};
}
impl_prim!{
    u8: 1,    i8: 1,
   u16: 2,   i16: 2,
   u32: 4,   i32: 4,   f32: 4,
   u64: 8,   i64: 8,   f64: 8,
}

// View into owned types

pub struct ViewOwned<T>(T);

impl<'a, L: Lense> ::std::ops::Deref for ViewOwned<L> {
    type Target = L; 
    fn deref<'b>(&'b self) -> &'b Self::Target { &self.0 }
}
impl<'a, L: Lense> ::std::ops::DerefMut for ViewOwned<L> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut Self::Target { &mut self.0 }
}

fn slice_lense_chunk<'a, L: Lense>(ptr: &mut &'a mut [u8]) -> &'a mut L {
    let mut x: &mut [u8] = &mut [];
    ::std::mem::swap(&mut x, ptr);
    let (v, rest) = x.split_at_mut(L::size());
    *ptr = rest;
    L::decode_mut(v)
}

#[macro_export]
macro_rules! __lense_void_helper { ($m:tt, $expr:expr) => ($expr) }
#[macro_export]
macro_rules! mk_lense_list {
    ($($n:expr => ($($m:ident)*)),* $(,)*) => ($(
        impl<'a, L> LenseView<'a, L> for ViewOwned<[&'a mut L; $n]> where L: Lense {
            #[allow(unused_variables)]
            fn new(mut ptr: &'a mut [u8]) -> (Self, &'a mut [u8]) {
                (ViewOwned([$(__lense_void_helper!($m, $crate::slice_lense_chunk(&mut ptr))),*]), ptr)
            }
            fn size() -> usize {
                $n * L::size()
            }
        }
        impl<'a, L> IntoLenseView<'a> for [L; $n] where L: Lense {
            type LenseView = ViewOwned<[&'a mut L; $n]>;
        }
        impl<'a, L> ::std::ops::Deref for ViewOwned<[&'a mut L; $n]> where L: Lense {
            type Target = [&'a mut L; $n]; 
            fn deref<'b>(&'b self) -> &'b Self::Target { &self.0 }
        }
        impl<'a, L> ::std::ops::DerefMut for ViewOwned<[&'a mut L; $n]> where L: Lense {
            fn deref_mut<'b>(&'b mut self) -> &'b mut Self::Target { &mut self.0 }
        }
    )*);
    ($($ty:ident)*) => (
        impl<'a, $($ty: Lense),*> LenseView<'a, ($($ty,)*)> for ViewOwned<($(&'a mut $ty,)*)> {
            fn new(mut ptr: &'a mut [u8]) -> (Self, &'a mut [u8]) {
                (ViewOwned(($(__lense_void_helper!($ty, $crate::slice_lense_chunk(&mut ptr)),)*)), ptr)
            }
            fn size() -> usize {
                let mut s = 0;
                $(s += $ty::size();)*
                s
            }
        }
        impl<'a, $($ty: Lense),*> IntoLenseView<'a> for ($($ty,)*) {
            type LenseView = ViewOwned<($(&'a mut $ty,)*)>;
        }
        impl<'a, $($ty: Lense),*> ::std::ops::Deref for ViewOwned<($(&'a mut $ty,)*)> {
            type Target = ($(&'a mut $ty,)*); 
            fn deref<'b>(&'b self) -> &'b Self::Target { &self.0 }
        }
        impl<'a, $($ty: Lense),*> ::std::ops::DerefMut for ViewOwned<($(&'a mut $ty,)*)> {
            fn deref_mut<'b>(&'b mut self) -> &'b mut Self::Target { &mut self.0 }
        }
    );
}
mk_lense_list!{A}
mk_lense_list!{A B}
mk_lense_list!{A B C}
mk_lense_list!{A B C D}
mk_lense_list!{A B C D E}
mk_lense_list!{A B C D E F}
mk_lense_list!{A B C D E F G}
mk_lense_list!{A B C D E F G H}
mk_lense_list!{A B C D E F G H I}
mk_lense_list!{A B C D E F G H I J}
mk_lense_list!{A B C D E F G H I J K}
mk_lense_list!{A B C D E F G H I J K L}
mk_lense_list!{A B C D E F G H I J K L M}
mk_lense_list!{
     0 => (),
     1 => (a),
     2 => (a a),
     3 => (a a a),
     4 => (a a a a),
     5 => (a a a a a),
     6 => (a a a a a a),
     7 => (a a a a a a a),
     8 => (a a a a a a a a),
     9 => (a a a a a a a a a),
    10 => (a a a a a a a a a a),
    11 => (a a a a a a a a a a a),
    12 => (a a a a a a a a a a a a),
    13 => (a a a a a a a a a a a a a),
    14 => (a a a a a a a a a a a a a a),
    15 => (a a a a a a a a a a a a a a a),
    16 => (a a a a a a a a a a a a a a a a),
    17 => (a a a a a a a a a a a a a a a a a),
    18 => (a a a a a a a a a a a a a a a a a a),
    19 => (a a a a a a a a a a a a a a a a a a a),
    20 => (a a a a a a a a a a a a a a a a a a a a),
    21 => (a a a a a a a a a a a a a a a a a a a a a),
    22 => (a a a a a a a a a a a a a a a a a a a a a a),
    23 => (a a a a a a a a a a a a a a a a a a a a a a a),
    24 => (a a a a a a a a a a a a a a a a a a a a a a a a),
    25 => (a a a a a a a a a a a a a a a a a a a a a a a a a),
    26 => (a a a a a a a a a a a a a a a a a a a a a a a a a a),
    27 => (a a a a a a a a a a a a a a a a a a a a a a a a a a a),
    28 => (a a a a a a a a a a a a a a a a a a a a a a a a a a a a),
    29 => (a a a a a a a a a a a a a a a a a a a a a a a a a a a a a),
    30 => (a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a),
    31 => (a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a),
    32 => (a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a a),
}
 
// User types

pub trait SafeLense<'a> {
    type Writer;
    fn from_buf(v: &'a mut [u8]) -> Result<(Self, usize), LenseError>;
    fn writer() -> Self::Writer;
    fn size() -> usize;
}
pub trait SafeWriter { }

pub trait UnsafeLense<'a>: SafeLense<'a> {
    unsafe fn writer_uninit() -> Self::Writer;
}
pub trait UnsafeWriter: SafeWriter {
    unsafe fn new_uninit() -> Self;
}

#[derive(Debug)]
pub enum LenseError {
    NothingToParse,
    UnexpectedSize, // reserved for variable length types
    Incomplete,
}

#[macro_export]
macro_rules! make_lense {
    (PUB struct $lense:ident: $($name:ident: $ty:ty),*) => (
        pub struct $lense<'a> {
            $($name: <$ty as $crate::IntoLenseView<'a>>::LenseView),*
        }
    );
    (PRIV struct $lense:ident: $($name:ident: $ty:ty),*) => (
        struct $lense<'a> {
            $($name: <$ty as $crate::IntoLenseView<'a>>::LenseView),*
        }
    );
    ($vis:ident, $lense:ident, $owned:ident,
        $($struct_item_name:ident: $struct_item_type:ty),*$(,)*
    ) => (
        make_lense!{$vis struct $lense: $($struct_item_name: $struct_item_type),*}
        impl<'a> $crate::SafeLense<'a> for $lense<'a> {
            type Writer = $owned;
            #[allow(unused_variables)] // let (,v) not used in last iteration
            fn from_buf(v: &'a mut [u8]) -> Result<(Self, usize), LenseError> {
                // [x] Ensure that chunks (starting, length) fit in the vector
                if v.len() == 0 { return Err($crate::LenseError::NothingToParse) }
                // [x] Error return for incomplete or invalid segments
                if v.len() < Self::size() { return Err($crate::LenseError::Incomplete) }

                $(let ($struct_item_name, v) =
                    <$struct_item_type as $crate::IntoLenseView<'a>>::LenseView::new(v);)*
                let a = $lense {
                    $($struct_item_name: $struct_item_name),*
                };
                Ok((a, Self::size() /* + variable length claimed data */))
            }
            fn writer() -> Self::Writer {
                $owned::new()
            }
            fn size() -> usize {
                let mut size = 0;
                $(size += <$struct_item_type as $crate::IntoLenseView<'a>>::LenseView::size();)*
                size
            }
        }
        // Want to pull this out of the macro
        pub struct $owned(Vec<u8>);
        impl $crate::SafeWriter for $owned { }
        impl $owned {
            fn new() -> Self {
                $owned(vec![0u8; <$lense>::size()])
            }
            fn borrow_lense<'b>(&'b mut self) -> $lense<'b> {
                $lense::from_buf(&mut *self.0).unwrap().0
            }
            #[allow(dead_code)]
            fn as_bytes(&self) -> &[u8] {
                &*self.0
            }
        }
    )
}

#[macro_export]
macro_rules! unsafe_writer {
    ($lense:ident, $owned:ident) => (
        impl<'a> $crate::UnsafeLense<'a> for $lense<'a> {
            unsafe fn writer_uninit() -> Self::Writer {
                <$owned>::new_uninit()
            }
        }
        impl $crate::UnsafeWriter for $owned {
            unsafe fn new_uninit() -> Self {
                let mut v = Vec::with_capacity(<$lense>::size());
                v.set_len(<$lense>::size());
                $owned(v)
            }
        }
    )
}
