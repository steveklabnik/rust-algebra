// local imports
use core::{
    Semigroup,
};

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(PartialEq)]
#[deriving(Show)]
pub struct Fst<A>(pub A);

impl<A> Semigroup for Fst<A>
    where
        A:Clone,
{
    #[inline]
    fn op(&self, _:&Fst<A>) -> Fst<A> {
        (*self).clone()
    }
}
