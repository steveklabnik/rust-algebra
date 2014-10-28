// local imports
use monoid::theory::{
    Monoid,
};

impl<A> Monoid for Vec<A>
    where
        A:Clone,
{
    #[inline]
    fn nil() -> Vec<A> {
        Vec::new()
    }
}
