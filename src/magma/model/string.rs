// local imports
use magma::theory::{
    Magma,
};

impl Magma for String
{
    #[inline]
    fn op(&self, rhs:&String) -> String {
        let mut res: String = self.clone();
        res.push_str(rhs.as_slice());
        res
    }
}
