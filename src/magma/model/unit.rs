// local imports
use magma::theory::{
    Magma,
};

impl Magma for ()
{
    #[inline]
    fn op(&self, _:&()) -> () {
        ()
    }
}
