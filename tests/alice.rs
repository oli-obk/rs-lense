#![cfg(feature = "fooo")]
#[macro_use] extern crate lense;

use lense::*;

mk_lense_ty!{struct AliceRef ref
    a: u8,
    b: (u8, u16),
    c: u32,
    d: u64,
}

mk_lense_ty!{pub struct BobRef ref
    a: ((), ),
    b: (u8, u8),
    c: u32,
    d: AliceRef<'a>,
}

#[test]
fn fake_alice_pool() {
    type FakeAlice = (u8, u8, u16, u32, u64);
    let ref mut pool = &*AlignedPool::<FakeAlice>::with_capacity(1);
    {
        let ref mut l = LenseRaw::from_buf(pool);
        let (a, b, c, d, e) = *l.slice::<FakeAlice>();
        assert_eq!(a, 0u8);
        assert_eq!(b, 0u8);
        assert_eq!(c, 0u16);
        assert_eq!(d, 0u32);
        assert_eq!(e, 0u64);
    }
}

#[test]
fn alice_pool() {
    let ref mut pool = &*AlignedPool::<AliceRef>::with_capacity(1);
    {
        let ref mut l = LenseRaw::from_buf(pool);
        let AliceRef { a, b, c, d } = AliceRef::slice(l);
        let (b1, b2) = b;
        assert_eq!(*a, 0u8);
        assert_eq!(*b1, 0u8);
        assert_eq!(*b2, 0u16);
        assert_eq!(*c, 0u32);
        assert_eq!(*d, 0u64);
    }
}

#[test]
fn alice_iter() {
    let ref mut pool = &*AlignedPool::<AliceRef>::with_capacity(4);
    {
        let l = LenseRaw::from_buf(pool);
        let it = l.into_iter::<AliceRef>();
        assert_eq!(it.len(), 4);
        for AliceRef { a, b, c, d } in it {
            let (b1, b2) = b;
            assert_eq!(*a, 0u8);
            assert_eq!(*b1, 0u8);
            assert_eq!(*b2, 0u16);
            assert_eq!(*c, 0u32);
            assert_eq!(*d, 0u64);
        }
    }
}
