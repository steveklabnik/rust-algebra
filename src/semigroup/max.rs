// external imports
use std::cmp;

// local imports
use core::{
    Semigroup,
};

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(PartialEq)]
#[deriving(Show)]
pub struct Max<A:Ord>(pub A);

impl<A> Semigroup for Max<A>
    where
        A:Clone,
        A:Ord,
{
    #[inline]
    fn op(&self, rhs:&Max<A>) -> Max<A> {
        let Max(lhs) = (*self).clone();
        let Max(rhs) = (*rhs ).clone();
        Max(cmp::max(lhs, rhs))
    }
}
