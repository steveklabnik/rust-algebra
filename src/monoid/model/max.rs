// external imports
use std;

// local imports
use monoid::theory::{
    Monoid,
};
use structure::{
    Max,
};

impl<A> Monoid for Max<A>
    where
        A:std::num::Bounded,
        A:Clone,
        A:Ord,
{
    #[inline]
    fn nil() -> Max<A> {
        Max(std::num::Bounded::min_value())
    }
}
