// local imports
use monoid::theory::{
    Monoid,
};
use structure::{
    Or,
};

impl Monoid for Or
{
    #[inline]
    fn nil() -> Or {
        Or(false)
    }
}
