#[macro_use] extern crate lense;

use lense::*;

mk_lense_ty!{struct AliceRefx ref
    a: u8,        // 1
    // 1 padding
    b: (u8, u16), // 3
    d: u32,       // 4
    e: u64,       // 8
}

mk_lense_ty!{pub struct AliceRef ref
    e: u64,       // 8
    d: u32,       // 4
    b: (u16, u8), // 3
    a: u8,        // 1
}

#[test]
fn fake_alice_pool() {
    type FakeAlice = (u8, u8, u16, u32, u64);
    let ref mut pool = &mut *AlignedPool::<FakeAlice>::with_capacity(1);
    let (a, b, c, d, e) = unsafe { FakeAlice::slice(pool) };
    assert_eq!(*a, 0u8);
    assert_eq!(*b, 0u8);
    assert_eq!(*c, 0u16);
    assert_eq!(*d, 0u32);
    assert_eq!(*e, 0u64);
}

#[test]
fn alice_pool() {
    let ref mut pool = &mut *AlignedPool::<AliceRef>::with_capacity(1);
    let AliceRef { a, b, d, e } = unsafe { AliceRef::slice(pool) };
    let (b, c) = b;
    assert_eq!(*a, 0u8);
    assert_eq!(*b, 0u16);
    assert_eq!(*c, 0u8);
    assert_eq!(*d, 0u32);
    assert_eq!(*e, 0u64);
}

#[test]
fn alice_immutable_iter() {
    let ref mut pool = AlignedPool::<AliceRef>::with_capacity(4);
    let it = Iter::from_aligned_pool(pool);
    for AliceRef { a, b, d, e } in it {
        let (b, c) = b;
        assert_eq!(*a, 0u8);
        assert_eq!(*b, 0u16);
        assert_eq!(*c, 0u8);
        assert_eq!(*d, 0u32);
        assert_eq!(*e, 0u64);
    }
}
