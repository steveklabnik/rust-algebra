// external imports
use std::collections::{
    RingBuf,
};

// local imports
use semigroup::theory::{
    Semigroup,
};

impl<A> Semigroup for RingBuf<A>
    where
        A:Clone,
{
}
