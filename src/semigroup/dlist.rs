// external imports
use std::collections::dlist::{
    DList,
};

// local imports
use core::{
    Semigroup,
};

impl<A> Semigroup for DList<A>
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
