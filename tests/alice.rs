#[macro_use] extern crate lense;

use lense::*;

mk_lense_ty!{struct AliceRef ref
    a: u8,
    b: (u8, u16),
    d: u32,
    e: u64,
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
    let ref mut pool = &mut *AlignedPool::<FakeAlice>::with_capacity(1);
    {
        let (a, b, c, d, e) = FakeAlice::slice(pool);
        assert_eq!(*a, 0u8);
        assert_eq!(*b, 0u8);
        assert_eq!(*c, 0u16);
        assert_eq!(*d, 0u32);
        assert_eq!(*e, 0u64);
    }
}

#[test]
fn alice_pool() {
    let ref mut pool = &mut *AlignedPool::<AliceRef>::with_capacity(1);
    {
        let AliceRef { a, b, d, e } = AliceRef::slice(pool);
        let (b, c) = b;
        assert_eq!(*a, 0u8);
        assert_eq!(*b, 0u8);
        assert_eq!(*c, 0u16);
        assert_eq!(*d, 0u32);
        assert_eq!(*e, 0u64);
    }
}

#[test]
fn alice_immutable_iter() {
    let ref mut pool = AlignedPool::<AliceRef>::with_capacity(4);
    {
        let it: Iter<AliceRef> = Iter::from_aligned_pool(pool);
        for AliceRef { a, b, d, e } in it {
            let (b, c) = b;
            assert_eq!(*a, 0u8);
            assert_eq!(*b, 0u8);
            assert_eq!(*c, 0u16);
            assert_eq!(*d, 0u32);
            assert_eq!(*e, 0u64);
        }
    }
}
