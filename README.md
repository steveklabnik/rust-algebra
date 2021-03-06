# rust-algebra

Abstract Algebra in Rust

[![build status](https://api.travis-ci.org/darinmorrison/rust-algebra.svg?branch=master)](https://travis-ci.org/darinmorrison/rust-algebra)

## Synopsis

This library implements `Monoid`, `Semigroup` and related traits. Semigroups are mathematical structures with an associative operation (e.g., `(+)`, `(*)`, `(&&)`, `(||)`, …).

The design of this library is roughly fashioned after Kmett's [semigroups](https://github.com/ekmett/semigroups/) for Haskell.

## Documentation

See the API documentation [here](http://darinmorrison.github.io/rust-algebra/doc/algebra/).

## Requirements

1.   [Rust](http://www.rust-lang.org/)
2.   [Cargo](http://crates.io/)

You can install both with the following:

```
$ curl -s https://static.rust-lang.org/rustup.sh | sudo sh
```

See [Installing Rust](http://doc.rust-lang.org/guide.html#installing-rust) for further details.

## Usage

```
$ cargo build       ## build library and binary
$ cargo test        ## run tests in ./tests
$ cargo bench       ## run benchmarks in ./benches
```
