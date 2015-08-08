Rust lense macro
================

Borrow based by-ref reader allowing the consumer to peek into [u8] streams
assuming given types. The consumer experience is almost identical to that of
using any normal struct.

TODO
----

- [ ] Variable length types
- [ ] Syntax extension
  - [ ] Automate ordering (`C` style)
  - [ ] Lint manually ordered fields
  - [ ] `#[derive(..)]` attribute

Usage
-----

```rust
// Manual creation
make_lense!{Alice, AliceWriter,
    a: u8,
    b: u16,
    c: u32,
    d: u64,
}
//// Syntax extension
// #[derive(lense)]
// struct Alice {
//     a: u8,
//     b: u16,
//     c: u32,
//     d: u64,
// }
```
```rust
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
    *a_l.b = 513;
    *a_l.c = 100992003;
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
            println!("a: {}; b: {}; c: {}; d: {}", *a.a, *a.b, *a.c, *a.d);
        },
        Err(LenseError::NothingToParse) => break, // no more to process :)
        Err(LenseError::UnexpectedSize) => break, // Invalid chunk
        Err(LenseError::Incomplete) => break, // Incomplete
    };
};

println!("Mutated result: {:?}", &*a);
```
Output altered for viewing
```
a:  0; b: 513; c: 100992003; d: 1012478732780767239
a: 15; b: 513; c: 100992003; d: 1012478732780767239
a: 30; b: 513; c: 100992003; d: 1012478732780767239
Mutated result:
  [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
   15, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
   30, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]

```

Benchmarks
----------

```
Linux livecd 3.18.9-hardened #1 SMP x86_64 Intel(R) Atom(TM) CPU N450 @ 1.66GHz GenuineIntel GNU/Linux
```

```
test alice_x3_reader   ... bench:         120 ns/iter (+/- 2)
test u64x2_writer      ... bench:         116 ns/iter (+/- 1)
test u64x31_reader     ... bench:         184 ns/iter (+/- 9)
test u64x31_writer     ... bench:         199 ns/iter (+/- 2)
test vec_macro_u64x128 ... bench:         253 ns/iter (+/- 6)
```

**Note:** The last bench exists to demonstrate the cost of `vec![0u8; 8*128]`,
this method is used inside the writer to allocate the desired size.
