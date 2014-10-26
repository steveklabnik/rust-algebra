// local imports
use core::{
    Semigroup,
};

impl Semigroup for ()
{
    #[inline]
    fn op(&self, _:&()) -> () {
        ()
    }
}
