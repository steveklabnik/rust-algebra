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
    SemigroupReplicate,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ELEM: Option<Add<f64>> = Some(Add(f64::consts::PI));
const ITERATIONS: uint = 10000u;

#[bench]
fn app(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a:  Option    <f64>  =  Arbitrary::arbitrary(gen);
    let a:  Option<Add<f64>> =  a.map(|x| Add(x));
    let b:  Option    <f64>  =  Arbitrary::arbitrary(gen);
    let b: &Option<Add<f64>> = &b.map(|x| Add(x));
    let task = || {
        a.app(b)
    };
    bencher.iter(task);
}

#[bench]
fn rep_one_naive(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a: Option    <f64>  = Arbitrary::arbitrary(gen);
    let a: Option<Add<f64>> = a.map(|x| Add(x));
    let task = || {
        util::rep_one_naive(a, ITERATIONS)
    };
    bencher.iter(task);
}

#[bench]
fn rep_one(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a: Option    <f64>  = Arbitrary::arbitrary(gen);
    let a: Option<Add<f64>> = a.map(|x| Add(x));
    let task = || {
        a.rep_one(ITERATIONS)
    };
    bencher.iter(task);
}

#[bench]
fn cat_one_naive(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, ITERATIONS);
    let xs: Vec<Option<f64>> = Arbitrary::arbitrary(gen);
    let mut it = xs.iter().map(|&x| x.map(|x| Add(x)));
    let task = || {
        util::cat_one_naive(&mut it, ELEM)
    };
    bencher.iter(task);
}

#[bench]
fn cat_one(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, ITERATIONS);
    let xs: Vec<Option<f64>> = Arbitrary::arbitrary(gen);
    let mut it = xs.iter().map(|&x| x.map(|x| Add(x)));
    let task = || {
        it.cat_one(ELEM)
    };
    bencher.iter(task);
}
