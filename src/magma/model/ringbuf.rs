// external imports
use std::collections::{
    Deque,
    RingBuf,
};

// local imports
use magma::theory::{
    Magma,
};

impl<A> Magma for RingBuf<A>
    where
        A:Clone,
{
    #[inline]
    fn op(&self, rhs:&RingBuf<A>) -> RingBuf<A> {
        if self.len() > rhs.len() {
            let mut acc = self.clone();
            for x in rhs.iter() {
                acc.push(x.clone())
            }
            acc
        } else {
            let mut acc = rhs.clone();
            for x in self.iter().rev() {
                acc.push_front(x.clone())
            }
            acc
        }
    }
}
