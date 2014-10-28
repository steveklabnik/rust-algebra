// external imports
use std::cmp;

// local imports
use magma::theory::{
    Magma,
};
use structure::{
    Max,
};

impl<A> Magma for Max<A>
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
