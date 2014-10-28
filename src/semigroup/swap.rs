// local imports
use core::{
    Semigroup,
};

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(PartialEq)]
#[deriving(Show)]
pub struct Swap<A:Semigroup>(pub A);

impl<A> Semigroup for Swap<A>
    where
        A:Semigroup,
{
    #[inline]
    fn app(&self, rhs:&Swap<A>) -> Swap<A> {
        let &Swap(ref rhs) = rhs;
        let &Swap(ref lhs) = self;
        Swap(rhs.app(lhs))
    }
}
