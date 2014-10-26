// local imports
use core::{
    Semigroup,
};

impl<A> Semigroup for Vec<A>
    where
        A:Clone,
{
    #[inline]
    fn op(&self, rhs:&Vec<A>) -> Vec<A> {
        let mut res = self.clone();
        res.push_all(rhs.as_slice());
        res
    }
}
