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
    Max,
    Semigroup,
    SemigroupIterator,
    SemigroupPowNonZero,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ELEM: Max<uint> = Max(0);
const ITERATIONS: uint = 10000u;

#[bench]
fn op(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a:  Max<uint> =  Max(Arbitrary::arbitrary(gen));
    let b: &Max<uint> = &Max(Arbitrary::arbitrary(gen));
    let task = || {
        a.op(b)
    };
    bencher.iter(task);
}

#[bench]
fn pownz_naive(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a: Max<uint> = Max(Arbitrary::arbitrary(gen));
    let task = || {
        util::pownz_naive(a, ITERATIONS)
    };
    bencher.iter(task);
}

#[bench]
fn pownz(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a: Max<uint> = Max(Arbitrary::arbitrary(gen));
    let task = || {
        a.pownz(ITERATIONS)
    };
    bencher.iter(task);
}

#[bench]
fn product_naive(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, ITERATIONS);
    let xs: Vec<uint> = Arbitrary::arbitrary(gen);
    let mut it = xs.iter().map(|&x| Max(x));
    let task = || {
        util::product_naive(&mut it, ELEM)
    };
    bencher.iter(task);
}

#[bench]
fn product(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, ITERATIONS);
    let xs: Vec<uint> = Arbitrary::arbitrary(gen);
    let mut it = xs.iter().map(|&x| Max(x));
    let task = || {
        it.product(ELEM)
    };
    bencher.iter(task);
}
