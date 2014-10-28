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
    S,
    SemigroupIterator,
    SemigroupReplicate,
};
use util::ordering::{
    Proxy,
    reify,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ITERATIONS: uint = 10000u;

#[quickcheck]
fn app_associative(a:Proxy, b:Proxy, c:Proxy) -> bool {
    let a = reify(a);
    let b = reify(b);
    let c = reify(c);
    S(a) * (S(b) * S(c)) == (S(a) * S(b)) * S(c)
}

#[quickcheck]
fn app_sound(a:Proxy, b:Proxy) -> bool {
    let a = reify(a);
    let b = reify(b);
    S(a) * S(b) == match a {
        Less    => { S(Less)    },
        Equal   => { S(b)       },
        Greater => { S(Greater) },
    }
}

#[quickcheck]
fn rep_one_equiv_naive(a:Proxy) -> bool {
    let a = reify(a);
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    a.rep_one(n) == util::rep_one_naive(a, n)
}

#[quickcheck]
fn cat_one_equiv_naive(a:Proxy, n:uint) -> bool {
    let a = reify(a);
    let mut it = iter::Repeat::new(a).take(n);
    it.clone().cat_one(a) == util::cat_one_naive(&mut it, a)
}
