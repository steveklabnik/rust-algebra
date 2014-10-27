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
use std::f64;

// local imports
use semigroup::{
    Add,
    Mul,
    SemigroupIterator,
    SemigroupPowNonZero,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ELEM: Result<Add<f64>,Mul<f64>> = Ok(Add(f64::consts::PI));
const ITERATIONS: uint = 10000u;

#[bench]
fn pownz_naive(b:&mut test::Bencher) {
    let r = util::seeded_rng();
    let g = &mut quickcheck::gen(r, quickcheck::DEFAULT_SIZE);
    let a: Result<    f64 ,    f64>  = Arbitrary::arbitrary(g);
    let a: Result<Add<f64>,Mul<f64>> = a
        .map    (|x| Add(x))
        .map_err(|y| Mul(y));
    let task = || {
        util::pownz_naive(a, ITERATIONS)
    };
    b.iter(task);
}

#[bench]
fn pownz(b:&mut test::Bencher) {
    let r = util::seeded_rng();
    let g = &mut quickcheck::gen(r, quickcheck::DEFAULT_SIZE);
    let a: Result<    f64 ,    f64>  = Arbitrary::arbitrary(g);
    let a: Result<Add<f64>,Mul<f64>> = a
            .map    (|x| Add(x))
            .map_err(|y| Mul(y));
    let task = || {
        a.pownz(ITERATIONS)
    };
    b.iter(task);
}

#[bench]
fn product_naive(b:&mut test::Bencher) {
    let r = util::seeded_rng();
    let g = &mut quickcheck::gen(r, ITERATIONS);
    let xs: Vec<Result<f64,f64>> = Arbitrary::arbitrary(g);
    let mut it = xs.into_iter()
        .map(|s| s
            .map    (|x| Add(x))
            .map_err(|y| Mul(y)));
    let task = || {
        util::product_naive(&mut it, ELEM)
    };
    b.iter(task);
}

#[bench]
fn product(b:&mut test::Bencher) {
    let r = util::seeded_rng();
    let g = &mut quickcheck::gen(r, ITERATIONS);
    let xs: Vec<Result<f64,f64>> = Arbitrary::arbitrary(g);
    let mut it = xs.into_iter()
        .map(|s| s
            .map    (|x| Add(x))
            .map_err(|y| Mul(y)));
    let task = || {
        it.product(ELEM)
    };
    b.iter(task);
}
