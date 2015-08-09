pub struct View<'a, T>(&'a mut T) where T: 'a;

impl<'a, T: Lense> View<'a, T> {
    pub fn new(ptr: &'a mut [u8]) -> (Self, &'a mut [u8]) {
        let (v, rest) = ptr.split_at_mut(T::size());
        (View(T::decode_mut(v)), rest)
    }
}

impl<'a, T: Lense> ::std::ops::Deref for View<'a, T> {
    type Target = T;

    fn deref<'b>(&'b self) -> &'b T {
        self.0
    }
}

impl<'a, T: Lense> ::std::ops::DerefMut for View<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        self.0
    }
}

pub trait Lense {
    type EncodeTarget;
    fn decode_mut(v: &mut [u8]) -> &mut Self; // Should this be unsafe fn?
    fn encode(self) -> Self::EncodeTarget;
    fn size() -> usize;
}

pub trait LenseStruct<'a> {
    type Writer;
    fn from_buf(v: &'a mut [u8]) -> Result<(Self, usize), LenseError>;
    fn writer() -> Self::Writer;
    unsafe fn writer_uninit() -> Self::Writer;
    fn size() -> usize;
}

#[derive(Debug)]
pub enum LenseError {
    NothingToParse,
    UnexpectedSize,
    Incomplete,
}

// Implement standard numeric types
macro_rules! impls {
    ($($ty:ty: $expr:expr),+$(,)*) => {$(
        impl Lense for $ty {
            type EncodeTarget = [u8; $expr];
            #[inline]
            fn decode_mut(v: &mut [u8]) -> &mut Self {
                // Safe because:
                //   the size is already that of Self::size() in View::new;
                //   $ty is not a struct with pointers;
                //
                // This hack is required due to both transmute and transmute_copy requiring Sized
                // in which [u8] is not
                unsafe { &mut *(v[0..Self::size()].as_ptr() as *mut Self) }
            }
            #[inline]
            fn encode(self) -> Self::EncodeTarget {
                unsafe { ::std::mem::transmute(self) }
            }
            #[inline]
            fn size() -> usize { $expr } // equal to the below
//          fn size() -> usize { ::std::mem::size_of::<Self>() }
        }
    )+};
}
impls!{
    u8: 1,    i8: 1,
   u16: 2,   i16: 2,
   u32: 4,   i32: 4,
   u64: 8,   i64: 8,
}

// pub vs priv objects
#[macro_export]
macro_rules! make_lense {
    ($lense_name:ident, $owned_name:ident,
        $($struct_item_name:ident: $struct_item_type:ty),*$(,)*
    ) => (
        pub struct $lense_name<'a> {
            $($struct_item_name: $crate::View<'a, $struct_item_type>),*
        }
        impl<'a> $crate::LenseStruct<'a> for $lense_name<'a> {
            type Writer = $owned_name;
            #[allow(unused_variables)] // let (,v) not used in last iteration
            fn from_buf(v: &'a mut [u8]) -> Result<(Self, usize), LenseError> {
                // [x] Ensure that chunks (starting, length) fit in the vector
                if v.len() == 0 { return Err($crate::LenseError::NothingToParse) }
                // [x] Error return for incomplete or invalid segments
                if v.len() < Self::size() { return Err($crate::LenseError::Incomplete) }

                $(let ($struct_item_name, v) = $crate::View::new(v);)*
                let a = $lense_name {
                    $($struct_item_name: $struct_item_name),*
                };
                Ok((a, Self::size() /* + variable length claimed data */))
            }
            fn writer() -> Self::Writer {
                $owned_name::new()
            }
            unsafe fn writer_uninit() -> Self::Writer {
                $owned_name::new_uninit()
            }
            fn size() -> usize {
                let mut size = 0;
                $(size += <$struct_item_type>::size();)*
                size
            }
        }
        // Want to pull this out of the macro
        pub struct $owned_name(Vec<u8>);
        impl $owned_name {
            fn new() -> Self {
                $owned_name(vec![0u8; <$lense_name>::size()])
            }
            unsafe fn new_uninit() -> Self {
                let mut v = Vec::with_capacity(<$lense_name>::size());
                v.set_len(<$lense_name>::size());
                $owned_name(v)
            }
            fn borrow_lense<'b>(&'b mut self) -> $lense_name<'b> {
                $lense_name::from_buf(&mut *self.0).unwrap().0
            }
            #[allow(dead_code)]
            fn as_bytes(&self) -> &[u8] {
                &*self.0
            }
        }
    )
}
