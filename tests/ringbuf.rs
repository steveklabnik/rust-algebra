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
use std::collections::ringbuf::{
    RingBuf,
};
use std::iter;
use std::rand;

// local imports
use semigroup::{
    S,
    SemigroupIterator,
    SemigroupReplicate,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ITERATIONS: uint = 100u;

#[quickcheck]
fn app_associative(a:Vec<uint>, b:Vec<uint>, c:Vec<uint>) -> bool {
    let a: RingBuf<uint> = a.into_iter().collect();
    let b: RingBuf<uint> = b.into_iter().collect();
    let c: RingBuf<uint> = c.into_iter().collect();
    S(a.clone()) * (S(b.clone()) * S(c.clone())) == (S(a) * S(b)) * S(c)
}

#[quickcheck]
fn app_sound(a:Vec<uint>, b:Vec<uint>) -> bool {
    let a: RingBuf<uint> = a.into_iter().collect();
    let b: RingBuf<uint> = b.into_iter().collect();
    let mut c = a.clone();
    c.extend(b.iter().map(|&x| x));
    S(a) * S(b) == S(c)
}

#[quickcheck]
fn rep_one_equiv_naive(a:Vec<uint>) -> bool {
    let a: RingBuf<uint> = a.into_iter().collect();
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    a.clone().rep_one(n) == util::rep_one_naive(a, n)
}

#[quickcheck]
fn cat_one_equiv_naive(a:Vec<uint>, n:uint) -> bool {
    let a: RingBuf<uint> = a.into_iter().collect();
    let mut it = iter::Repeat::new(a.clone()).take(n);
    it.clone().cat_one(a.clone()) == util::cat_one_naive(&mut it, a)
}
