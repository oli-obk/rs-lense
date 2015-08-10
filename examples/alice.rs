#[macro_use] extern crate lense;
use lense::*;

make_lense!{PUB, Alice, AliceWriter,
    a:  u8,
    b: (u8, u8),
    c: [u8; 4],
    d: u64,
}

fn main() {
    let mut a = vec![0x00, // a[0].a
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

    let mut a_ = Alice::writer();
    { // construct a new Alice and populate the fields
        let mut a_l = a_.borrow_lense();
        *a_l.a = 0;
        *(*a_l.b).0 = 0x01;
        *(*a_l.b).1 = 0x02;
        *a_l.c[0] = 0x03;
        *a_l.c[1] = 0x04;
        *a_l.c[2] = 0x05;
        *a_l.c[3] = 0x06;
        *a_l.d = 1012478732780767239;
    }

    // Check that the manually populated Alice is equal to the buffer a
    assert!(a_.as_bytes() == &a[0..Alice::size()]);

    // Format: [(size, chunk); n] where n is number of chunks
    //           size only present for variable length fields
    let mut pos = 0;
    loop { // iterate over each Alice in our buffer a
        let (_, mut a) = a.split_at_mut(pos);
        match Alice::from_buf(&mut a) {
            Ok((mut a, size)) => {
                *a.a += pos as u8; pos += size;
                println!("a: {}; b: {:?}; c: {:?}; d: {}", *a.a, *a.b, *a.c, *a.d);
            },
            Err(LenseError::NothingToParse) => break, // no more to process :)
            Err(LenseError::UnexpectedSize) => break, // Invalid chunk
            Err(LenseError::Incomplete) => break, // Incomplete
        };
    };

    println!("Mutated result: {:?}", &*a);
}
