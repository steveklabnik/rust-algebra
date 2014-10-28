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
    SemigroupReplicate,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ITERATIONS: uint = 10000u;

#[quickcheck]
fn app_associative(a:bool, b:bool, c:bool) -> bool {
    S(Or(a)) * (S(Or(b)) * S(Or(c))) == (S(Or(a)) * S(Or(b))) * S(Or(c))
}

#[quickcheck]
fn app_sound(a:bool, b:bool) -> bool {
    S(Or(a)) * S(Or(b)) == S(Or(a || b))
}

#[quickcheck]
fn rep_one_equiv_naive(a:bool) -> bool {
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    Or(a).rep_one(n) == util::rep_one_naive(Or(a), n)
}

#[quickcheck]
fn cat_one_equiv_naive(a:bool, n:uint) -> bool {
    let mut it = iter::Repeat::new(Or(a)).take(n);
    it.clone().cat_one(Or(a)) == util::cat_one_naive(&mut it, Or(a))
}
