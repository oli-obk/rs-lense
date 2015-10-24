
#[macro_use] mod prim;
mod iterator;
mod pool;
mod file;

pub use iterator::{Iter, IterMut};
pub use pool::AlignedPool;

pub trait Lense<'a> {
    fn size() -> usize;
}

pub trait LenseRef<'a>: Lense<'a> {
    type Ref;
    fn slice<B: Dice<'a>>(&mut B) -> Self::Ref;
}

pub trait LenseMut<'a>: Lense<'a> {
    type Mut;
    fn slice_mut<B: DiceMut<'a>>(&mut B) -> Self::Mut;
}

pub struct LenseRaw<'a, B: 'a + ?Sized> {
    buf: &'a mut B,
    len: usize,
}

impl<'a, B: ?Sized> LenseRaw<'a, B> {
    pub fn from_buf(buf: &'a mut B) -> Self {
        LenseRaw {
            buf: buf,
            len: 0
        }
    }

    fn align_for<L: Lense<'a>>(&mut self) {
        if self.len % L::size() > 0 {
            unreachable!("Bad alignment {} {}", self.len, L::size());
        }
    }
}

pub trait Dice<'a>: Sized {
    fn slice<L: Lense<'a>>(&mut self) -> &'a L;
    fn len(&self) -> usize;

    fn into_iter<L: LenseRef<'a>>(self) -> Iter<'a, Self, L> { Iter::new(self) }
}

pub trait DiceMut<'a>: Dice<'a> {
    fn slice_mut<L: Lense<'a>>(&mut self) -> &'a mut L;

    fn into_iter_mut<L: LenseMut<'a>>(self) -> IterMut<'a, Self, L> { IterMut::new(self) }
}

// Buffer type, empty pointer, split function
macro_rules! mk_lense_raw {
    (mut $ty:ty, $x:expr, $split:ident) => (
        mk_lense_raw!{$ty, $x, $split}
        impl<'a> DiceMut<'a> for LenseRaw<'a, $ty> {
            fn slice_mut<L: Lense<'a>>(&mut self) -> &'a mut L {
                self.align_for::<L>();
                let mut x: $ty = $x;
                ::std::mem::swap(&mut x, self.buf);
                let (v, rest) = x.$split(L::size());
                *self.buf = rest;
                self.len += L::size();
                unsafe { &mut *(v.as_ptr() as *mut L) }
            }
        }
    );
    ($ty:ty, $x:expr, $split:ident) => (
        impl<'a> Dice<'a> for LenseRaw<'a, $ty> {
            fn slice<L: Lense<'a>>(&mut self) -> &'a L {
                self.align_for::<L>();
                let mut x: $ty = $x;
                ::std::mem::swap(&mut x, self.buf);
                let (v, rest) = x.$split(L::size());
                *self.buf = rest;
                self.len += L::size();
                unsafe { &*(v.as_ptr() as *const L) }
            }

            fn len(&self) -> usize {
                self.buf.len()
            }
        }
    );
}

mk_lense_raw!{    &'a     [u8], &[],     split_at}
mk_lense_raw!{mut &'a mut [u8], &mut [], split_at_mut}
