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
    Semigroup,
    SemigroupIterator,
    SemigroupPowNonZero,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ELEM: Option<Add<f64>> = Some(Add(f64::consts::PI));
const ITERATIONS: uint = 10000u;

#[bench]
fn op(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a:  Option    <f64>  =  Arbitrary::arbitrary(gen);
    let a:  Option<Add<f64>> =  a.map(|x| Add(x));
    let b:  Option    <f64>  =  Arbitrary::arbitrary(gen);
    let b: &Option<Add<f64>> = &b.map(|x| Add(x));
    let task = || {
        a.op(b)
    };
    bencher.iter(task);
}

#[bench]
fn pownz_naive(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a: Option    <f64>  = Arbitrary::arbitrary(gen);
    let a: Option<Add<f64>> = a.map(|x| Add(x));
    let task = || {
        util::pownz_naive(a, ITERATIONS)
    };
    bencher.iter(task);
}

#[bench]
fn pownz(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a: Option    <f64>  = Arbitrary::arbitrary(gen);
    let a: Option<Add<f64>> = a.map(|x| Add(x));
    let task = || {
        a.pownz(ITERATIONS)
    };
    bencher.iter(task);
}

#[bench]
fn product_naive(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, ITERATIONS);
    let xs: Vec<Option<f64>> = Arbitrary::arbitrary(gen);
    let mut it = xs.iter().map(|&x| x.map(|x| Add(x)));
    let task = || {
        util::product_naive(&mut it, ELEM)
    };
    bencher.iter(task);
}

#[bench]
fn product(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, ITERATIONS);
    let xs: Vec<Option<f64>> = Arbitrary::arbitrary(gen);
    let mut it = xs.iter().map(|&x| x.map(|x| Add(x)));
    let task = || {
        it.product(ELEM)
    };
    bencher.iter(task);
}
