extern crate lense;

use lense::*;

#[test]
fn prim_immutable() {
    let ref mut pool = &mut *AlignedPool::<u16>::with_capacity(1);
    {
        let n = pool.slice::<u16>();
        assert_eq!(*n, 0u16);
    }
}

#[test]
fn prim_mutable() {
    let ref mut pool = &mut *AlignedPool::<u16>::with_capacity(1);
    {
        let mut n = pool.slice_mut::<u16>();
        *n = 12345;
    }
}
