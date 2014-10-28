#![feature(phase)]
#![feature(tuple_indexing)]

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
    Semigroup,
    SemigroupIterator,
    SemigroupReplicate,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ITERATIONS: uint = 10000u;

#[quickcheck]
fn app_associative(a:(uint,uint), b:(uint,uint), c:(uint,uint)) -> bool {
    let (al, ar) = a;
    let (bl, br) = b;
    let (cl, cr) = c;
    let a = (Add(al), Mul(ar));
    let b = (Add(bl), Mul(br));
    let c = (Add(cl), Mul(cr));
    S(a) * (S(b) * S(c)) == (S(a) * S(b)) * S(c)
}

#[quickcheck]
fn app_sound(a:(uint,uint), b:(uint,uint)) -> bool {
    let (al, ar) = a;
    let (bl, br) = b;
    let a = (Add(al), Mul(ar));
    let b = (Add(bl), Mul(br));
    S(a) * S(b) == S((a.0.app(&b.0), a.1.app(&b.1)))
}

#[quickcheck]
fn rep_one_equiv_naive(a:(uint,uint)) -> bool {
    let (al, ar) = a;
    let a = (Add(al), Mul(ar));
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    a.rep_one(n) == util::rep_one_naive(a, n)
}

#[quickcheck]
fn cat_one_equiv_naive(a:(uint,uint), n:uint) -> bool {
    let (al, ar) = a;
    let a = (Add(al), Mul(ar));
    let mut it = iter::Repeat::new(a).take(n);
    it.clone().cat_one(a) == util::cat_one_naive(&mut it, a)
}
