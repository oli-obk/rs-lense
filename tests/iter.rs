extern crate lense;

use lense::*;

#[test]
fn prim_immutable_iter() {
    let pool = AlignedPool::<u16>::with_capacity(3);
    {
        let ref mut buf = &*pool;
        let l = LenseRaw::from_buf(buf);
        let it: Iter<_, u16> = Iter::new(l);
        for n in it {
            assert_eq!(*n, 0u16);
        }
    }
}

#[test]
fn prim_mutable_iter() {
    let mut pool = AlignedPool::<u16>::with_capacity(3);
    {
        let ref mut buf = &mut *pool;
        let l = LenseRaw::from_buf(buf);
        let it: IterMut<_, u16> = IterMut::new(l);
        for mut n in it {
            *n = 12345;
        }
    }
}
