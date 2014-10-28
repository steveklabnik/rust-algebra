// external imports
use semigroup::theory::{
    Semigroup,
};
use std::collections::{
    RingBuf,
};

// local imports

impl<A> Semigroup for RingBuf<A>
    where
        A:Clone,
{
}
