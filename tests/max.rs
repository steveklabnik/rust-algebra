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
    Max,
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
    S(Max(a)) * (S(Max(b)) * S(Max(c))) == (S(Max(a)) * S(Max(b))) * S(Max(c))
}

#[quickcheck]
fn op_sound(a:uint, b:uint) -> bool {
    S(Max(a)) * S(Max(b)) == S(Max(cmp::max(a,b)))
}

#[quickcheck]
fn pownz_equiv_naive(a:uint) -> bool {
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    Max(a).pownz(n) == util::pownz_naive(Max(a), n)
}

#[quickcheck]
fn product_equiv_naive(a:uint, n:uint) -> bool {
    let mut it = iter::Repeat::new(Max(a)).take(n);
    it.clone().product(Max(a)) == util::product_naive(&mut it, Max(a))
}
