// local imports
use core::{
    Semigroup,
};

impl Semigroup for Ordering
{
    #[inline]
    fn app(&self, rhs:&Ordering) -> Ordering {
        match *self {
            Less    => { Less    },
            Equal   => { *rhs    },
            Greater => { Greater },
        }
    }
}
