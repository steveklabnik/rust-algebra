// external imports
use std::collections::ringbuf::{
    RingBuf,
};

// local imports
use monoid::theory::{
    Monoid,
};

impl<A> Monoid for RingBuf<A>
    where
        A:Clone,
{
    #[inline]
    fn nil() -> RingBuf<A> {
        RingBuf::new()
    }
}
