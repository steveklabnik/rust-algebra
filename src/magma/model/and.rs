// local imports
use magma::theory::{
    Magma,
};
use structure::{
    And,
};

impl Magma for And
{
    #[inline]
    fn op(&self, rhs:&And) -> And {
        let And(lhs) = *self;
        let And(rhs) = *rhs;
        And(lhs && rhs)
    }
}
