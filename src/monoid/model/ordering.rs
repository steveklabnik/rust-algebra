// local imports
use monoid::theory::{
    Monoid,
};

impl Monoid for Ordering
{
    fn nil() -> Ordering {
        Equal
    }
}
