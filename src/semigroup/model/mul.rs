// local imports
use semigroup::theory::{
    Semigroup,
};
use structure::{
    Mul,
};

impl<A> Semigroup for Mul<A>
    where
        A:Num,
{
}
