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

// local imports
use algebra::monoid::{
    Monoid,
};
use algebra::semigroup::{
    Semigroup,
    SemigroupIterator,
    SemigroupReplicate,
};
use algebra::structure::{
    Or,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ELEM: Or = Or(true);
const ITERATIONS: uint = 10000u;

#[bench]
fn app(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a:  Or =  Or(Arbitrary::arbitrary(gen));
    let b: &Or = &Or(Arbitrary::arbitrary(gen));
    let task = || {
        a.app(b)
    };
    bencher.iter(task);
}

#[bench]
fn rep_one_naive(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a: Or = Or(Arbitrary::arbitrary(gen));
    let task = || {
        util::rep_one_naive(a, ITERATIONS)
    };
    bencher.iter(task);
}

#[bench]
fn rep_one(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, quickcheck::DEFAULT_SIZE);
    let a: Or = Or(Arbitrary::arbitrary(gen));
    let task = || {
        a.rep_one(ITERATIONS)
    };
    bencher.iter(task);
}

#[bench]
fn cat_one_naive(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, ITERATIONS);
    let xs: Vec<bool> = Arbitrary::arbitrary(gen);
    let mut it = xs.iter().map(|&x| Or(x));
    let task = || {
        util::cat_one_naive(&mut it, ELEM)
    };
    bencher.iter(task);
}

#[bench]
fn cat_one(bencher:&mut test::Bencher) {
    let rng = util::seeded_rng();
    let gen = &mut quickcheck::gen(rng, ITERATIONS);
    let xs: Vec<bool> = Arbitrary::arbitrary(gen);
    let mut it = xs.iter().map(|&x| Or(x));
    let task = || {
        it.cat_one(ELEM)
    };
    bencher.iter(task);
}

#[bench]
fn nil(bencher:&mut test::Bencher) {
    let task = || {
        let _: Or = Monoid::nil();
    };
    bencher.iter(task);
}
