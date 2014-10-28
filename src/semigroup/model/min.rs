// local imports
use semigroup::theory::{
    Semigroup,
};
use structure::{
    Min,
};

impl<A> Semigroup for Min<A>
    where
        A:Clone,
        A:Ord,
{
}
