// local imports
use core::{
    Semigroup,
};

impl Semigroup for String
{
    #[inline]
    fn op(&self, rhs:&String) -> String {
        let mut res: String = self.clone();
        res.push_str(rhs.as_slice());
        res
    }
}
