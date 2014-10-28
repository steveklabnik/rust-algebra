// local imports
use semigroup::theory::{
    Semigroup,
};

impl<A> Semigroup for Vec<A>
    where
        A:Clone,
{
}
