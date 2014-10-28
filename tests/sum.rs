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
    Add,
    Mul,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ITERATIONS: uint = 10000u;

#[quickcheck]
fn mag_app_asc(a:Result<uint,uint>, b:Result<uint,uint>, c:Result<uint,uint>) -> bool {
    let a = a.map(|l| Add(l)).map_err(|r| Mul(r));
    let b = b.map(|l| Add(l)).map_err(|r| Mul(r));
    let c = c.map(|l| Add(l)).map_err(|r| Mul(r));
    M(a) * (M(b) * M(c)) == (M(a) * M(b)) * M(c)
}

#[quickcheck]
fn mag_app_snd(a:Result<uint,uint>, b:Result<uint,uint>) -> bool {
    let a = a.map(|l| Add(l)).map_err(|r| Mul(r));
    let b = b.map(|l| Add(l)).map_err(|r| Mul(r));
    M(a) * M(b) == M(a.or(b))
}

#[quickcheck]
fn sem_cat_one_eqv_nai(a:Result<uint,uint>, n:uint) -> bool {
    let a = a.map(|l| Add(l)).map_err(|r| Mul(r));
    let mut it = iter::Repeat::new(a).take(n);
    it.clone().cat_one(a) == util::cat_one_naive(&mut it, a)
}

#[quickcheck]
fn sem_rep_one_eqv_nai(a:Result<uint,uint>) -> bool {
    let a = a.map(|l| Add(l)).map_err(|r| Mul(r));
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    a.rep_one(n) == util::rep_one_naive(a, n)
}
