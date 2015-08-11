#[macro_use] extern crate lense;
use lense::{Lense, ContinuousLenseReader};

// Public struct Alice
lense_struct!{pub Alice:
    a:  u8,
    b: (u8, u8),
    c: [u8; 4],
    d: u64,
}

//// Private struct Bob
//lense_struct!{Bob:
//    // Note the <'a> is inherited from struct Alice<'a> in which we don't see. This allows us to
//    // work on our own struct types directly
//    a: Alice<'a>,
//}

fn main() {
    // Buffer containing 3x Alice
    let mut alice = vec![0x00, // a[0].a
                         0x01, 0x02, // a[0].b
                         0x03, 0x04, 0x05, 0x06, // a[0].c
                         0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, // a[0].d
                         0x00, // ...
                         0x01, 0x02,
                         0x03, 0x04, 0x05, 0x06,
                         0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
                         0x00,
                         0x01, 0x02,
                         0x03, 0x04, 0x05, 0x06,
                         0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
                         ];

    // New vector of Alice::size() ready to be used.
    let mut alice_writer = vec![0u8; Alice::size()];
    { // Populate our new vector using a lense
        let (mut alice_writer_lense, rest) = Alice::new(&mut alice_writer);
        assert!(rest.len() == 0);
        *alice_writer_lense.a = 0;
        *alice_writer_lense.b.0 = 0x01;
        *alice_writer_lense.b.1 = 0x02;
        *alice_writer_lense.c[0] = 0x03;
        *alice_writer_lense.c[1] = 0x04;
        *alice_writer_lense.c[2] = 0x05;
        *alice_writer_lense.c[3] = 0x06;
        *alice_writer_lense.d = 1012478732780767239;
    }

    // Check that our manually populated Alice is identical to the first Alice in the vector 'a'
    assert!(&*alice_writer == &alice[0..Alice::size()]);

    { // Read each Alice from 'a'
        let mut remaining = &mut *alice;
        while let Ok(Some(mut a)) = Alice::from_buf(&mut remaining) {
            *a.a += 1;
            println!("a: {}; b: {:?}; c: {:?}; d: {}", *a.a, a.b, a.c, *a.d);
        }
        // If there is any excess, it is still accessible through the 'remaining' variable.
        // Alternatively this can be used as a starting point in a pool that owns some
        // preallocated-large buffer.
    }

    println!("Mutated result: {:?}", &*alice);
}
