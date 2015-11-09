extern crate lense;

use lense::*;

#[test]
fn prim_immutable() {
    let ref mut pool = &mut *AlignedPool::<u16>::with_capacity(4);
    let n = unsafe { pool.dice::<u16>() };
    assert_eq!(*n, 0u16);
}

#[test]
fn prim_mutable() {
    let ref mut pool = &mut *AlignedPool::<u16>::with_capacity(4);
    let mut n = unsafe { pool.dice_mut::<u16>() };
    *n = 12345;
}
