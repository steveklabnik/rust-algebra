#![allow(unused_attribute)]
#![feature(phase)]

// external crates
extern crate quickcheck;
#[phase(plugin)]
extern crate quickcheck_macros;

// local crates
extern crate algebra;

// external exports
use quickcheck::{
    Arbitrary,
    gen,
};
use std::iter;
use std::rand;

// local imports
use algebra::magma::{
    M,
};
use algebra::semigroup::{
    SemigroupIterator,
    SemigroupReplicate,
};
use algebra::structure::{
    Fst,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ITERATIONS: uint = 10000u;

#[quickcheck]
fn mag_app_asc(a:uint, b:uint, c:uint) -> bool {
    let a = Fst(a);
    let b = Fst(b);
    let c = Fst(c);
    M(a) * (M(b) * M(c)) == (M(a) * M(b)) * M(c)
}

#[quickcheck]
fn mag_app_snd(a:uint, b:uint) -> bool {
    M(Fst(a)) * M(Fst(b)) == M(Fst(a))
}

#[quickcheck]
fn sem_cat_one_eqv_nai(a:uint, n:uint) -> bool {
    let a = Fst(a);
    let mut it = iter::Repeat::new(a).take(n);
    it.clone().cat_one(a) == util::cat_one_naive(&mut it, a)
}

#[quickcheck]
fn sem_rep_one_eqv_nai(a:uint) -> bool {
    let a = Fst(a);
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    a.rep_one(n) == util::rep_one_naive(a, n)
}
