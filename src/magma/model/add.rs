// local imports
use magma::theory::{
    Magma,
};
use structure::{
    Add,
};

impl<A> Magma for Add<A>
    where
        A:Num,
{
    #[inline]
    fn op(&self, rhs:&Add<A>) -> Add<A> {
        let &Add(ref lhs) = self;
        let &Add(ref rhs) = rhs;
        Add(lhs.add(rhs))
    }
}
