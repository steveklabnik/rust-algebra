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
    SemigroupPowNonZero,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ITERATIONS: uint = 10000u;

#[quickcheck]
fn associative(a:(uint,uint), b:(uint,uint), c:(uint,uint)) -> bool {
    let (al, ar) = a;
    let (bl, br) = b;
    let (cl, cr) = c;
    let a = (Add(al), Mul(ar));
    let b = (Add(bl), Mul(br));
    let c = (Add(cl), Mul(cr));
    S(a) * (S(b) * S(c)) == (S(a) * S(b)) * S(c)
}

#[quickcheck]
fn pownz_correct(a:(uint,uint)) -> bool {
    let (al, ar) = a;
    let a = (Add(al), Mul(ar));
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    a.pownz(n) == util::pownz_naive(a, n)
}

#[quickcheck]
fn product_correct(a:(uint,uint), n:uint) -> bool {
    let (al, ar) = a;
    let a = (Add(al), Mul(ar));
    let mut it = iter::Repeat::new(a).take(n);
    it.clone().product(a) == util::product_naive(&mut it, a)
}
