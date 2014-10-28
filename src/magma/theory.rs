use std;

pub trait Magma
{
    fn op(&self, rhs:&Self) -> Self;
}

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(PartialEq)]
#[deriving(Show)]
pub struct M<A:Magma>(pub A);

impl<A> Magma for M<A>
    where
        A:Magma,
{
    #[inline]
    fn op(&self, rhs:&M<A>) -> M<A> {
        let &M(ref lhs) = self;
        let &M(ref rhs) = rhs;
        M(lhs.op(rhs))
    }
}

impl<A> std::ops::Mul<M<A>,M<A>> for M<A>
    where
        A:Magma,
{
    #[inline]
    fn mul(&self, rhs:&M<A>) -> M<A> {
        self.op(rhs)
    }
}
