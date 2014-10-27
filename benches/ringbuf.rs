#![feature(phase)]

// external crates
extern crate quickcheck;
#[phase(plugin)]
extern crate quickcheck_macros;
extern crate test;

// local crates
extern crate semigroup;

// external exports
use quickcheck::{
    Arbitrary,
};
use std::collections::ringbuf::{
    RingBuf,
};
use std::f64;
use std::iter;

// local imports
use semigroup::{
    SemigroupIterator,
    SemigroupPowNonZero,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ITERATIONS: uint = 100u;

#[allow(non_snake_case)]
#[inline]
fn ELEM() -> RingBuf<f64> {
    iter::Repeat::new(f64::consts::PI).take(10).collect()
}

#[bench]
fn pownz_naive(b:&mut test::Bencher) {
    let r = util::seeded_rng();
    let g = &mut quickcheck::gen(r, quickcheck::DEFAULT_SIZE);
    let sx: Vec    <f64> = Arbitrary::arbitrary(g);
    let sx: RingBuf<f64> = sx.into_iter().collect();
    let task = || {
        util::pownz_naive(sx.clone(), ITERATIONS)
    };
    b.iter(task);
}

#[bench]
fn pownz(b:&mut test::Bencher) {
    let r = util::seeded_rng();
    let g = &mut quickcheck::gen(r, quickcheck::DEFAULT_SIZE);
    let sx: Vec    <f64> = Arbitrary::arbitrary(g);
    let sx: RingBuf<f64> = sx.into_iter().collect();
    let task = || {
        sx.clone().pownz(ITERATIONS)
    };
    b.iter(task);
}

#[bench]
fn product_naive(b:&mut test::Bencher) {
    let r = util::seeded_rng();
    let g = &mut quickcheck::gen(r, ITERATIONS);
    let xs: Vec    <Vec    <f64>> = Arbitrary::arbitrary(g);
    let xs: RingBuf<RingBuf<f64>> = xs
        .into_iter()
        .map(|x| x.into_iter().collect())
        .collect();
    let mut it = xs.iter().map(|ref x| (*x).clone());
    let task = || {
        util::product_naive(&mut it, ELEM())
    };
    b.iter(task);
}

#[bench]
fn product(b:&mut test::Bencher) {
    let r = util::seeded_rng();
    let g = &mut quickcheck::gen(r, ITERATIONS);
    let xs: Vec    <Vec    <f64>> = Arbitrary::arbitrary(g);
    let xs: RingBuf<RingBuf<f64>> = xs
        .into_iter()
        .map(|x| x.into_iter().collect())
        .collect();
    let mut it = xs.iter().map(|ref x| (*x).clone());
    let task = || {
        it.product(ELEM())
    };
    b.iter(task);
}
