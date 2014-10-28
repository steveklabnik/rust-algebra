// local imports
use monoid::theory::{
    Monoid,
};
use structure::{
    And,
};

impl Monoid for And
{
    #[inline]
    fn nil() -> And {
        And(true)
    }
}
