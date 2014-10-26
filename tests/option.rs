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
    SemigroupIterator,
    SemigroupPowNonZero,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ITERATIONS: uint = 10000u;

#[quickcheck]
fn associative(a:Option<uint>, b:Option<uint>, c:Option<uint>) -> bool {
    let a = a.map(|x| Add(x));
    let b = b.map(|x| Add(x));
    let c = c.map(|x| Add(x));
    S(a) * (S(b) * S(c)) == (S(a) * S(b)) * S(c)
}

#[quickcheck]
fn pownz_correct(a:Option<uint>) -> bool {
    let a = a.map(|x| Add(x));
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    a.pownz(n) == util::pownz_naive(a, n)
}

#[quickcheck]
fn product_correct(a:Option<uint>, n:uint) -> bool {
    let a = a.map(|x| Add(x));
    let mut it = iter::Repeat::new(a).take(n);
    it.clone().product(a) == util::product_naive(&mut it, a)
}
