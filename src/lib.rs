#[macro_use] mod prim;
mod iterator;
mod file;
mod pool;

pub use iterator::{LenseIteratable, LenseIterator};
pub use file::LenseFile;

pub trait Lense<'a> {
    fn new(ptr: &'a mut [u8]) -> (Self, &'a mut [u8]);
    fn size() -> usize;
}

#[doc(hidden)]
pub trait IntoLense<'a> {
    type Lense: Lense<'a>;
}

#[doc(hidden)]
pub fn slice_lense_chunk<'a, L: Lense<'a>>(ptr: &mut &'a mut [u8]) -> L {
    let mut x: &mut [u8] = &mut [];
    ::std::mem::swap(&mut x, ptr);
    let (v, rest) = L::new(x);
    *ptr = rest;
    v
}

// Implement user defined structs

// Used internally for testcases
pub fn test_lense_struct_alignment(mut x: &mut usize, mut m: &mut usize,
                                   a: usize, s: usize, n: &'static str) {
    assert!(*x % a == 0, "Field {} is misaligned. Expected offset {}, found {}", n, a, *x % a);
    if cfg!(feature = "strict_ordering") { // forwards vs reverse ordering?
        assert!(a > *m, "Field {} is misaligned. Alignment: {} < {}", n, a, *m);
    }
    if a > *m { *m = a }
    *x += s;
}
