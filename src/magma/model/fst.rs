// local imports
use magma::theory::{
    Magma,
};
use structure::{
    Fst,
};

impl<A> Magma for Fst<A>
    where
        A:Clone,
{
    #[inline]
    fn op(&self, _:&Fst<A>) -> Fst<A> {
        (*self).clone()
    }
}
