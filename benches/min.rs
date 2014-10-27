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

// local imports
use semigroup::{
    Min,
    SemigroupIterator,
    SemigroupPowNonZero,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ELEM: Min<uint> = Min(0);
const ITERATIONS: uint = 10000u;

#[bench]
fn pownz_naive(b:&mut test::Bencher) {
    let r = util::seeded_rng();
    let g = &mut quickcheck::gen(r, quickcheck::DEFAULT_SIZE);
    let a: Min<uint> = Min(Arbitrary::arbitrary(g));
    let task = || {
        util::pownz_naive(a, ITERATIONS)
    };
    b.iter(task);
}

#[bench]
fn pownz(b:&mut test::Bencher) {
    let r = util::seeded_rng();
    let g = &mut quickcheck::gen(r, quickcheck::DEFAULT_SIZE);
    let a: Min<uint> = Min(Arbitrary::arbitrary(g));
    let task = || {
        a.pownz(ITERATIONS)
    };
    b.iter(task);
}

#[bench]
fn product_naive(b:&mut test::Bencher) {
    let r = util::seeded_rng();
    let g = &mut quickcheck::gen(r, ITERATIONS);
    let xs: Vec<uint> = Arbitrary::arbitrary(g);
    let mut it = xs.iter().map(|&x| Min(x));
    let task = || {
        util::product_naive(&mut it, ELEM)
    };
    b.iter(task);
}

#[bench]
fn product(b:&mut test::Bencher) {
    let r = util::seeded_rng();
    let g = &mut quickcheck::gen(r, ITERATIONS);
    let xs: Vec<uint> = Arbitrary::arbitrary(g);
    let mut it = xs.iter().map(|&x| Min(x));
    let task = || {
        it.product(ELEM)
    };
    b.iter(task);
}
