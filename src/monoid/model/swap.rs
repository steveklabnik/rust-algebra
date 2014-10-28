// local imports
use monoid::theory::{
    Monoid,
};
use structure::{
    Swap,
};

impl<A> Monoid for Swap<A>
    where
        A:Monoid,
{
    #[inline]
    fn nil() -> Swap<A> {
        Swap(Monoid::nil())
    }
}
