// local imports
use semigroup::theory::{
    Semigroup,
};
use structure::{
    Add,
};

impl<A> Semigroup for Add<A>
    where
        A:Num,
{
}
