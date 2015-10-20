#![allow(dead_code)]
#[macro_use] extern crate lense;

// All fields must align at modules of their size.
// A struct is cyclic safe if the sizes `mod` the largest alignment equals zero.

/*
lense_struct!{BadAlignment:
    a:  u8,
    b: (u8, u8),
    c: [u8; 4],
    d: u64,
} // (1 + 2 + 4 + 8) % 8 == 15 % 8 == 7
*/

lense_struct!{GoodAlignment:
    padding: u8,
    a:  u8,
    b: (u8, u8),
    c: [u8; 4],
    d: u64,
} // (1 + 1 + 2 + 4 + 8) % 8 == 16 % 8 == 0

/*
lense_struct!{BadOrder:
    a:  u8,
    b: (u8, u8),
    c: u64,
} // (1 + 2 + 8) % 8 == 11 % 8 == 3
*/

lense_struct!{GoodOrder: // Fails strict_ordering
    c: u64,
    a:  u8,
    b: (u8, u8),
    padding: [u8; 5],
} // (8 + 2 + 1 + 5) % 8 == 16 % 8 == 0

lense_struct!{GoodStrictOrder: // --features strict_ordering
    a: u8,
    b: (u8, u8),
    padding: [u8; 5],
    c: u64,
} // (1 + 2 + 5 + 8) % 8 == 16 % 8 == 0
