// local imports
use monoid::theory::{
    Monoid,
};

impl Monoid for String
{
    #[inline]
    fn nil() -> String {
        String::new()
    }
}
