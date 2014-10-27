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
use std::cmp;
use std::iter;
use std::rand;

// local imports
use semigroup::{
    Min,
    S,
    SemigroupIterator,
    SemigroupPowNonZero,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ITERATIONS: uint = 10000u;

#[quickcheck]
fn op_associative(a:uint, b:uint, c:uint) -> bool {
    S(Min(a)) * (S(Min(b)) * S(Min(c))) == (S(Min(a)) * S(Min(b))) * S(Min(c))
}

#[quickcheck]
fn op_sound(a:uint, b:uint) -> bool {
    S(Min(a)) * S(Min(b)) == S(Min(cmp::min(a,b)))
}

#[quickcheck]
fn pownz_equiv_naive(a:uint) -> bool {
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    Min(a).pownz(n) == util::pownz_naive(Min(a), n)
}

#[quickcheck]
fn product_equiv_naive(a:uint, n:uint) -> bool {
    let mut it = iter::Repeat::new(Min(a)).take(n);
    it.clone().product(Min(a)) == util::product_naive(&mut it, Min(a))
}
