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
pub struct Min<A:Ord>(pub A);

impl<A> Semigroup for Min<A>
    where
        A:Clone,
        A:Ord,
{
    #[inline]
    fn op(&self, rhs:&Min<A>) -> Min<A> {
        let Min(lhs) = (*self).clone();
        let Min(rhs) = (*rhs ).clone();
        Min(cmp::min(lhs, rhs))
    }
}
