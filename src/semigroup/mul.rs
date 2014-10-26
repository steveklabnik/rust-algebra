// local imports
use core::{
    Semigroup,
};

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(PartialEq)]
#[deriving(Show)]
pub struct Mul<A:Num>(pub A);

impl<A> Semigroup for Mul<A>
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
