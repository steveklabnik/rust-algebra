// local imports
use monoid::theory::{
    Monoid,
};
use semigroup::theory::{
    Semigroup,
};

impl<A> Monoid for Option<A>
    where
        A:Clone,
        A:Semigroup,
{
    #[inline]
    fn nil() -> Option<A> {
        None
    }
}
