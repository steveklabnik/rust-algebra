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
    SemigroupReplicate,
    SemigroupIterator,
};

pub use self::add::{
    Add,
};

pub use self::and::{
    And,
};

pub use self::fst::{
    Fst,
};

pub use self::max::{
    Max,
};

pub use self::min::{
    Min,
};

pub use self::mul::{
    Mul,
};

pub use self::or::{
    Or,
};

pub use self::snd::{
    Snd,
};

pub use self::swap::{
    Swap,
};

pub mod core;
pub mod add;
pub mod and;
pub mod dlist;
pub mod fst;
pub mod max;
pub mod min;
pub mod mul;
pub mod option;
pub mod or;
pub mod product;
pub mod ringbuf;
pub mod snd;
pub mod string;
pub mod sum;
pub mod swap;
pub mod unit;
pub mod vec;
