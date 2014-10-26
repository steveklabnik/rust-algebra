// local crates
extern crate semigroup;

// external imports
use std::num;

// local imports
use semigroup::{
    Semigroup,
};

#[allow(dead_code)]
#[inline]
pub fn pownz_builtin<A>(base:A, exp:uint) -> A
    where
        A:num::One,
{
    num::pow(base, exp + 1)
}

#[inline]
pub fn pownz_naive<A>(base:A, mut exp:uint) -> A
    where
        A:Clone,
        A:Semigroup,
{
    let mut acc = base.clone();
    exp = exp + 1;
    while exp > 1 {
        acc = acc.op(&base);
        exp = exp - 1
    }
    acc
}

#[inline]
pub fn product_naive<A,F>(it:&mut F, mut acc:A) -> A
    where
        A:Semigroup,
        F:Iterator<A>,
{
    for x in it { acc = acc.op(&x) }
    acc
}
