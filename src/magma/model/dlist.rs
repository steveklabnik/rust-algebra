// external imports
use std::collections::dlist::{
    DList,
};

// local imports
use magma::theory::{
    Magma,
};

impl<A> Magma for DList<A>
    where
        A:Clone,
{
    #[inline]
    fn op(&self, rhs:&DList<A>) -> DList<A> {
        let mut res = self.clone();
        res.append(rhs.clone());
        res
    }
}
