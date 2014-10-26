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
use std::rand;

// local imports
use semigroup::{
    Mul,
    SemigroupIterator,
    SemigroupPowNonZero,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ELEM: Mul<f64> = Mul(f64::consts::PI);
const ITERATIONS: uint = 10000u;

#[bench]
fn pownz_builtin(b:&mut test::Bencher) {
    let r = rand::task_rng();
    let g = &mut quickcheck::gen(r, quickcheck::DEFAULT_SIZE);
    let a: f64 = Arbitrary::arbitrary(g);
    let task = || {
        Mul(util::pownz_builtin(a, ITERATIONS))
    };
    b.iter(task);
}

#[bench]
fn pownz_naive(b:&mut test::Bencher) {
    let r = rand::task_rng();
    let g = &mut quickcheck::gen(r, quickcheck::DEFAULT_SIZE);
    let a: Mul<f64> = Mul(Arbitrary::arbitrary(g));
    let task = || {
        util::pownz_naive(a, ITERATIONS)
    };
    b.iter(task);
}

#[bench]
fn pownz(b:&mut test::Bencher) {
    let r = rand::task_rng();
    let g = &mut quickcheck::gen(r, quickcheck::DEFAULT_SIZE);
    let a: Mul<f64> = Mul(Arbitrary::arbitrary(g));
    let task = || {
        a.pownz(ITERATIONS)
    };
    b.iter(task);
}

#[bench]
fn product_naive(b:&mut test::Bencher) {
    let r = rand::task_rng();
    let g = &mut quickcheck::gen(r, ITERATIONS);
    let xs: Vec<f64> = Arbitrary::arbitrary(g);
    let mut it = xs.iter().map(|&x| Mul(x));
    let task = || {
        util::product_naive(&mut it, ELEM)
    };
    b.iter(task);
}

#[bench]
fn product(b:&mut test::Bencher) {
    let r = rand::task_rng();
    let g = &mut quickcheck::gen(r, ITERATIONS);
    let xs: Vec<f64> = Arbitrary::arbitrary(g);
    let mut it = xs.iter().map(|&x| Mul(x));
    let task = || {
        it.product(ELEM)
    };
    b.iter(task);
}
