#![feature(phase)]

// external crates
extern crate quickcheck;
#[phase(plugin)]
extern crate quickcheck_macros;

// local crates
extern crate semigroup;

// external exports
use quickcheck::{
    Arbitrary,
    gen,
};
use std::iter;
use std::rand;

// local imports
use semigroup::{
    S,
    SemigroupIterator,
    SemigroupPowNonZero,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ITERATIONS: uint = 10000u;

#[quickcheck]
fn associative(a:(), b:(), c:()) -> bool {
    S(a) * (S(b) * S(c)) == (S(a) * S(b)) * S(c)
}

#[quickcheck]
fn pownz_correct(a:()) -> bool {
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    a.pownz(n) == util::pownz_naive(a, n)
}

#[quickcheck]
fn product_correct(a:(), n:uint) -> bool {
    let mut it = iter::Repeat::new(a).take(n);
    it.clone().product(a) == util::product_naive(&mut it, a)
}
