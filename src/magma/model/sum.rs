// local imports
use magma::theory::{
    Magma,
};

impl<A,B> Magma for Result<A,B>
    where
        A:Clone,
        B:Clone,
        A:Magma,
        B:Magma,
{
    #[inline]
    fn op(&self, rhs:&Result<A,B>) -> Result<A,B> {
        match self {
            &Ok (_) => { (*self).clone() },
            &Err(_) => { (* rhs).clone() },
        }
    }
}
