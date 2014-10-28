// local imports
use core::{
    Semigroup,
};

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(Ord)]
#[deriving(PartialEq)]
#[deriving(PartialOrd)]
#[deriving(Show)]
pub struct And(pub bool);

impl Semigroup for And
{
    #[inline]
    fn app(&self, rhs:&And) -> And {
        let And(lhs) = *self;
        let And(rhs) = *rhs;
        And(lhs && rhs)
    }
}
