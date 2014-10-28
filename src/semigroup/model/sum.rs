// local imports
use semigroup::theory::{
    Semigroup,
};

impl<A,B> Semigroup for Result<A,B>
    where
        A:Clone,
        B:Clone,
        A:Semigroup,
        B:Semigroup,
{
}
