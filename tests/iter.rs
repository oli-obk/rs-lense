extern crate lense;

use lense::*;

#[test]
fn prim_immutable_iter() {
    let ref mut pool = AlignedPool::<u16>::with_capacity(4);
    {
        let it: Iter<u16> = Iter::from_aligned_pool(pool);
        for n in it {
            assert_eq!(*n, 0u16);
        }
    }
}

#[test]
fn prim_mutable_iter() {
    let ref mut pool = AlignedPool::<u16>::with_capacity(4);
    {
        let it: IterMut<u16> = IterMut::from_aligned_pool(pool);
        for mut n in it {
            *n = 12345;
        }
    }
}
