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
use std::rand;

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
fn ELEM() -> String {
    String::from_str("")
}

#[bench]
fn pownz_naive(b:&mut test::Bencher) {
    let r = rand::task_rng();
    let g = &mut quickcheck::gen(r, quickcheck::DEFAULT_SIZE);
    let sx: String = Arbitrary::arbitrary(g);
    let task = || {
        util::pownz_naive(sx.clone(), ITERATIONS)
    };
    b.iter(task);
}

#[bench]
fn pownz(b:&mut test::Bencher) {
    let r = rand::task_rng();
    let g = &mut quickcheck::gen(r, quickcheck::DEFAULT_SIZE);
    let sx: String = Arbitrary::arbitrary(g);
    let task = || {
        sx.clone().pownz(ITERATIONS)
    };
    b.iter(task);
}

#[bench]
fn product_naive(b:&mut test::Bencher) {
    let r = rand::task_rng();
    let g = &mut quickcheck::gen(r, ITERATIONS);
    let xs: Vec<String> = Arbitrary::arbitrary(g);
    let mut it = xs.into_iter();
    let task = || {
        util::product_naive(&mut it, ELEM())
    };
    b.iter(task);
}

#[bench]
fn product(b:&mut test::Bencher) {
    let r = rand::task_rng();
    let g = &mut quickcheck::gen(r, ITERATIONS);
    let xs: Vec<String> = Arbitrary::arbitrary(g);
    let mut it = xs.into_iter();
    let task = || {
        it.product(ELEM())
    };
    b.iter(task);
}