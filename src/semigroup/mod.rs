#![crate_name="semigroup"]
#![crate_type="rlib"]

#![feature(overloaded_calls)]
#![feature(phase)]
#![feature(slicing_syntax)]

// external crates
extern crate collections;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[phase(plugin)]
extern crate quickcheck_macros;

pub use self::core::{
    S,
    Semigroup,
    SemigroupPowNonZero,
    SemigroupIterator,
};

pub use self::add::{
    Add,
};

pub use self::mul::{
    Mul,
};

pub mod core;
pub mod add;
pub mod dlist;
pub mod mul;
pub mod option;
pub mod product;
pub mod ringbuf;
pub mod sum;
pub mod unit;
pub mod vec;
