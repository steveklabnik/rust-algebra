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
    Semigroup,
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
fn op(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a:  Vec    <f64> =  Arbitrary::arbitrary(gen);
    let b:  Vec    <f64> =  Arbitrary::arbitrary(gen);
    let a:  RingBuf<f64> =  a.into_iter().collect();
    let b: &RingBuf<f64> = &b.into_iter().collect();
    let task = || {
        a.op(b)
    };
    bencher.iter(task);
}

#[bench]
fn pownz_naive(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a: Vec    <f64> = Arbitrary::arbitrary(gen);
    let a: RingBuf<f64> = a.into_iter().collect();
    let task = || {
        util::pownz_naive(a.clone(), ITERATIONS)
    };
    bencher.iter(task);
}

#[bench]
fn pownz(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a: Vec    <f64> = Arbitrary::arbitrary(gen);
    let a: RingBuf<f64> = a.into_iter().collect();
    let task = || {
        a.clone().pownz(ITERATIONS)
    };
    bencher.iter(task);
}

#[bench]
fn product_naive(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, ITERATIONS);
    let xs: Vec    <Vec    <f64>> = Arbitrary::arbitrary(gen);
    let xs: RingBuf<RingBuf<f64>> = xs
        .into_iter()
        .map(|x| x.into_iter().collect())
        .collect();
    let mut it = xs.iter().map(|ref x| (*x).clone());
    let task = || {
        util::product_naive(&mut it, ELEM())
    };
    bencher.iter(task);
}

#[bench]
fn product(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, ITERATIONS);
    let xs: Vec    <Vec    <f64>> = Arbitrary::arbitrary(gen);
    let xs: RingBuf<RingBuf<f64>> = xs
        .into_iter()
        .map(|x| x.into_iter().collect())
        .collect();
    let mut it = xs.iter().map(|ref x| (*x).clone());
    let task = || {
        it.product(ELEM())
    };
    bencher.iter(task);
}
