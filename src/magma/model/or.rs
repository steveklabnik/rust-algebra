// local imports
use magma::theory::{
    Magma,
};
use structure::{
    Or,
};

impl Magma for Or
{
    #[inline]
    fn op(&self, rhs:&Or) -> Or {
        let Or(lhs) = *self;
        let Or(rhs) = *rhs;
        Or(lhs || rhs)
    }
}
