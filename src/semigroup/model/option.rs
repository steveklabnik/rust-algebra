// local imports
use semigroup::theory::{
    Semigroup,
};

impl<A> Semigroup for Option<A>
    where
        A:Clone,
        A:Semigroup,
{
}
