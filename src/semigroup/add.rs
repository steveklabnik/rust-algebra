// local imports
use core::{
    Semigroup,
};

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(PartialEq)]
#[deriving(Show)]
pub struct Add<A:Num>(pub A);

impl<A> Semigroup for Add<A>
    where
        A:Num,
{
    #[inline]
    fn op(&self, rhs:&Add<A>) -> Add<A> {
        let &Add(ref lhs) = self;
        let &Add(ref rhs) = rhs;
        Add(lhs.add(rhs))
    }
}
