use semigroup::theory::{
    Semigroup,
};

pub trait Monoid: Semigroup
{
    fn nil() -> Self;
}

pub trait MonoidIterator<A>
    where
        A:Monoid,
{
    fn cat(&mut self) -> A;
}

pub trait MonoidReplicate: Monoid
{
    fn rep(self, exp:uint) -> Self;
}

impl<A,F> MonoidIterator<A> for F
    where
        A:Monoid,
        F:Iterator<A>,
{
    #[inline]
    fn cat(&mut self) -> A {
        self.fold(Monoid::nil(), |acc:A, x| acc.app(&x))
    }
}

impl<A> MonoidReplicate for A
    where
        A:Clone,
        A:Monoid,
{
    #[inline]
    fn rep(mut self, mut exp:uint) -> A {
        if exp == 1 { self }
        else {
            let mut acc: A = Monoid::nil();
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
