// local imports
use semigroup::theory::{
    Semigroup,
};
use structure::{
    Swap,
};

impl<A> Semigroup for Swap<A>
    where
        A:Semigroup,
{
}
