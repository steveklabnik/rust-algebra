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
    Or,
    S,
    SemigroupIterator,
    SemigroupPowNonZero,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ITERATIONS: uint = 10000u;

#[quickcheck]
fn associative(a:bool, b:bool, c:bool) -> bool {
    S(Or(a)) * (S(Or(b)) * S(Or(c))) == (S(Or(a)) * S(Or(b))) * S(Or(c))
}

#[quickcheck]
fn pownz_correct(a:bool) -> bool {
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    Or(a).pownz(n) == util::pownz_naive(Or(a), n)
}

#[quickcheck]
fn product_correct(a:bool, n:uint) -> bool {
    let mut it = iter::Repeat::new(Or(a)).take(n);
    it.clone().product(Or(a)) == util::product_naive(&mut it, Or(a))
}
