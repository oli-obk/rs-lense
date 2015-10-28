#[macro_use] mod prim;
mod iterator;
mod pool;
mod file;
mod aligned;

pub use iterator::{Iter, IterMut};
pub use aligned::Aligned;
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

pub trait Dice<'a>: Sized {
    fn slice<L: Lense<'a>>(&mut self) -> &'a L;
    fn size(&self) -> usize;
}

pub trait DiceMut<'a>: Dice<'a> {
    fn slice_mut<L: Lense<'a>>(&mut self) -> &'a mut L;
}

macro_rules! mk_dice {
    (mut $ty:ty, $x:expr, $split:ident) => (
        mk_dice!{$ty, $x, $split}
        impl<'a> DiceMut<'a> for $ty {
            fn slice_mut<L: Lense<'a>>(&mut self) -> &'a mut L {
                let mut x: $ty = $x;
                ::std::mem::swap(&mut x, self);
                let (v, rest) = x.$split(L::size());
                *self = rest;
                unsafe { &mut *(v.as_ptr() as *mut L) }
            }
        }
    );
    ($ty:ty, $x:expr, $split:ident) => (
        impl<'a> Dice<'a> for $ty {
            fn slice<L: Lense<'a>>(&mut self) -> &'a L {
                let mut x: $ty = $x;
                ::std::mem::swap(&mut x, self);
                let (v, rest) = x.$split(L::size());
                *self = rest;
                unsafe { &*(v.as_ptr() as *const L) }
            }

            fn size(&self) -> usize {
                self.len()
            }
        }
    );
}

mk_dice!{    &'a     [u8], &[],     split_at}
mk_dice!{mut &'a mut [u8], &mut [], split_at_mut}
