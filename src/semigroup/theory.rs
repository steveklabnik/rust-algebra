use magma::theory::{
    Magma,
};

pub trait Semigroup: Magma
{
    #[inline]
    fn app(&self, rhs:&Self) -> Self {
        self.op(rhs)
    }
}

pub trait SemigroupIterator<A>
    where
        A:Semigroup,
{
    fn cat_one(&mut self, base:A) -> A;
}

pub trait SemigroupReplicate: Semigroup
{
    fn rep_one(self, exp:uint) -> Self;
}

impl<A,F> SemigroupIterator<A> for F
    where
        A:Semigroup,
        F:Iterator<A>,
{
    #[inline]
    fn cat_one(&mut self, base:A) -> A {
        self.fold(base, |acc, x| acc.app(&x))
    }
}

impl<A> SemigroupReplicate for A
    where
        A:Clone,
        A:Semigroup,
{
    #[inline]
    fn rep_one(mut self, mut exp:uint) -> A {
        if exp == 0 { self }
        else {
            let mut acc = self.clone();
            while exp > 0 {
                if (exp & 1) == 1 {
                    acc = acc.app(&self);
                }
                self = self.app(&self);
                exp = exp >> 1;
            }
            acc
        }
    }
}
