// local imports
use monoid::theory::{
    Monoid,
};

impl<A,B> Monoid for (A,B)
    where
        A:Monoid,
        B:Monoid,
{
    #[inline]
    fn nil() -> (A,B) {
        (Monoid::nil(), Monoid::nil())
    }
}
