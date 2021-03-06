#![feature(phase)]

// external crates
extern crate quickcheck;
#[phase(plugin)]
extern crate quickcheck_macros;
extern crate test;

// local crates
extern crate algebra;

// external exports
use quickcheck::{
    Arbitrary,
};
use std::f64;

// local imports
use algebra::monoid::{
    Monoid,
    MonoidIterator,
    MonoidReplicate,
};
use algebra::semigroup::{
    Semigroup,
    SemigroupIterator,
    SemigroupReplicate,
};
use algebra::structure::{
    Mul,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ELEM: Mul<f64> = Mul(f64::consts::PI);
const ITERATIONS: uint = 10000u;

#[bench]
fn app(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a:  Mul<f64> =  Mul(Arbitrary::arbitrary(gen));
    let b: &Mul<f64> = &Mul(Arbitrary::arbitrary(gen));
    let task = || {
        a.app(b)
    };
    bencher.iter(task);
}

#[bench]
fn rep_naive(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a: Mul<f64> = Mul(Arbitrary::arbitrary(gen));
    let task = || {
        util::rep_naive(a, ITERATIONS)
    };
    bencher.iter(task);
}

#[bench]
fn rep_one_naive(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a: Mul<f64> = Mul(Arbitrary::arbitrary(gen));
    let task = || {
        util::rep_one_naive(a, ITERATIONS)
    };
    bencher.iter(task);
}

#[bench]
fn rep(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a: Mul<f64> = Mul(Arbitrary::arbitrary(gen));
    let task = || {
        a.rep(ITERATIONS)
    };
    bencher.iter(task);
}

#[bench]
fn rep_one(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a: Mul<f64> = Mul(Arbitrary::arbitrary(gen));
    let task = || {
        a.rep_one(ITERATIONS)
    };
    bencher.iter(task);
}

#[bench]
fn cat_naive(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, ITERATIONS);
    let xs: Vec<f64> = Arbitrary::arbitrary(gen);
    let mut it = xs.iter().map(|&x| Mul(x));
    let task = || {
        util::cat_naive(&mut it)
    };
    bencher.iter(task);
}

#[bench]
fn cat_one_naive(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, ITERATIONS);
    let xs: Vec<f64> = Arbitrary::arbitrary(gen);
    let mut it = xs.iter().map(|&x| Mul(x));
    let task = || {
        util::cat_one_naive(&mut it, ELEM)
    };
    bencher.iter(task);
}

#[bench]
fn cat(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, ITERATIONS);
    let xs: Vec<f64> = Arbitrary::arbitrary(gen);
    let mut it = xs.iter().map(|&x| Mul(x));
    let task = || {
        it.cat()
    };
    bencher.iter(task);
}

#[bench]
fn cat_one(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, ITERATIONS);
    let xs: Vec<f64> = Arbitrary::arbitrary(gen);
    let mut it = xs.iter().map(|&x| Mul(x));
    let task = || {
        it.cat_one(ELEM)
    };
    bencher.iter(task);
}

#[bench]
fn nil(bencher:&mut test::Bencher) {
    let task = || {
        let _: Mul<f64> = Monoid::nil();
    };
    bencher.iter(task);
}
