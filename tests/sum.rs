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
    Mul,
    S,
    SemigroupIterator,
    SemigroupReplicate,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ITERATIONS: uint = 10000u;

#[quickcheck]
fn app_associative(a:Result<uint,uint>, b:Result<uint,uint>, c:Result<uint,uint>) -> bool {
    let a = a.map(|l| Add(l)).map_err(|r| Mul(r));
    let b = b.map(|l| Add(l)).map_err(|r| Mul(r));
    let c = c.map(|l| Add(l)).map_err(|r| Mul(r));
    S(a) * (S(b) * S(c)) == (S(a) * S(b)) * S(c)
}

#[quickcheck]
fn app_sound(a:Result<uint,uint>, b:Result<uint,uint>) -> bool {
    let a = a.map(|l| Add(l)).map_err(|r| Mul(r));
    let b = b.map(|l| Add(l)).map_err(|r| Mul(r));
    S(a) * S(b) == S(a.or(b))
}

#[quickcheck]
fn rep_one_equiv_naive(a:Result<uint,uint>) -> bool {
    let a = a.map(|l| Add(l)).map_err(|r| Mul(r));
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    a.rep_one(n) == util::rep_one_naive(a, n)
}

#[quickcheck]
fn cat_one_equiv_naive(a:Result<uint,uint>, n:uint) -> bool {
    let a = a.map(|l| Add(l)).map_err(|r| Mul(r));
    let mut it = iter::Repeat::new(a).take(n);
    it.clone().cat_one(a) == util::cat_one_naive(&mut it, a)
}
