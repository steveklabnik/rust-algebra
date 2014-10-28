// external imports
use std;

// local imports
use monoid::theory::{
    Monoid,
};
use structure::{
    Min,
};

impl<A> Monoid for Min<A>
    where
        A:std::num::Bounded,
        A:Clone,
        A:Ord,
{
    #[inline]
    fn nil() -> Min<A> {
        Min(std::num::Bounded::max_value())
    }
}
