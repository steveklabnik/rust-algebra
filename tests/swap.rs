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
    Add,
    S,
    SemigroupIterator,
    SemigroupPowNonZero,
    Swap,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ITERATIONS: uint = 10000u;

#[quickcheck]
fn associative(a:uint, b:uint, c:uint) -> bool {
    S(Swap(Add(a))) * (S(Swap(Add(b))) * S(Swap(Add(c)))) == (S(Swap(Add(a))) * S(Swap(Add(b)))) * S(Swap(Add(c)))
}

#[quickcheck]
fn pownz_correct(a:uint) -> bool {
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    Swap(Add(a)).pownz(n) == util::pownz_naive(Swap(Add(a)), n)
}

#[quickcheck]
fn product_correct(a:uint, n:uint) -> bool {
    let mut it = iter::Repeat::new(Swap(Add(a))).take(n);
    it.clone().product(Swap(Add(a))) == util::product_naive(&mut it, Swap(Add(a)))
}
