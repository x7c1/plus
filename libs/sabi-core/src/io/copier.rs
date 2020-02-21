use std::io;
use std::io::{Read, Write};

pub trait Copier {
    //    fn copy_to<W: Write + CanLoan>(self, out: &mut W) -> io::Result<u64>;
}

impl<R> Copier for R
where
    R: Read,
{
    //    fn copy_to<W: Write + CanLoan>(mut self, out: &mut W) -> io::Result<u64> {
    //        out.loan(|mut out| io::copy(&mut self, &mut out))
    //        //        let total = io::copy(&mut self, out)?;
    //        //        Ok(total)
    //    }
}

pub trait CanLoan {
    //    fn loan<F, Y>(&mut self, mut f: F) -> Y
    //    where
    //        F: FnMut(&mut Self) -> Y;

    fn copy_from<A: Read>(&mut self, a: A) -> io::Result<u64>;
}
