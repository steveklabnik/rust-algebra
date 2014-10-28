// local imports
use magma::theory::{
    Magma,
};

impl Magma for Ordering
{
    #[inline]
    fn op(&self, rhs:&Ordering) -> Ordering {
        match *self {
            Less    => { Less    },
            Equal   => { *rhs    },
            Greater => { Greater },
        }
    }
}
