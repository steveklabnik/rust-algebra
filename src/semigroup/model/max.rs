// local imports
use semigroup::theory::{
    Semigroup,
};
use structure::{
    Max,
};

impl<A> Semigroup for Max<A>
    where
        A:Clone,
        A:Ord,
{
}
