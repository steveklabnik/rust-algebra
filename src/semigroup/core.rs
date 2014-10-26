// FIXME: revisit when associated-types are working
pub trait Semigroup
{
    fn op(&self, rhs:&Self) -> Self;
}

pub trait SemigroupIterator<A>
    where
        A:Semigroup,
{
    fn product(&mut self, base:A) -> A;
}

pub trait SemigroupPowNonZero: Semigroup
{
    fn pownz(self, exp:uint) -> Self;
}

impl<A,F> SemigroupIterator<A> for F
    where
        A:Semigroup,
        F:Iterator<A>,
{
    #[inline]
    fn product(&mut self, base:A) -> A {
        self.fold(base, |acc, x| acc.op(&x))
    }
}

impl<A> SemigroupPowNonZero for A
    where
        A:Clone,
        A:Semigroup,
{
    #[inline]
    fn pownz(mut self, mut exp:uint) -> A {
        if exp == 0 { self }
        else {
            let mut acc = self.clone();
            while exp > 0 {
                if (exp & 1) == 1 {
                    acc = acc.op(&self);
                }
                self = self.op(&self);
                exp = exp >> 1;
            }
            acc
        }
    }
}

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(PartialEq)]
#[deriving(Show)]
pub struct S<A:Semigroup>(pub A);

impl<A> Semigroup for S<A>
    where
        A:Semigroup,
{
    #[inline]
    fn op(&self, rhs:&S<A>) -> S<A> {
        let &S(ref lhs) = self;
        let &S(ref rhs) = rhs;
        S(lhs.op(rhs))
    }
}

impl<A> Mul<S<A>,S<A>> for S<A>
    where
        A:Semigroup,
{
    #[inline]
    fn mul(&self, rhs:&S<A>) -> S<A> {
        self.op(rhs)
    }
}
