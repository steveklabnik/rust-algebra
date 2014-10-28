// external imports
use std::collections::dlist::{
    DList,
};

// local imports
use semigroup::theory::{
    Semigroup,
};

impl<A> Semigroup for DList<A>
    where
        A:Clone,
{
}
