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
    Semigroup,
    SemigroupIterator,
    SemigroupReplicate,
    Swap,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ITERATIONS: uint = 10000u;

#[quickcheck]
fn app_associative(a:uint, b:uint, c:uint) -> bool {
    S(Swap(Add(a))) * (S(Swap(Add(b))) * S(Swap(Add(c)))) == (S(Swap(Add(a))) * S(Swap(Add(b)))) * S(Swap(Add(c)))
}

#[quickcheck]
fn app_sound(a:uint, b:uint) -> bool {
    S(Swap(Add(a))) * S(Swap(Add(b))) == S(Swap(Add(b).app(&Add(a))))
}

#[quickcheck]
fn rep_one_equiv_naive(a:uint) -> bool {
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    Swap(Add(a)).rep_one(n) == util::rep_one_naive(Swap(Add(a)), n)
}

#[quickcheck]
fn cat_one_equiv_naive(a:uint, n:uint) -> bool {
    let mut it = iter::Repeat::new(Swap(Add(a))).take(n);
    it.clone().cat_one(Swap(Add(a))) == util::cat_one_naive(&mut it, Swap(Add(a)))
}
