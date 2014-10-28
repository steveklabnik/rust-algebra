// local imports
use magma::theory::{
    Magma,
};
use structure::{
    Snd,
};

impl<A> Magma for Snd<A>
    where
        A:Clone,
{
    #[inline]
    fn op(&self, rhs:&Snd<A>) -> Snd<A> {
        (*rhs).clone()
    }
}
