// external imports
use std;

// local imports
use monoid::theory::{
    Monoid,
};
use structure::{
    Mul,
};

impl<A> Monoid for Mul<A>
    where
        A:Num,
{
    #[inline]
    fn nil() -> Mul<A> {
        Mul(std::num::One::one())
    }
}
