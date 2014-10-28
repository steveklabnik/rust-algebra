// external imports
use std::cmp;

// local imports
use magma::theory::{
    Magma,
};
use structure::{
    Min,
};

impl<A> Magma for Min<A>
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
