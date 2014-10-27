#![crate_name="semigroup"]
#![crate_type="rlib"]

#![feature(phase)]

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

pub use self::and::{
    And,
};

pub use self::mul::{
    Mul,
};

pub use self::or::{
    Or,
};

pub use self::swap::{
    Swap,
};

pub mod core;
pub mod add;
pub mod and;
pub mod dlist;
pub mod mul;
pub mod option;
pub mod or;
pub mod product;
pub mod ringbuf;
pub mod sum;
pub mod swap;
pub mod unit;
pub mod vec;
