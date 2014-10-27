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
    Snd,
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
    S(Snd(a)) * (S(Snd(b)) * S(Snd(c))) == (S(Snd(a)) * S(Snd(b))) * S(Snd(c))
}

#[quickcheck]
fn pownz_correct(a:uint) -> bool {
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    Snd(a).pownz(n) == util::pownz_naive(Snd(a), n)
}

#[quickcheck]
fn product_correct(a:uint, n:uint) -> bool {
    let mut it = iter::Repeat::new(Snd(a)).take(n);
    it.clone().product(Snd(a)) == util::product_naive(&mut it, Snd(a))
}
