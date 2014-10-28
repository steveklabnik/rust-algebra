// external imports
use std::collections::dlist::{
    DList,
};

// local imports
use monoid::theory::{
    Monoid,
};

impl<A> Monoid for DList<A>
    where
        A:Clone,
{
    #[inline]
    fn nil() -> DList<A> {
        DList::new()
    }
}
