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
pub struct Or(pub bool);

impl Semigroup for Or
{
    #[inline]
    fn app(&self, rhs:&Or) -> Or {
        let Or(lhs) = *self;
        let Or(rhs) = *rhs;
        Or(lhs || rhs)
    }
}
