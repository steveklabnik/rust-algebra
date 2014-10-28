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
    SemigroupReplicate,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ITERATIONS: uint = 10000u;

#[quickcheck]
fn app_associative(a:uint, b:uint, c:uint) -> bool {
    S(Snd(a)) * (S(Snd(b)) * S(Snd(c))) == (S(Snd(a)) * S(Snd(b))) * S(Snd(c))
}

#[quickcheck]
fn app_sound(a:uint, b:uint) -> bool {
    S(Snd(a)) * S(Snd(b)) == S(Snd(b))
}

#[quickcheck]
fn rep_one_equiv_naive(a:uint) -> bool {
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    Snd(a).rep_one(n) == util::rep_one_naive(Snd(a), n)
}

#[quickcheck]
fn cat_one_equiv_naive(a:uint, n:uint) -> bool {
    let mut it = iter::Repeat::new(Snd(a)).take(n);
    it.clone().cat_one(Snd(a)) == util::cat_one_naive(&mut it, Snd(a))
}
