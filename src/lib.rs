#![crate_name="algebra"]
#![crate_type="lib"]

#![license = "MIT"]
#![doc(html_root_url = "http://darinmorrison.github.io/rust-algebra/doc/algebra/")]

#![feature(phase)]

//! This crate defines traits for working with
//! [structures](http://en.wikipedia.org/wiki/Algebraic_structure)
//! from [abstract algebra](http://en.wikipedia.org/wiki/Abstract_algebra),
//! such as [semigroups](http://en.wikipedia.org/wiki/Semigroup),
//! [monoids](http://en.wikipedia.org/wiki/Monoid), etc.

// external crates
extern crate collections;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[phase(plugin)]
extern crate quickcheck_macros;

pub mod magma;
pub mod monoid;
pub mod semigroup;
pub mod structure;
