#![feature(phase)]
#![feature(tuple_indexing)]

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
    Semigroup,
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
fn mag_app_asc(a:(uint,uint), b:(uint,uint), c:(uint,uint)) -> bool {
    let a = (Add(a.0), Mul(a.1));
    let b = (Add(b.0), Mul(b.1));
    let c = (Add(c.0), Mul(c.1));
    M(a) * (M(b) * M(c)) == (M(a) * M(b)) * M(c)
}

#[quickcheck]
fn mag_app_snd(a:(uint,uint), b:(uint,uint)) -> bool {
    let a = (Add(a.0), Mul(a.1));
    let b = (Add(b.0), Mul(b.1));
    M(a) * M(b) == M((a.0.app(&b.0), a.1.app(&b.1)))
}

#[quickcheck]
fn mon_nil_app_idn(b:(uint,uint)) -> bool {
    let b = (Add(b.0), Mul(b.1));
    M(Monoid::nil()) * M(b) == M(b)
}

#[quickcheck]
fn mon_app_nil_idn(a:(uint,uint)) -> bool {
    let a = (Add(a.0), Mul(a.1));
    M(a) * M(Monoid::nil()) == M(a)
}

#[quickcheck]
fn mon_cat_eqv_nai(a:(uint,uint), n:uint) -> bool {
    let a = (Add(a.0), Mul(a.1));
    let mut it = iter::Repeat::new(a).take(n);
    it.clone().cat() == util::cat_naive(&mut it)
}

#[quickcheck]
fn mon_rep_eqv_nai(a:(uint,uint)) -> bool {
    let a = (Add(a.0), Mul(a.1));
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    a.rep(n) == util::rep_naive(a, n)
}

#[quickcheck]
fn mon_sem_cat_cmp(a:(uint,uint), n:uint) -> bool {
    let a = (Add(a.0), Mul(a.1));
    let it = iter::Repeat::new(a).take(n + 1u);
    it.clone().cat() == it.skip(1u).cat_one(a)
}

#[quickcheck]
fn mon_sem_rep_cmp(a:(uint,uint)) -> bool {
    let a = (Add(a.0), Mul(a.1));
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n: uint = Arbitrary::arbitrary(g);
    a.rep(n + 1u) == util::rep_one_naive(a, n)
}

#[quickcheck]
fn sem_cat_one_eqv_nai(a:(uint,uint), n:uint) -> bool {
    let a = (Add(a.0), Mul(a.1));
    let mut it = iter::Repeat::new(a).take(n);
    it.clone().cat_one(a) == util::cat_one_naive(&mut it, a)
}

#[quickcheck]
fn sem_rep_one_eqv_nai(a:(uint,uint)) -> bool {
    let a = (Add(a.0), Mul(a.1));
    let g = &mut gen(rand::task_rng(), ITERATIONS);
    let n = Arbitrary::arbitrary(g);
    a.rep_one(n) == util::rep_one_naive(a, n)
}
