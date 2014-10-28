// local imports
use semigroup::theory::{
    Semigroup,
};

impl<A,B> Semigroup for (A,B)
    where
        A:Semigroup,
        B:Semigroup,
{
}
