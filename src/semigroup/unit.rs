// local imports
use core::{
    Semigroup,
};

impl Semigroup for ()
{
    #[inline]
    fn app(&self, _:&()) -> () {
        ()
    }
}
