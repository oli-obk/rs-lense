#[macro_use] extern crate lense;

use lense::{Lense, SeekablePool};

// Correct ordering can lead to favourable padding dynamics (in this case, no padding required)
mk_lense_ty!{pub struct AliceRef ref
    e:  u64,       // 8
    d:  u32,       // 4
    bc: (u16, u8), // 3
    a:  u8,        // 1
} // 8 + 4 + 3 + 1 = 16

#[test]
fn tuple_alice_iter() {
    type FakeAlice = (u8, (u8, u16), u32, u64);
    let mut pool = SeekablePool::<FakeAlice>::with_capacity(1);
    for guard in pool.iter() {
        let (a, (b, c), d, e) = *guard;
        assert_eq!(*a, 0u8);
        assert_eq!(*b, 0u8);
        assert_eq!(*c, 0u16);
        assert_eq!(*d, 0u32);
        assert_eq!(*e, 0u64);
    }
}

#[test]
fn alice_iter() {
    let mut pool = SeekablePool::<AliceRef>::with_capacity(4);
    let it = pool.iter();

    assert_eq!(it.len(), 4);

    for guard in it {
        let AliceRef { a, bc: (b, c), d, e } = *guard;
        assert_eq!(*a, 0u8);
        assert_eq!(*b, 0u16);
        assert_eq!(*c, 0u8);
        assert_eq!(*d, 0u32);
        assert_eq!(*e, 0u64);
    }
}

#[test]
fn size_alice_16() {
    assert_eq!(AliceRef::size(), 16);
}
