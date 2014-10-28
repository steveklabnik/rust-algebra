// local imports
use magma::theory::{
    Magma,
};

impl<A> Magma for Option<A>
    where
        A:Clone,
        A:Magma,
{
    #[inline]
    fn op(&self, rhs:&Option<A>) -> Option<A> {
        match self {
            &None          => { (*rhs).clone() },
            &Some(ref lhs) => {
                match rhs {
                    &None          => { (*self).clone() },
                    &Some(ref rhs) => {
                        Some(lhs.op(rhs))
                    },
                }
            },
        }
    }
}
