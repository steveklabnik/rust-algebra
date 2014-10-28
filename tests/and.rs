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
    And,
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
    S(And(a)) * (S(And(b)) * S(And(c))) == (S(And(a)) * S(And(b))) * S(And(c))
}

#[quickcheck]
fn app_sound(a:bool, b:bool) -> bool {
    S(And(a)) * S(And(b)) == S(And(a && b))
}

#[quickcheck]
fn rep_one_equiv_naive(a:bool) -> bool {
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    And(a).rep_one(n) == util::rep_one_naive(And(a), n)
}

#[quickcheck]
fn cat_one_equiv_naive(a:bool, n:uint) -> bool {
    let mut it = iter::Repeat::new(And(a)).take(n);
    it.clone().cat_one(And(a)) == util::cat_one_naive(&mut it, And(a))
}
