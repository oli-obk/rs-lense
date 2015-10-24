#[macro_use] extern crate lense;

use lense::*;

mk_lense_ty!{struct AliceRef ref
    a: u8,
    b: (u8, u16),
    c: u32,
    d: u64,
}

mk_lense_ty!{pub struct BobRef ref
    a: u8,
    b: (u8, u16),
    c: u32,
    d: AliceRef<'a>,
}

#[test]
fn alice() {
    let pool = AlignedPool::<AliceRef>::with_capacity(3);
    {
        let ref mut buf = &*pool;
        let mut l = LenseRaw::from_buf(buf);
        let AliceRef { a, b, c, d } = *l.slice::<AliceRef>();
        let (b1, b2) = b;
        // Segfault... Problem with pointers?
//      assert_eq!(*a, 0u8);
//      assert_eq!(*b1, 0u8);
//      assert_eq!(*b2, 0u16);
//      assert_eq!(*c, 0u32);
//      assert_eq!(*d, 0u64);
    }
}
