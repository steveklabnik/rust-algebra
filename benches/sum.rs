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
use algebra::semigroup::{
    Semigroup,
    SemigroupIterator,
    SemigroupReplicate,
};
use algebra::structure::{
    Add,
    Mul,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ELEM: Result<Add<f64>,Mul<f64>> = Ok(Add(f64::consts::PI));
const ITERATIONS: uint = 10000u;

#[bench]
fn app(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a:  Result<    f64 ,    f64>  =  Arbitrary::arbitrary(gen);
    let a:  Result<Add<f64>,Mul<f64>> =  a
        .map    (|x| Add(x))
        .map_err(|y| Mul(y));
    let b:  Result<    f64 ,    f64>  =  Arbitrary::arbitrary(gen);
    let b: &Result<Add<f64>,Mul<f64>> = &b
        .map    (|x| Add(x))
        .map_err(|y| Mul(y));
    let task = || {
        a.app(b)
    };
    bencher.iter(task);
}

#[bench]
fn rep_one_naive(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a: Result<    f64 ,    f64>  = Arbitrary::arbitrary(gen);
    let a: Result<Add<f64>,Mul<f64>> = a
        .map    (|x| Add(x))
        .map_err(|y| Mul(y));
    let task = || {
        util::rep_one_naive(a, ITERATIONS)
    };
    bencher.iter(task);
}

#[bench]
fn rep_one(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a: Result<    f64 ,    f64>  = Arbitrary::arbitrary(gen);
    let a: Result<Add<f64>,Mul<f64>> = a
        .map    (|x| Add(x))
        .map_err(|y| Mul(y));
    let task = || {
        a.rep_one(ITERATIONS)
    };
    bencher.iter(task);
}

#[bench]
fn cat_one_naive(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, ITERATIONS);
    let xs: Vec<Result<f64,f64>> = Arbitrary::arbitrary(gen);
    let mut it = xs.into_iter()
        .map(|s| s
            .map    (|x| Add(x))
            .map_err(|y| Mul(y)));
    let task = || {
        util::cat_one_naive(&mut it, ELEM)
    };
    bencher.iter(task);
}

#[bench]
fn cat_one(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, ITERATIONS);
    let xs: Vec<Result<f64,f64>> = Arbitrary::arbitrary(gen);
    let mut it = xs.into_iter()
        .map(|s| s
            .map    (|x| Add(x))
            .map_err(|y| Mul(y)));
    let task = || {
        it.cat_one(ELEM)
    };
    bencher.iter(task);
}
