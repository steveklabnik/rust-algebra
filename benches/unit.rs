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
    SemigroupIterator,
    SemigroupPowNonZero,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ELEM: () = ();
const ITERATIONS: uint = 10000u;

#[bench]
fn pownz_naive(b:&mut test::Bencher) {
    let r = util::seeded_rng();
    let g = &mut quickcheck::gen(r, quickcheck::DEFAULT_SIZE);
    let sx: () = Arbitrary::arbitrary(g);
    let task = || {
        util::pownz_naive(sx, ITERATIONS)
    };
    b.iter(task);
}

#[bench]
fn pownz(b:&mut test::Bencher) {
    let r = util::seeded_rng();
    let g = &mut quickcheck::gen(r, quickcheck::DEFAULT_SIZE);
    let sx: () = Arbitrary::arbitrary(g);
    let task = || {
        sx.pownz(ITERATIONS)
    };
    b.iter(task);
}

#[bench]
fn product_naive(b:&mut test::Bencher) {
    let r = util::seeded_rng();
    let g = &mut quickcheck::gen(r, ITERATIONS);
    let xs: Vec<()> = Arbitrary::arbitrary(g);
    let mut it = xs.iter().map(|&x| x);
    let task = || {
        util::product_naive(&mut it, ELEM)
    };
    b.iter(task);
}

#[bench]
fn product(b:&mut test::Bencher) {
    let r = util::seeded_rng();
    let g = &mut quickcheck::gen(r, ITERATIONS);
    let xs: Vec<()> = Arbitrary::arbitrary(g);
    let mut it = xs.iter().map(|&x| x);
    let task = || {
        it.product(ELEM)
    };
    b.iter(task);
}
