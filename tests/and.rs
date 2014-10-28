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
use algebra::monoid::{
    Monoid,
    MonoidIterator,
    MonoidReplicate,
};
use algebra::semigroup::{
    SemigroupIterator,
    SemigroupReplicate,
};
use algebra::structure::{
    And,
};

// custom mods
#[path="../src/util/mod.rs"]
mod util;

const ITERATIONS: uint = 10000u;

#[quickcheck]
fn mag_app_asc(a:bool, b:bool, c:bool) -> bool {
    let a = And(a);
    let b = And(b);
    let c = And(c);
    M(a) * (M(b) * M(c)) == (M(a) * M(b)) * M(c)
}

#[quickcheck]
fn mag_app_snd(a:bool, b:bool) -> bool {
    M(And(a)) * M(And(b)) == M(And(a && b))
}

#[quickcheck]
fn mon_nil_app_idn(b:bool) -> bool {
    let b = And(b);
    M(Monoid::nil()) * M(b) == M(b)
}

#[quickcheck]
fn mon_app_nil_idn(a:bool) -> bool {
    let a = And(a);
    M(a) * M(Monoid::nil()) == M(a)
}

#[quickcheck]
fn mon_cat_eqv_nai(a:bool, n:uint) -> bool {
    let a = And(a);
    let mut it = iter::Repeat::new(a).take(n);
    it.clone().cat() == util::cat_naive(&mut it)
}

#[quickcheck]
fn mon_rep_eqv_nai(a:bool) -> bool {
    let a = And(a);
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    a.rep(n) == util::rep_naive(a, n)
}

#[quickcheck]
fn mon_sem_cat_cmp(a:bool, n:uint) -> bool {
    let a = And(a);
    let it = iter::Repeat::new(a).take(n + 1u);
    it.clone().cat() == it.skip(1u).cat_one(a)
}

#[quickcheck]
fn mon_sem_rep_cmp(a:bool) -> bool {
    let a = And(a);
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n: uint = Arbitrary::arbitrary(g);
    a.rep(n + 1u) == util::rep_one_naive(a, n)
}

#[quickcheck]
fn sem_cat_one_eqv_nai(a:bool, n:uint) -> bool {
    let a = And(a);
    let mut it = iter::Repeat::new(a).take(n);
    it.clone().cat_one(a) == util::cat_one_naive(&mut it, a)
}

#[quickcheck]
fn sem_rep_one_eqv_nai(a:bool) -> bool {
    let a = And(a);
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    a.rep_one(n) == util::rep_one_naive(a, n)
}
