// local imports
use core::{
    Semigroup,
};

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(PartialEq)]
#[deriving(Show)]
pub struct Snd<A>(pub A);

impl<A> Semigroup for Snd<A>
    where
        A:Clone,
{
    #[inline]
    fn app(&self, rhs:&Snd<A>) -> Snd<A> {
        (*rhs).clone()
    }
}
