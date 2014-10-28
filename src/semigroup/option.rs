// local imports
use core::{
    Semigroup,
};

impl<A> Semigroup for Option<A>
    where
        A:Clone,
        A:Semigroup,
{
    #[inline]
    fn app(&self, rhs:&Option<A>) -> Option<A> {
        match self {
            &None          => { (*rhs).clone() },
            &Some(ref lhs) => {
                match rhs {
                    &None          => { (*self).clone() },
                    &Some(ref rhs) => {
                        Some(lhs.app(rhs))
                    },
                }
            },
        }
    }
}
