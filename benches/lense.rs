#![feature(test)]
extern crate test;
#[macro_use] extern crate lense;

use test::{Bencher, black_box};

use lense::*;

lense_struct!{Alice:
    a:  u8,
    b: (u8, u8),
    c: [u8; 4],
    d: u64,
}

lense_struct!{U64x32x32:
    a: [[u64; 32]; 32],
}

#[bench]
fn alice_x3_reader(b: &mut Bencher) {
    // Buffer containing 3x Alice
    let mut alice = black_box(
        vec![0x00, // a[0].a
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
             ]);

    b.bytes = (Alice::size() * 3) as u64;
    b.iter(|| {
        let mut remaining = &mut *alice;
        while let Ok(Some(a)) = Alice::from_buf(&mut remaining) {
            black_box(a);
        }
        black_box(remaining);
    })
}

#[bench]
fn alice_writer(b: &mut Bencher) {
    // New vector of Alice::size() ready to be used.
    let mut alice_writer = black_box(vec![0u8; Alice::size()]);

    b.bytes = Alice::size() as u64;
    b.iter(|| {
        let (mut alice_writer_lense, _) = Alice::new(&mut alice_writer);
        *alice_writer_lense.a = 0;
        *alice_writer_lense.b.0 = 0x01;
        *alice_writer_lense.b.1 = 0x02;
        *alice_writer_lense.c[0] = 0x03;
        *alice_writer_lense.c[1] = 0x04;
        *alice_writer_lense.c[2] = 0x05;
        *alice_writer_lense.c[3] = 0x06;
        *alice_writer_lense.d = 1012478732780767239;
        test::black_box(alice_writer_lense);
    })
}

#[bench]
fn u64x32x32_reader(b: &mut Bencher) {
    // Buffer containing 3x Alice
    let mut vec = black_box(vec![0u8; U64x32x32::size()]);

    b.bytes = U64x32x32::size() as u64;
    b.iter(|| {
        let mut remaining = &mut *vec;
        while let Ok(Some(s)) = U64x32x32::from_buf(&mut remaining) {
            black_box(s);
        }
        black_box(remaining);
    })
}

#[bench]
fn u64x32x32_writer(b: &mut Bencher) {
    // New vector of Alice::size() ready to be used.
    let mut u64x32x32_writer = black_box(vec![0u8; U64x32x32::size()]);

    b.bytes = U64x32x32::size() as u64;
    b.iter(|| {
        let (mut u64x32x32_writer_lense, _) = U64x32x32::new(&mut u64x32x32_writer);
        for i in (0..32) {
            for j in (0..32) {
                *u64x32x32_writer_lense.a[i][j] = 0;
            }
        }
        test::black_box(u64x32x32_writer_lense);
    })
}
