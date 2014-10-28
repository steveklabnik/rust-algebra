// local imports
use semigroup::theory::{
    Semigroup,
};
use structure::{
    Fst,
};

impl<A> Semigroup for Fst<A>
    where
        A:Clone,
{
}
