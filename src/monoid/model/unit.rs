// local imports
use monoid::theory::{
    Monoid,
};

impl Monoid for ()
{
    #[inline]
    fn nil() -> () {
        ()
    }
}
