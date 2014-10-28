// external imports
use std;

// local imports
use monoid::theory::{
    Monoid,
};
use structure::{
    Add,
};

impl<A> Monoid for Add<A>
    where
        A:Num,
{
    #[inline]
    fn nil() -> Add<A> {
        Add(std::num::Zero::zero())
    }
}
