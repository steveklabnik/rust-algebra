// local imports
use core::{
    Semigroup,
};

impl<A,B> Semigroup for (A,B)
    where
        A:Semigroup,
        B:Semigroup,
{
    #[inline]
    fn op(&self, rhs:&(A,B)) -> (A,B) {
        let &(ref a1, ref b1) = self;
        let &(ref a2, ref b2) = rhs;
        (a1.op(a2), b1.op(b2))
    }
}
