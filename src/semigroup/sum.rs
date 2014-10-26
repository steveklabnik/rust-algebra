// local imports
use core::{
    Semigroup,
};

impl<A,B> Semigroup for Result<A,B>
    where
        A:Clone,
        B:Clone,
        A:Semigroup,
        B:Semigroup,
{
    #[inline]
    fn op(&self, rhs:&Result<A,B>) -> Result<A,B> {
        match self {
            &Ok (_) => { (*self).clone() },
            &Err(_) => { (* rhs).clone() },
        }
    }
}
