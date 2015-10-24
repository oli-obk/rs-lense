extern crate lense;

use lense::*;

#[test]
fn prim_immutable() {
    let pool = AlignedPool::<u16>::with_capacity(3);
    {
        let ref mut buf = &*pool;
        let mut l = LenseRaw::from_buf(buf);
        let n = l.slice::<u16>();
        assert_eq!(*n, 0u16);
    }
}

#[test]
fn prim_mutable() {
    let mut pool = AlignedPool::<u16>::with_capacity(3);
    {
        let ref mut buf = &mut *pool;
        let mut l = LenseRaw::from_buf(buf);
        let mut n = l.slice_mut::<u16>();
        *n = 12345; // "unused variable" directly influences the pool
    }
}
