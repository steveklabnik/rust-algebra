// local imports
use magma::theory::{
    Magma,
};

impl<A,B> Magma for (A,B)
    where
        A:Magma,
        B:Magma,
{
    #[inline]
    fn op(&self, rhs:&(A,B)) -> (A,B) {
        let &(ref a1, ref b1) = self;
        let &(ref a2, ref b2) = rhs;
        (a1.op(a2), b1.op(b2))
    }
}
