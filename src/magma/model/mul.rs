// local imports
use magma::theory::{
    Magma,
};
use structure::{
    Mul,
};

impl<A> Magma for Mul<A>
    where
        A:Num,
{
    #[inline]
    fn op(&self, rhs:&Mul<A>) -> Mul<A> {
        let &Mul(ref lhs) = self;
        let &Mul(ref rhs) = rhs;
        Mul(lhs.mul(rhs))
    }
}
