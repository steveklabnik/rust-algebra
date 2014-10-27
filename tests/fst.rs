#![allow(unused_attribute)]
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
    Fst,
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
    S(Fst(a)) * (S(Fst(b)) * S(Fst(c))) == (S(Fst(a)) * S(Fst(b))) * S(Fst(c))
}

#[quickcheck]
fn pownz_correct(a:uint) -> bool {
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    Fst(a).pownz(n) == util::pownz_naive(Fst(a), n)
}

#[quickcheck]
fn product_correct(a:uint, n:uint) -> bool {
    let mut it = iter::Repeat::new(Fst(a)).take(n);
    it.clone().product(Fst(a)) == util::product_naive(&mut it, Fst(a))
}
