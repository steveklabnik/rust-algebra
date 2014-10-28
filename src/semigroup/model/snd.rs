// local imports
use semigroup::theory::{
    Semigroup,
};
use structure::{
    Snd,
};

impl<A> Semigroup for Snd<A>
    where
        A:Clone,
{
}
