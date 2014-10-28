// local imports
use magma::theory::{
    Magma,
};
use structure::{
    Swap,
};

impl<A> Magma for Swap<A>
    where
        A:Magma,
{
    #[inline]
    fn op(&self, rhs:&Swap<A>) -> Swap<A> {
        let &Swap(ref rhs) = rhs;
        let &Swap(ref lhs) = self;
        Swap(rhs.op(lhs))
    }
}
