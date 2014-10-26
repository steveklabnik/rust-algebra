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
    Mul,
    S,
    SemigroupIterator,
    SemigroupPowNonZero,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ITERATIONS: uint = 10000u;

#[quickcheck]
fn associative(a:uint, b:uint, c:uint) -> bool {
    S(Mul(a)) * (S(Mul(b)) * S(Mul(c))) == (S(Mul(a)) * S(Mul(b))) * S(Mul(c))
}

#[quickcheck]
fn pownz_correct(a:uint) -> bool {
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    Mul(a).pownz(n) == util::pownz_naive(Mul(a), n)
}

#[quickcheck]
fn product_correct(a:uint, n:uint) -> bool {
    let mut it = iter::Repeat::new(Mul(a)).take(n);
    it.clone().product(Mul(a)) == util::product_naive(&mut it, Mul(a))
}
