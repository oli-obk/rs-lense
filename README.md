Rust lense macro
================

Mutable by-ref reader that allows the consumer to peek into [u8] streams
assuming given types. The consumer experience is almost identical to that of
using any normal struct. Due to how little the Lenses actually do, performance
is expected to max your hardware's ability. Lenses don't get in the way and
feel just like using any normal struct!

## Usecases

- High performance stateless networking
- Streamed file format for storing big data (blockchain-like)

## Warnings

- Endianness isn't touched in the lense, you must handle this if you're doing
  networking or otherwise sharing accross platforms.

- Padding and alignment currently isn't automated and is a manual task. The
  order the fields are written in the `make_lense!()` macro is the order in
  which they are stored. Add `_padding_n` fields of the respective types.

- Versioning of protocols using lense should be handled by supporting multiple
  versions at once and determining the newest version on both endpoints. Lense
  does not handle any versioning (yet?). Semver is recommended.

Room for improvement
--------------------

However already a powerful mutable by-ref reader, there are limitations with
working with only macros. Notably, a syntax extension can be used to automate
and lint the alignment of all struct fields and types.

- [ ] Variable length types (**must be known at writer time!**)
- [ ] Syntax extension
  - [ ] Automate ordering (`C` style)
  - [ ] Lint manually ordered fields
  - [ ] `#[derive(..)]` attribute

Usage
-----

The following example is `examples/alice.rs` and can be ran with `cargo run --example alice`

```rust
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
```
Output altered for viewing
```
a:  0; b: (1, 2); c: [3, 4, 5, 6]; d: 1012478732780767239
a: 15; b: (1, 2); c: [3, 4, 5, 6]; d: 1012478732780767239
a: 30; b: (1, 2); c: [3, 4, 5, 6]; d: 1012478732780767239
Mutated result:
  [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
   15, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
   30, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
```

Large writers
-------------

The writer is just a helper function to create a new owned buffer of the
expected size.

**NOTE**: The performance benefit for using an uninitialised writer is very
tiny and exists only for benchmarking and big data usecases where the tiny
improvement can add up.

If you find it a good idea to have a very large writer you may employ the
`unsafe_writer` which gives you an uninitialized slice of the perfect size for
your data.  This is obviously discouraged, however, it is perfectly safe **IF
YOU USE IT RIGHT**.  The correct usage requires that you set all struct fields,
unlike the safe writer with default zeros if you miss something.

```rust
let mut a_ = unsafe { Alice::writer_uninit() };
{
    let mut a_l = a_.borrow_lense();
    *a_l.a = 0;
    *a_l.b = 513;
    *a_l.c[0] = 123;
    *a_l.c[1] = 321;
    *a_l.d = 1012478732780767239;
}
```

You may find it desirable to lift the unsafe around the whole struct definition
to visually show that the entire block is unsafe (if you mess things up). If
you do miss a field, expect the compiler to not scream at you - howeer, the
runtime **SHOULD** crash due to least the destructor expecting the values to
not be uninitialised.

Benchmarks
----------

### Linux livecd 3.18.9-hardened #1 SMP x86_64 Intel(R) Atom(TM) CPU N450 @ 1.66GHz GenuineIntel GNU/Linux
```
test alice_writer_init    ... bench:         179 ns/iter (+/- 4) = 83 MB/s
test alice_writer_uninit  ... bench:         178 ns/iter (+/- 4) = 84 MB/s
test alice_x3_reader      ... bench:           3 ns/iter (+/- 1) = 14999 MB/s
test u64x2_reader         ... bench:           1 ns/iter (+/- 0) = 16000 MB/s
test u64x2_writer_init    ... bench:         123 ns/iter (+/- 5) = 130 MB/s
test u64x2_writer_uninit  ... bench:         123 ns/iter (+/- 13) = 130 MB/s
test u64x31_reader        ... bench:          18 ns/iter (+/- 0) = 13777 MB/s
test u64x31_writer_init   ... bench:         185 ns/iter (+/- 12) = 1340 MB/s
test u64x31_writer_uninit ... bench:         141 ns/iter (+/- 18) = 1758 MB/s
```
### Macbook Pro 2014, i7? (Thanks frankmcsherry) (slightly stale results. Alice has since changed)
```
test alice_writer_init    ... bench:          29 ns/iter (+/- 14) = 517 MB/s
test alice_writer_uninit  ... bench:          29 ns/iter (+/- 9) = 517 MB/s
test alice_x3_reader      ... bench:           1 ns/iter (+/- 1) = 45000 MB/s
test u64x2_reader         ... bench:           0 ns/iter (+/- 0) = 16000 MB/s
test u64x2_writer_init    ... bench:          20 ns/iter (+/- 8) = 800 MB/s
test u64x2_writer_uninit  ... bench:          21 ns/iter (+/- 9) = 761 MB/s
test u64x31_reader        ... bench:           2 ns/iter (+/- 0) = 124000 MB/s
test u64x31_writer_init   ... bench:          29 ns/iter (+/- 4) = 8551 MB/s
test u64x31_writer_uninit ... bench:          26 ns/iter (+/- 6) = 9538 MB/s
```
